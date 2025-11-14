use crate::anchor::models::StellarTransaction as Transaction;
use crate::models::error::ServiceError;
use crate::util::{generate_headers, parse_response};
use log::debug;
use reqwest::Client;

pub struct StellarAnchor {
    pub host: String,
    pub client: Client,
}

impl StellarAnchor {
    pub fn new(host: String) -> Self {
        StellarAnchor {
            host,
            client: Client::new(),
        }
    }

    fn host(&self) -> String {
        self.host.clone()
    }

    pub async fn get_transaction(&self, transaction_id: &str) -> Result<Transaction, ServiceError> {
        let url = format!("{}/transactions/{}", self.host, transaction_id);
        debug!("Requesting transaction data from {}", self.host());
        let response = self
            .client
            .get(url)
            .headers(generate_headers(None, None))
            .send()
            .await;
        parse_response::<Transaction>(response).await
    }
}
