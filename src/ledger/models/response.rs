use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionListResponse {
    pub transactions: Vec<TransactionResponse>,
    pub page: u8,
    pub limit: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub id: String,
    pub external_reference: Option<String>,
    pub source: String,
    pub reference: String,
    pub transaction_type: String,
    pub status: String,
    pub incoming_currency: String,
    pub outgoing_currency: String,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub requested_amount: BigDecimal,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub expected_amount_in: BigDecimal,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub amount_out: BigDecimal,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub amount_in: BigDecimal,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub fee: BigDecimal,
    pub payer: Option<String>,
    pub payee: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub requester_first_name: String,
    pub requester_last_name: String,
    pub originating_ip_address: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TransactionEvent {
    Payment {
        id: String,
        external_reference: String,
        source: String,
        reference: String,
        payer_name: String,
        currency: String,
        value: String,
        bank_account_number: String,
        created_at: NaiveDateTime,
        applied_at: Option<NaiveDateTime>,
    },
    StatusUpdate {
        id: String,
        reference: String,
        status: String,
        message: String,
        created_at: NaiveDateTime,
        applied_at: Option<NaiveDateTime>,
    },
    Correction {
        id: String,
        reference: String,
        value: BigDecimal,
        message: String,
        currency: String,
        source: String,
        created_at: NaiveDateTime,
        applied_at: Option<NaiveDateTime>,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDetailsResponse {
    pub transaction: TransactionResponse,
    pub events: Vec<TransactionEvent>,
}

pub type ComplianceEventsResponse = HashMap<String, bool>;
pub type TransactionStatusesResponse = Vec<String>;
