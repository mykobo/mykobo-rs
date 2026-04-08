pub mod models;

use log::debug;
use reqwest::Client;
use serde_json::json;

use crate::{
    identity::models::ServiceToken,
    models::error::ServiceError,
    util::{generate_headers, parse_response},
};
use models::{CircleAddress, CreateRelayAddressPairRequest, PaginatedTransactions, RelayAddress, Transaction};
use models::response::RelayAddressPair;

#[derive(Clone)]
pub struct CircleServiceClient {
    pub host: String,
    pub client: Client,
    pub client_identifier: Option<String>,
}

impl CircleServiceClient {
    pub fn new(host: String) -> Self {
        Self {
            host,
            client: Client::new(),
            client_identifier: Some("mykobo-rs".to_string()),
        }
    }

    pub async fn health(&self) -> Result<reqwest::Response, ServiceError> {
        let response = self
            .client
            .get(format!("{}/health", self.host))
            .send()
            .await?;

        Ok(response)
    }

    pub async fn create_relay_address_pair(
        &self,
        token: Option<ServiceToken>,
        request: CreateRelayAddressPairRequest,
    ) -> Result<RelayAddressPair, ServiceError> {
        debug!("Creating relay address pair...");
        let payload = json!(request).to_string();
        let response = self
            .client
            .post(format!("{}/relay-addresses/pair", self.host))
            .headers(generate_headers(token, self.client_identifier.clone()))
            .body(payload)
            .send()
            .await;

        parse_response::<RelayAddressPair>(response).await
    }

    pub async fn list_relay_addresses(
        &self,
        token: Option<ServiceToken>,
        chain: Option<&str>,
    ) -> Result<Vec<RelayAddress>, ServiceError> {
        debug!("Listing relay addresses...");
        let mut url = format!("{}/relay-addresses", self.host);
        if let Some(chain) = chain {
            url = format!("{}?chain={}", url, chain);
        }

        let response = self
            .client
            .get(url)
            .headers(generate_headers(token, self.client_identifier.clone()))
            .send()
            .await;

        parse_response::<Vec<RelayAddress>>(response).await
    }

    pub async fn list_circle_addresses(
        &self,
        token: Option<ServiceToken>,
        chain: Option<&str>,
        currency: Option<&str>,
        purpose: Option<&str>,
    ) -> Result<Vec<CircleAddress>, ServiceError> {
        debug!("Listing circle addresses...");
        let mut params = Vec::new();
        if let Some(chain) = chain {
            params.push(format!("chain={}", chain));
        }
        if let Some(currency) = currency {
            params.push(format!("currency={}", currency));
        }
        if let Some(purpose) = purpose {
            params.push(format!("purpose={}", purpose));
        }

        let url = if params.is_empty() {
            format!("{}/circle-addresses", self.host)
        } else {
            format!("{}/circle-addresses?{}", self.host, params.join("&"))
        };

        let response = self
            .client
            .get(url)
            .headers(generate_headers(token, self.client_identifier.clone()))
            .send()
            .await;

        parse_response::<Vec<CircleAddress>>(response).await
    }

    pub async fn list_transactions(
        &self,
        token: Option<ServiceToken>,
        page: Option<i32>,
        per_page: Option<i32>,
        chain: Option<&str>,
        status: Option<&str>,
        asset: Option<&str>,
        sender: Option<&str>,
        recipient: Option<&str>,
    ) -> Result<PaginatedTransactions, ServiceError> {
        debug!("Listing transactions...");
        let mut params = Vec::new();
        if let Some(page) = page {
            params.push(format!("page={}", page));
        }
        if let Some(per_page) = per_page {
            params.push(format!("per_page={}", per_page));
        }
        if let Some(chain) = chain {
            params.push(format!("chain={}", chain));
        }
        if let Some(status) = status {
            params.push(format!("status={}", status));
        }
        if let Some(asset) = asset {
            params.push(format!("token={}", asset));
        }
        if let Some(sender) = sender {
            params.push(format!("sender={}", sender));
        }
        if let Some(recipient) = recipient {
            params.push(format!("recipient={}", recipient));
        }

        let url = if params.is_empty() {
            format!("{}/transactions", self.host)
        } else {
            format!("{}/transactions?{}", self.host, params.join("&"))
        };

        let response = self
            .client
            .get(url)
            .headers(generate_headers(token, self.client_identifier.clone()))
            .send()
            .await;

        parse_response::<PaginatedTransactions>(response).await
    }

    pub async fn get_transaction(
        &self,
        token: Option<ServiceToken>,
        transaction_id: &str,
    ) -> Result<Transaction, ServiceError> {
        debug!("Getting transaction {}...", transaction_id);
        let response = self
            .client
            .get(format!("{}/transactions/{}", self.host, transaction_id))
            .headers(generate_headers(token, self.client_identifier.clone()))
            .send()
            .await;

        parse_response::<Transaction>(response).await
    }
}
