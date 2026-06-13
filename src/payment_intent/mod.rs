pub mod models;

use std::env;

use reqwest::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};

use crate::models::error::ServiceError;
use crate::util::{parse_empty_response, parse_response};
use models::{CreateReferenceRequest, HealthResponse, ReferenceResponse};

#[derive(Clone)]
pub struct PaymentIntentServiceClient {
    pub host: String,
    pub client: Client,
    pub max_retries: i8,
    pub client_identifier: Option<String>,
}

impl PaymentIntentServiceClient {
    pub fn new(max_retries: i8) -> Self {
        let host = env::var("PAYMENT_INTENT_SERVICE_HOST")
            .expect("PAYMENT_INTENT_SERVICE_HOST must be set");

        Self {
            host,
            client: Client::new(),
            max_retries,
            client_identifier: Some("mykobo-rs".to_string()),
        }
    }

    fn build_headers(&self, user_token: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", user_token).parse().unwrap(),
        );
        if let Some(ref client_id) = self.client_identifier {
            headers.insert(USER_AGENT, client_id.parse().unwrap());
        }
        headers.insert(
            CONTENT_TYPE,
            "application/json".to_string().parse().unwrap(),
        );
        headers
    }

    pub async fn health_check(&self) -> Result<HealthResponse, ServiceError> {
        let response = self
            .client
            .get(format!("{}/health", self.host))
            .send()
            .await;

        parse_response::<HealthResponse>(response).await
    }

    pub async fn create_reference(
        &self,
        user_token: &str,
        payload: CreateReferenceRequest,
    ) -> Result<ReferenceResponse, ServiceError> {
        let response = self
            .client
            .post(format!("{}/payment-references", self.host))
            .headers(self.build_headers(user_token))
            .json(&payload)
            .send()
            .await;

        parse_response::<ReferenceResponse>(response).await
    }

    pub async fn get_reference_by_value(
        &self,
        user_token: &str,
        reference: &str,
    ) -> Result<ReferenceResponse, ServiceError> {
        let response = self
            .client
            .get(format!(
                "{}/payment-references/{}",
                self.host, reference
            ))
            .headers(self.build_headers(user_token))
            .send()
            .await;

        parse_response::<ReferenceResponse>(response).await
    }

    pub async fn get_references_by_user(
        &self,
        user_token: &str,
        profile_id: &str,
    ) -> Result<Vec<ReferenceResponse>, ServiceError> {
        let response = self
            .client
            .get(format!(
                "{}/payment-references/user/{}",
                self.host, profile_id
            ))
            .headers(self.build_headers(user_token))
            .send()
            .await;

        parse_response::<Vec<ReferenceResponse>>(response).await
    }

    pub async fn delete_reference(
        &self,
        user_token: &str,
        reference: &str,
    ) -> Result<(), ServiceError> {
        let response = self
            .client
            .delete(format!(
                "{}/payment-references/{}",
                self.host, reference
            ))
            .headers(self.build_headers(user_token))
            .send()
            .await;

        parse_empty_response(response).await
    }
}
