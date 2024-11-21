use crate::models::request::{Credentials, RefreshToken, TokenCheckRequest};
use crate::models::response::{MykoboStatusCode, UserKycStatusResponse, WalletProfile};
use crate::models::response::{ServiceError, ServiceToken, TokenCheckResponse, TokenClaims};
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::{info, warn};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, USER_AGENT},
    Response,
};
use serde::de::DeserializeOwned;
use std::{env, time};
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
        let access_key = env::var("IDENTITY_ACCESS_KEY").expect("IDENTITY_ACCESS_KEY must be set");
        let secret_key = env::var("IDENTITY_SECRET_KEY").expect("IDENTITY_SECRET_KEY must be set");
        let identity_service_host =
            env::var("IDENTITY_SERVICE_HOST").expect("IDENTITY_SERVICE_HOST must be set");
        let credentials = Credentials {
            access_key,
            secret_key,
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

    fn token_is_valid(&self) -> bool {
        if let Some(service_token) = &self.token {
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
        let response = self
            .client
            .post(format!("{}/authenticate", self.host))
            .headers(self.generate_headers(false))
            .json(&self.credentials)
            .send()
            .await?;
        self.parse_response::<ServiceToken>(response).await
    }

    fn generate_headers(&self, with_token: bool) -> HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        if with_token {
            if let Some(token) = &self.token {
                headers.insert(
                    AUTHORIZATION,
                    format!("Bearer {}", token.token).parse().unwrap(),
                );
            }
        }
        if let Some(client_id) = &self.client_identifier {
            headers.insert(USER_AGENT, client_id.parse().unwrap());
        };

        headers
    }

    pub async fn get_token(&mut self) -> ServiceToken {
        self.attempt_token_acquisition().await;
        self.token.clone().unwrap()
    }

    async fn attempt_token_acquisition(&mut self) {
        if self.token_is_valid() {
            return;
        }
        match self.acquire_token().await {
            Ok(token_response) => {
                info!("Token acquired from IDENTITY service!");
                self.token = Some(token_response);
            }
            Err(err) => {
                warn!("Failed to acquire token: [{}]", err);
                for attempt in 0..self.max_retries {
                    info!("Attempting to acquire token {} again...", attempt);
                    self.token = match self.acquire_token().await {
                        Ok(token_response) => {
                            info!(
                                "Successfully acquired token from IDENTITY SERVICE on attempt {}",
                                attempt
                            );
                            Some(token_response)
                        }
                        Err(err) => {
                            info!(
                                "Failed to acquire token [{}], retrying attempt {}",
                                err, attempt
                            );
                            sleep(time::Duration::from_secs(3)).await;
                            None
                        }
                    }
                }
            }
        }
    }

    async fn parse_response<T: DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<T, ServiceError> {
        if response.status().is_success() {
            Ok(response.json::<T>().await?)
        } else {
            let status = response.status();
            let service_error = match response.text().await {
                Ok(text) => {
                    serde_json::from_str::<ServiceError>(text.as_str()).unwrap_or(ServiceError {
                        error: Some(text.to_string()),
                        message: Some(text),
                        status: MykoboStatusCode::from(status),
                    })
                }
                Err(e) => ServiceError {
                    error: Some(format!("{:?}", e).to_string()),
                    message: Some(format!("{:?}", e).to_string()),
                    status: e
                        .status()
                        .map(MykoboStatusCode::from)
                        .unwrap_or(MykoboStatusCode::DependencyFailed),
                },
            };
            Err(service_error)
        }
    }

    pub async fn refresh_token(&mut self) {
        if let Some(service_token) = &self.token {
            let response = self
                .client
                .post(format!("{}/authenticate/refresh", self.host))
                .headers(self.generate_headers(false))
                .json(&RefreshToken {
                    refresh_token: String::from(service_token.refresh_token.as_str()),
                })
                .send()
                .await;

            match response {
                Ok(resp) => match self.parse_response::<ServiceToken>(resp).await {
                    Ok(service_token) => {
                        self.token = Some(service_token);
                    }
                    Err(e) => {
                        warn!("Failed to refresh token {:?}", e);
                        self.token = None;
                    }
                },
                Err(e) => {
                    warn!("Failed to refresh token {:?}", e);
                    self.token = None;
                }
            }
        }
    }

    pub async fn check_scope(
        &mut self,
        requester_token: &str,
        scope: &str,
    ) -> Result<TokenCheckResponse, ServiceError> {
        self.attempt_token_acquisition().await;
        let response = self
            .client
            .post(format!("{}/authorise/scope", self.host))
            .headers(self.generate_headers(true))
            .json(&TokenCheckRequest {
                token: requester_token.to_string(),
                scope: Some(scope.to_string()),
                subject: None,
            })
            .send()
            .await?;

        self.parse_response::<TokenCheckResponse>(response).await
    }

    pub async fn check_subject(
        &mut self,
        subject_token: &str,
        subject: &str,
    ) -> Result<TokenCheckResponse, ServiceError> {
        self.attempt_token_acquisition().await;
        let response = self
            .client
            .post(format!("{}/authorise/subject", self.host))
            .headers(self.generate_headers(true))
            .json(&TokenCheckRequest {
                token: subject_token.to_string(),
                subject: Some(subject.to_string()),
                scope: None,
            })
            .send()
            .await?;

        self.parse_response::<TokenCheckResponse>(response).await
    }

    pub async fn get_profile(&mut self, id: String) -> Result<UserKycStatusResponse, ServiceError> {
        self.attempt_token_acquisition().await;

        let response = self
            .client
            .get(format!("{}/kyc/profile/{}", self.host, id))
            .headers(self.generate_headers(true))
            .send()
            .await?;

        self.parse_response::<UserKycStatusResponse>(response).await
    }

    pub async fn get_customer(
        &mut self,
        account_id: &str,
        memo: Option<&str>,
    ) -> Result<UserKycStatusResponse, ServiceError> {
        self.attempt_token_acquisition().await;

        let wallet_host = env::var("WALLET_HOST").expect("WALLET_HOST must be set");
        let wallet_url = match memo {
            Some(m) => format!("{}/user/wallet/{}?memo={}", wallet_host, account_id, m),
            None => format!("{}/user/wallet/{}", wallet_host, account_id),
        };

        let wallet_response = self
            .client
            .get(wallet_url)
            .headers(self.generate_headers(true))
            .send()
            .await;

        match wallet_response {
            Ok(wallet_profile) => {
                let parsed_wallet_profile =
                    self.parse_response::<WalletProfile>(wallet_profile).await;

                match parsed_wallet_profile {
                    Ok(w_profile) => self.get_profile(w_profile.profile_id).await,
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e.into()),
        }
    }
}
