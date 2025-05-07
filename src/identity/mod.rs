use crate::models::request::identity::{
    Credentials, CustomerRequest, NewDocumentRequest, TokenCheckRequest, UpdateProfileRequest,
};
use crate::models::response::auth::{ServiceToken, TokenCheckResponse, TokenClaims};
use crate::models::response::identity::{
    CustomerResponse, NewDocumentResponse, UserKycStatusResponse,
};
use crate::models::response::ServiceError;
use crate::util::{generate_headers, parse_response};
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::{debug, info, warn};
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
    pub client: reqwest::Client,
    pub max_retries: i8,
    pub client_identifier: Option<String>,
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
            client: reqwest::Client::new(),
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

    fn set_token(&mut self, token: Option<ServiceToken>) {
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
            let key = DecodingKey::from_secret(&[]);
            let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
            validation.insecure_disable_signature_validation();
            validation.set_audience(&["Service"]);
            validation.validate_exp = true;

            match decode::<TokenClaims>(service_token.token.as_str(), &key, &validation) {
                Ok(_) => true,
                Err(e) => {
                    warn!("Token is invalid {:?}", e);
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
                warn!("Failed to acquire token: [{}]", err);
                for attempt in 0..self.max_retries() {
                    info!("Attempting to acquire token {} again...", attempt);
                    match self.acquire_token().await {
                        Ok(token_response) => {
                            info!(
                                "Successfully acquired token from IDENTITY SERVICE on attempt {}",
                                attempt
                            );
                            self.set_token(Some(token_response));
                        }
                        Err(err) => {
                            warn!(
                                "Failed to acquire token [{}], retrying attempt {}",
                                err, attempt
                            );
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
            h.insert(
                AUTHORIZATION,
                format!("Bearer {}", u_token).parse().unwrap(),
            );
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
            h.insert(
                AUTHORIZATION,
                format!("Bearer {}", u_token).parse().unwrap(),
            );
            h
        } else {
            generate_headers(service_token, None)
        };

        let url = format!("{}/user/profile/{}", self.host, id);
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
        h.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
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
    ) -> Result<CustomerResponse, ServiceError> {
        self.attempt_token_acquisition().await;
        let response = self
            .client
            .patch(format!("{}/user/profile/update", self.host))
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
        debug!("PAYLOAD FOR SUBMITTING DOCUMENT: {}", payload);
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
}
