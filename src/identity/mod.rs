pub mod models;

use crate::identity::models::response::UserRiskProfileResponse;
use crate::models::error::ServiceError;
use crate::util::{generate_headers, parse_empty_response, parse_response};
use jsonwebtoken::dangerous::insecure_decode;
use log::{debug, info, warn};
use models::{
    Credentials, CredentialsResponse, CustomerRequest, CustomerResponse, NewDocumentRequest,
    NewDocumentResponse, PaginatedServicesResponse, PatchScopesRequest, ServiceResponse,
    ServiceToken, TokenCheckRequest, TokenCheckResponse, TokenClaims, UpdateProfileRequest,
    UpdateServiceProfileRequest, UserKycStatusResponse,
};
use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde_json::json;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Clone)]
pub struct IdentityServiceClient {
    pub credentials: Credentials,
    pub token: Option<ServiceToken>,
    pub host: String,
    pub client: Client,
    pub max_retries: i8,
    pub client_identifier: Option<String>,
}

pub fn extract_token_claims(token: &str) -> Result<TokenClaims, ServiceError> {
    insecure_decode::<TokenClaims>(token)
        .map(|td| td.claims)
        .map_err(|e| ServiceError {
            error: Some(format!("Could not extract token claim {e}")),
            message: None,
            description: None,
            status: Default::default(),
        })
}

impl IdentityServiceClient {
    pub fn new(max_retries: i8) -> Self {
        let identity_service_host =
            env::var("IDENTITY_SERVICE_HOST").expect("IDENTITY_SERVICE_HOST must be set");
        let credentials = Credentials {
            access_key: env::var("IDENTITY_ACCESS_KEY").expect("IDENTITY_ACCESS_KEY must be set"),
            secret_key: env::var("IDENTITY_SECRET_KEY").expect("IDENTITY_SECRET_KEY must be set"),
        };

        Self {
            credentials,
            host: identity_service_host,
            token: None,
            client: Client::new(),
            max_retries,
            client_identifier: None,
        }
    }

    pub fn get_token(&self) -> Option<ServiceToken> {
        self.token.clone()
    }

    fn client(&self) -> &Client {
        &self.client
    }

    fn credentials(&self) -> &Credentials {
        &self.credentials
    }

    pub fn set_token(&mut self, token: Option<ServiceToken>) {
        self.token = token;
    }

    fn max_retries(&self) -> i8 {
        self.max_retries
    }

    fn host(&self) -> String {
        self.host.clone()
    }

    fn token_is_valid(&self) -> bool {
        if let Some(service_token) = &self.get_token() {
            match extract_token_claims(service_token.token.as_str()) {
                Ok(claims) => {
                    let current_time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as usize;

                    if claims.exp <= current_time {
                        warn!(
                            "Token has expired. Expiration: {}, Current: {}",
                            claims.exp, current_time
                        );
                        false
                    } else {
                        true
                    }
                }
                Err(e) => {
                    warn!("Token is invalid {e:?}");
                    false
                }
            }
        } else {
            false
        }
    }

    async fn acquire_token(&self) -> Result<ServiceToken, ServiceError> {
        debug!("Authenticating against {}", self.host());
        let response = self
            .client()
            .post(format!("{}/authenticate", self.host()))
            .headers(generate_headers(None, None))
            .json(&self.credentials())
            .send()
            .await;
        parse_response::<ServiceToken>(response).await
    }

    pub async fn attempt_token_acquisition(&mut self) -> Option<ServiceToken> {
        if self.token_is_valid() {
            return self.token.clone();
        }
        match self.acquire_token().await {
            Ok(token_response) => {
                info!("Token acquired from IDENTITY service!");
                self.set_token(Some(token_response));
                self.get_token()
            }
            Err(err) => {
                warn!("Failed to acquire token: [{err}]");
                for attempt in 0..self.max_retries() {
                    info!("Attempting to acquire token {attempt} again...");
                    match self.acquire_token().await {
                        Ok(token_response) => {
                            info!(
                                "Successfully acquired token from IDENTITY SERVICE on attempt {attempt}"
                            );
                            self.set_token(Some(token_response));
                        }
                        Err(err) => {
                            warn!("Failed to acquire token [{err}], retrying attempt {attempt}");
                            sleep(Duration::from_secs(3)).await;
                            self.set_token(None);
                        }
                    }
                }
                self.get_token()
            }
        }
    }

    pub async fn check_scope(
        &mut self,
        requester_token: &str,
        scope: &str,
    ) -> Result<TokenCheckResponse, ServiceError> {
        let service_token = self.attempt_token_acquisition().await;
        let response = self
            .client()
            .post(format!("{}/authorise/scope", self.host()))
            .headers(generate_headers(service_token, None))
            .json(&TokenCheckRequest {
                token: requester_token.to_string(),
                scope: Some(scope.to_string()),
                subject: None,
            })
            .send()
            .await;

        parse_response::<TokenCheckResponse>(response).await
    }

    pub async fn check_subject(
        &mut self,
        subject_token: &str,
        subject: &str,
    ) -> Result<TokenCheckResponse, ServiceError> {
        let service_token = self.attempt_token_acquisition().await;
        let response = self
            .client()
            .post(format!("{}/authorise/subject", self.host()))
            .headers(generate_headers(service_token, None))
            .json(&TokenCheckRequest {
                token: subject_token.to_string(),
                subject: Some(subject.to_string()),
                scope: None,
            })
            .send()
            .await;

        parse_response::<TokenCheckResponse>(response).await
    }

    /** Get a more detailed KYC status with the user profile
     */
    pub async fn get_kyc_status_with_profile(
        &mut self,
        id: &str,
        user_token: Option<String>,
    ) -> Result<UserKycStatusResponse, ServiceError> {
        self.attempt_token_acquisition().await;
        let service_token = self.attempt_token_acquisition().await;
        let headers = if let Some(u_token) = user_token {
            let mut h = generate_headers(service_token, None);
            h.insert(AUTHORIZATION, format!("Bearer {u_token}").parse().unwrap());
            h
        } else {
            generate_headers(service_token, None)
        };

        let url = format!("{}/kyc/profile/{}", self.host, id);
        let response = self.client.get(url).headers(headers).send().await;

        parse_response::<UserKycStatusResponse>(response).await
    }

    /**
     * Get a user's profile information. This function will usually be invoked by a service as this uses
     * the implicit service token to get the user's profile information. If a user token is provided, it will be used instead.
     */
    pub async fn get_profile_by_id(
        &mut self,
        id: &str,
        user_token: Option<String>,
    ) -> Result<CustomerResponse, ServiceError> {
        let service_token = self.attempt_token_acquisition().await;
        let headers = if let Some(u_token) = user_token {
            let mut h = generate_headers(service_token, None);
            h.insert(AUTHORIZATION, format!("Bearer {u_token}").parse().unwrap());
            h
        } else {
            generate_headers(service_token, None)
        };

        let url = format!("{}/user/profile/{}", self.host, id);
        let response = self.client.get(url).headers(headers).send().await;

        parse_response::<CustomerResponse>(response).await
    }

    /**
     * Given an email address return a user profile
     */
    pub async fn get_profile_by_email(
        &mut self,
        email: &str,
        user_token: Option<String>,
    ) -> Result<CustomerResponse, ServiceError> {
        let service_token = self.attempt_token_acquisition().await;
        let headers = if let Some(u_token) = user_token {
            let mut h = generate_headers(service_token, None);
            h.insert(AUTHORIZATION, format!("Bearer {u_token}").parse().unwrap());
            h
        } else {
            generate_headers(service_token, None)
        };

        let url = format!("{}/user/profile/email/{}", self.host, email);
        let response = self.client.get(url).headers(headers).send().await;

        parse_response::<CustomerResponse>(response).await
    }

    /**
    This function allows a service to get a user's profile information using a token. This token would be the user token. It can be invoked by a service but
    the service token will NOT be used for this operation. Instead a provided user token will be used to get the user's profile information.
    */
    pub async fn get_profile_with_token(
        &self,
        token: &str,
    ) -> Result<CustomerResponse, ServiceError> {
        let mut h = generate_headers(None, None);
        h.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
        let url = format!("{}/user/profile", self.host);
        let response = self.client.get(url).headers(h).send().await;
        parse_response::<CustomerResponse>(response).await
    }

    pub async fn new_profile(
        &mut self,
        customer: CustomerRequest,
    ) -> Result<CustomerResponse, ServiceError> {
        self.attempt_token_acquisition().await;
        let response = self
            .client
            .post(format!("{}/user/profile/new", self.host))
            .body(json!(customer).to_string())
            .headers(generate_headers(
                self.get_token(),
                self.client_identifier.clone(),
            ))
            .send()
            .await;

        parse_response::<CustomerResponse>(response).await
    }

    pub async fn update_profile(
        &mut self,
        customer: UpdateProfileRequest,
        id: Option<&str>,
    ) -> Result<CustomerResponse, ServiceError> {
        self.attempt_token_acquisition().await;
        let url = match id {
            Some(id) => format!("{}/user/profile/update/{}", self.host, id),
            None => format!("{}/user/profile/update", self.host),
        };
        let response = self
            .client
            .patch(url)
            .body(json!(customer).to_string())
            .headers(generate_headers(
                self.get_token(),
                self.client_identifier.clone(),
            ))
            .send()
            .await;

        parse_response::<CustomerResponse>(response).await
    }

    pub async fn new_document(
        &mut self,
        new_document_request: NewDocumentRequest,
    ) -> Result<NewDocumentResponse, ServiceError> {
        self.attempt_token_acquisition().await;
        let payload = json!(new_document_request).to_string();
        debug!("PAYLOAD FOR SUBMITTING DOCUMENT: {payload}");
        let response = self
            .client
            .put(format!("{}/kyc/documents", self.host))
            .body(payload)
            .headers(generate_headers(
                self.get_token(),
                self.client_identifier.clone(),
            ))
            .send()
            .await;

        parse_response::<NewDocumentResponse>(response).await
    }

    pub async fn get_risk_score(
        &mut self,
        id: &str,
        token: &str,
    ) -> Result<UserRiskProfileResponse, ServiceError> {
        let mut h = generate_headers(None, None);
        h.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
        let url = format!("{}/user/profile/{id}/risk_profile", self.host);
        let response = self.client.get(url).headers(h).send().await;
        parse_response::<UserRiskProfileResponse>(response).await
    }

    /// List service profiles with optional status filter (`active`, `suspended`, `all`)
    /// and pagination. Requires `service:admin` scope on the caller's token.
    pub async fn list_services(
        &mut self,
        status: Option<&str>,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<PaginatedServicesResponse, ServiceError> {
        let service_token = self.attempt_token_acquisition().await;
        let mut url = format!("{}/service/list", self.host);
        let mut params: Vec<(String, String)> = Vec::new();
        if let Some(s) = status {
            params.push(("status".to_string(), s.to_string()));
        }
        if let Some(p) = page {
            params.push(("page".to_string(), p.to_string()));
        }
        if let Some(ps) = page_size {
            params.push(("page_size".to_string(), ps.to_string()));
        }
        if !params.is_empty() {
            let qs = params
                .into_iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("&");
            url = format!("{url}?{qs}");
        }
        let response = self
            .client
            .get(url)
            .headers(generate_headers(
                service_token,
                self.client_identifier.clone(),
            ))
            .send()
            .await;
        parse_response::<PaginatedServicesResponse>(response).await
    }

    /// Update a service profile's name and/or email. Requires `service:admin` scope.
    pub async fn update_service(
        &mut self,
        id: &str,
        req: UpdateServiceProfileRequest,
    ) -> Result<ServiceResponse, ServiceError> {
        let service_token = self.attempt_token_acquisition().await;
        let response = self
            .client
            .put(format!("{}/service/{id}", self.host))
            .json(&req)
            .headers(generate_headers(
                service_token,
                self.client_identifier.clone(),
            ))
            .send()
            .await;
        parse_response::<ServiceResponse>(response).await
    }

    /// Get the credential metadata for a service profile (does not include the secret key).
    /// Requires `service:admin` scope.
    pub async fn get_service_credentials(
        &mut self,
        id: &str,
    ) -> Result<CredentialsResponse, ServiceError> {
        let service_token = self.attempt_token_acquisition().await;
        let response = self
            .client
            .get(format!("{}/service/{id}/credentials", self.host))
            .headers(generate_headers(
                service_token,
                self.client_identifier.clone(),
            ))
            .send()
            .await;
        parse_response::<CredentialsResponse>(response).await
    }

    /// Add and/or remove scopes on a service's credentials. Requires `service:admin` scope.
    pub async fn patch_service_credentials_scopes(
        &mut self,
        id: &str,
        req: PatchScopesRequest,
    ) -> Result<CredentialsResponse, ServiceError> {
        let service_token = self.attempt_token_acquisition().await;
        let response = self
            .client
            .patch(format!("{}/service/{id}/credentials/scopes", self.host))
            .json(&req)
            .headers(generate_headers(
                service_token,
                self.client_identifier.clone(),
            ))
            .send()
            .await;
        parse_response::<CredentialsResponse>(response).await
    }

    /// Rotate a service's credentials. The plaintext secret is returned only in this response.
    /// Requires `service:admin` scope.
    pub async fn rotate_service_credentials(
        &mut self,
        id: &str,
    ) -> Result<ServiceResponse, ServiceError> {
        let service_token = self.attempt_token_acquisition().await;
        let response = self
            .client
            .post(format!("{}/service/{id}/credentials/rotate", self.host))
            .headers(generate_headers(
                service_token,
                self.client_identifier.clone(),
            ))
            .send()
            .await;
        parse_response::<ServiceResponse>(response).await
    }

    /// Suspend a service's credentials. Requires `service:admin` scope.
    pub async fn suspend_service_credentials(
        &mut self,
        id: &str,
    ) -> Result<CredentialsResponse, ServiceError> {
        let service_token = self.attempt_token_acquisition().await;
        let response = self
            .client
            .post(format!("{}/service/{id}/credentials/suspend", self.host))
            .headers(generate_headers(
                service_token,
                self.client_identifier.clone(),
            ))
            .send()
            .await;
        parse_response::<CredentialsResponse>(response).await
    }

    /// Unsuspend a service's credentials. Requires `service:admin` scope.
    pub async fn unsuspend_service_credentials(
        &mut self,
        id: &str,
    ) -> Result<CredentialsResponse, ServiceError> {
        let service_token = self.attempt_token_acquisition().await;
        let response = self
            .client
            .post(format!("{}/service/{id}/credentials/unsuspend", self.host))
            .headers(generate_headers(
                service_token,
                self.client_identifier.clone(),
            ))
            .send()
            .await;
        parse_response::<CredentialsResponse>(response).await
    }

    /// Revoke all active access tokens for a service credential. The credential's
    /// `tokens_invalid_before` is stamped to the current time and any outstanding
    /// refresh-token row is deleted. Requires `token:admin` scope.
    pub async fn revoke_service_sessions(&mut self, id: &str) -> Result<(), ServiceError> {
        let service_token = self.attempt_token_acquisition().await;
        let response = self
            .client
            .post(format!(
                "{}/service/{id}/credentials/revoke-sessions",
                self.host
            ))
            .headers(generate_headers(
                service_token,
                self.client_identifier.clone(),
            ))
            .send()
            .await;
        parse_empty_response(response).await
    }

    /// Revoke all active access tokens for a user credential. The credential's
    /// `tokens_invalid_before` is stamped to the current time and any outstanding
    /// refresh-token row is deleted. Requires `token:admin` scope.
    pub async fn revoke_user_sessions(&mut self, id: &str) -> Result<(), ServiceError> {
        let service_token = self.attempt_token_acquisition().await;
        let response = self
            .client
            .post(format!("{}/user/sessions/{id}/revoke", self.host))
            .headers(generate_headers(
                service_token,
                self.client_identifier.clone(),
            ))
            .send()
            .await;
        parse_empty_response(response).await
    }
}
