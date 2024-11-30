use jsonwebtoken::{decode, DecodingKey, Validation};
use log::{debug, info, warn};
use reqwest::Client;
use std::{env, time::Duration};
use tokio::time::sleep;

use crate::{
    models::{
        request::{Credentials, TokenCheckRequest},
        response::{
            auth::{ServiceToken, TokenCheckResponse, TokenClaims},
            ServiceError,
        },
    },
    util::{generate_headers, parse_response},
};

pub trait Authentication {
    fn token(&self) -> Option<ServiceToken>;
    fn set_token(&mut self, token: Option<ServiceToken>);
    fn client(&self) -> Client;
    fn credentials(&self) -> Credentials;
    fn host(&self) -> String {
        env::var("IDENTITY_SERVICE_HOST").expect("IDENTITY_SERVICE_HOST must be set")
    }
    fn max_retries(&self) -> i8;

    fn token_is_valid(&self) -> bool {
        if let Some(service_token) = &self.token() {
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
    #[allow(async_fn_in_trait)]
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

    #[allow(async_fn_in_trait)]
    async fn attempt_token_acquisition(&mut self) {
        if self.token_is_valid() {
            return;
        }
        match self.acquire_token().await {
            Ok(token_response) => {
                info!("Token acquired from IDENTITY service!");
                self.set_token(Some(token_response));
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
                            self.set_token(Some(token_response))
                        }
                        Err(err) => {
                            info!(
                                "Failed to acquire token [{}], retrying attempt {}",
                                err, attempt
                            );
                            sleep(Duration::from_secs(3)).await;
                            self.set_token(None)
                        }
                    }
                }
            }
        }
    }

    #[allow(async_fn_in_trait)]
    async fn check_scope(
        &mut self,
        requester_token: &str,
        scope: &str,
    ) -> Result<TokenCheckResponse, ServiceError> {
        self.attempt_token_acquisition().await;
        let response = self
            .client()
            .post(format!("{}/authorise/scope", self.host()))
            .headers(generate_headers(None, None))
            .json(&TokenCheckRequest {
                token: requester_token.to_string(),
                scope: Some(scope.to_string()),
                subject: None,
            })
            .send()
            .await;

        parse_response::<TokenCheckResponse>(response).await
    }

    #[allow(async_fn_in_trait)]
    async fn check_subject(
        &mut self,
        subject_token: &str,
        subject: &str,
    ) -> Result<TokenCheckResponse, ServiceError> {
        self.attempt_token_acquisition().await;
        let response = self
            .client()
            .post(format!("{}/authorise/subject", self.host()))
            .headers(generate_headers(None, None))
            .json(&TokenCheckRequest {
                token: subject_token.to_string(),
                subject: Some(subject.to_string()),
                scope: None,
            })
            .send()
            .await;

        parse_response::<TokenCheckResponse>(response).await
    }
}
