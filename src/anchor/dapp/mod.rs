use crate::anchor::models::DappTransaction as Transaction;
use crate::models::error::ServiceError;
use crate::util::{generate_headers, parse_response};
use log::debug;
use reqwest::Client;

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

    pub async fn get_transaction(&self, transaction_id: &str) -> Result<Transaction, ServiceError> {
        let url = format!("{}/api/transactions/{}", self.host, transaction_id);
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
