use crate::anchor::models::DappTransaction as Transaction;
use crate::identity::models::ServiceToken;
use crate::models::error::ServiceError;
use crate::util::{generate_headers, parse_response};
use log::debug;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TransactionEnvelope {
    transaction: Transaction,
}

pub struct DappAnchor {
    pub host: String,
    pub client: Client,
}

impl DappAnchor {
    pub fn new(host: String) -> Self {
        DappAnchor {
            host,
            client: Client::new(),
        }
    }

    fn host(&self) -> String {
        self.host.clone()
    }

    pub async fn get_transaction(&self, service_token: ServiceToken, transaction_id: &str) -> Result<Transaction, ServiceError> {
        let url = format!("{}/v1/transactions/{}", self.host, transaction_id);
        debug!("Requesting transaction data from {}", self.host());
        let response = self
            .client
            .get(url)
            .headers(generate_headers(Some(service_token), None))
            .send()
            .await;
        parse_response::<TransactionEnvelope>(response)
            .await
            .map(|envelope| envelope.transaction)
    }
}
