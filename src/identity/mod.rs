use crate::auth::Authentication;
use crate::models::request::{Credentials, CustomerRequest};
use crate::models::response::auth::ServiceToken;
use crate::models::response::identity::{CustomerResponse, UserKycStatusResponse};
use crate::models::response::ServiceError;
use crate::util::{generate_headers, parse_response};
use log::debug;
use reqwest::Client;
use serde_json::json;
use std::env;

#[derive(Clone)]
pub struct IdentityServiceClient {
    pub credentials: Credentials,
    pub token: Option<ServiceToken>,
    pub host: String,
    pub client: reqwest::Client,
    pub max_retries: i8,
    pub client_identifier: Option<String>,
    pub wallet_host: String,
}

impl Authentication for IdentityServiceClient {
    fn token(&self) -> Option<ServiceToken> {
        self.token.clone()
    }

    fn client(&self) -> Client {
        self.client.clone()
    }

    fn credentials(&self) -> Credentials {
        self.credentials.clone()
    }

    fn set_token(&mut self, token: Option<ServiceToken>) {
        self.token = token;
    }

    fn max_retries(&self) -> i8 {
        self.max_retries
    }
}

impl IdentityServiceClient {
    pub fn new(max_retries: i8) -> Self {
        let access_key = env::var("IDENTITY_ACCESS_KEY").expect("IDENTITY_ACCESS_KEY must be set");
        let secret_key = env::var("IDENTITY_SECRET_KEY").expect("IDENTITY_SECRET_KEY must be set");
        let identity_service_host =
            env::var("IDENTITY_SERVICE_HOST").expect("IDENTITY_SERVICE_HOST must be set");
        let wallet_host = env::var("WALLET_HOST").expect("WALLET_HOST must be set");
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
            wallet_host,
        }
    }

    pub async fn get_profile(&mut self, id: &str) -> Result<UserKycStatusResponse, ServiceError> {
        self.attempt_token_acquisition().await;
        debug!("Getting profile with [{}] at {}", id, self.host);
        let response = self
            .client
            .get(format!("{}/kyc/profile/{}", self.host, id))
            .headers(generate_headers(
                self.token(),
                self.client_identifier.clone(),
            ))
            .send()
            .await;

        parse_response::<UserKycStatusResponse>(response).await
    }

    pub async fn new_customer(
        &mut self,
        customer: CustomerRequest,
    ) -> Result<CustomerResponse, ServiceError> {
        self.attempt_token_acquisition().await;
        let response = self
            .client
            .post(format!("{}/user/profile/new", self.host))
            .body(json!(customer).to_string())
            .headers(generate_headers(
                self.token(),
                self.client_identifier.clone(),
            ))
            .send()
            .await;

        parse_response::<CustomerResponse>(response).await
    }

    pub async fn update_customer(
        &mut self,
        customer: CustomerRequest,
    ) -> Result<CustomerResponse, ServiceError> {
        self.attempt_token_acquisition().await;
        let response = self
            .client
            .patch(format!("{}/user/profile/update", self.host))
            .body(json!(customer).to_string())
            .headers(generate_headers(
                self.token(),
                self.client_identifier.clone(),
            ))
            .send()
            .await;

        parse_response::<CustomerResponse>(response).await
    }
}
