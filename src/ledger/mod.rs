pub mod models;

use std::env;

use log::info;
use reqwest::Client;

use crate::{
    identity::models::ServiceToken,
    models::error::ServiceError,
    util::{generate_headers, parse_response},
};

use crate::ledger::models::response::TransactionDetailsResponse;
use models::{
    ComplianceEventsResponse, TransactionFilterRequest, TransactionListResponse,
    TransactionResponse, TransactionStatusesResponse,
};

#[derive(Clone)]
pub struct LedgerServiceClient {
    pub host: String,
    pub client: Client,
    pub max_retries: i8,
    pub client_identifier: Option<String>,
}

impl LedgerServiceClient {
    pub fn new(max_retries: i8) -> Self {
        let ledger_host = env::var("LEDGER_SERVICE_HOST").expect("LEDGER_SERVICE_HOST must be set");

        Self {
            host: ledger_host,
            client: Client::new(),
            max_retries,
            client_identifier: Some("mykobo-rs".to_string()),
        }
    }

    /// Get a list of transactions with a set of filter options
    ///
    /// # Arguments
    /// * `token` - Authentication token
    /// * `params` - Transaction filter parameters including sources, transaction_types,
    ///   statuses, currencies, date ranges, payee, payer, and pagination
    pub async fn transaction_list(
        &self,
        token: Option<ServiceToken>,
        params: TransactionFilterRequest,
    ) -> Result<TransactionListResponse, ServiceError> {
        info!("Getting transactions with filters: {:?}", params);

        let response = self
            .client
            .post(format!("{}/transactions/list", self.host))
            .headers(generate_headers(token, self.client_identifier.clone()))
            .json(&params)
            .send()
            .await;

        parse_response::<TransactionListResponse>(response).await
    }

    /// Get transaction statuses, optionally filtered by a specific status
    ///
    /// # Arguments
    /// * `token` - Authentication token
    /// * `status` - Optional status filter to get status transitions for a specific status
    pub async fn get_transaction_statuses(
        &self,
        token: Option<ServiceToken>,
        status: Option<&str>,
    ) -> Result<TransactionStatusesResponse, ServiceError> {
        let url = if let Some(s) = status {
            format!("{}/transactions/statuses/transitions/{}", self.host, s)
        } else {
            format!("{}/transactions/statuses", self.host)
        };

        let response = self
            .client
            .get(url)
            .headers(generate_headers(token, self.client_identifier.clone()))
            .send()
            .await;

        parse_response::<TransactionStatusesResponse>(response).await
    }

    /// Get a single standalone transaction by reference
    ///
    /// # Arguments
    /// * `token` - Authentication token
    /// * `reference` - Transaction reference
    pub async fn get_transaction_by_reference(
        &self,
        token: Option<ServiceToken>,
        reference: &str,
    ) -> Result<TransactionResponse, ServiceError> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/reference/{}",
                self.host, reference
            ))
            .headers(generate_headers(token, self.client_identifier.clone()))
            .send()
            .await;

        parse_response::<TransactionResponse>(response).await
    }

    /// Get transaction details by reference, this will return the full transaction details including
    /// payment events and transaction statuses.
    ///
    /// # Arguments
    /// * `token` - Authentication token
    /// * `reference` - Transaction reference
    pub async fn get_transaction_details_by_reference(
        &self,
        token: Option<ServiceToken>,
        reference: &str,
    ) -> Result<TransactionDetailsResponse, ServiceError> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/reference/{}/details",
                self.host, reference
            ))
            .headers(generate_headers(token, self.client_identifier.clone()))
            .send()
            .await;

        parse_response::<TransactionDetailsResponse>(response).await
    }

    /// Get transaction details by external ID
    ///
    /// # Arguments
    /// * `token` - Authentication token
    /// * `external_id` - External transaction ID
    pub async fn get_transaction_by_external_id(
        &self,
        token: Option<ServiceToken>,
        external_id: &str,
    ) -> Result<TransactionResponse, ServiceError> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/external/{}",
                self.host, external_id
            ))
            .headers(generate_headers(token, self.client_identifier.clone()))
            .send()
            .await;

        parse_response::<TransactionResponse>(response).await
    }

    /// Get compliance events for a transaction by reference
    ///
    /// # Arguments
    /// * `token` - Authentication token
    /// * `reference` - Transaction reference
    pub async fn get_transaction_compliance_events(
        &self,
        token: Option<ServiceToken>,
        reference: &str,
    ) -> Result<ComplianceEventsResponse, ServiceError> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/reference/{}/compliance",
                self.host, reference
            ))
            .headers(generate_headers(token, self.client_identifier.clone()))
            .send()
            .await;

        parse_response::<ComplianceEventsResponse>(response).await
    }
}
