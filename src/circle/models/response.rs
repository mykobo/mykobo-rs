use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RelayAddress {
    pub id: String,
    pub address: String,
    pub chain: String,
    pub active: bool,
    pub label: Option<String>,
    pub counterpart_id: Option<String>,
    pub external_address: Option<String>,
    pub client_domain: Option<String>,
    pub email: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelayAddressPair {
    pub source: RelayAddress,
    pub destination: RelayAddress,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircleAddress {
    pub id: String,
    pub address: String,
    pub address_tag: Option<String>,
    pub chain: String,
    pub currency: String,
    pub purpose: String,
    pub wallet_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub amount: String,
    pub chain: String,
    pub token: String,
    pub sender: String,
    pub recipient: String,
    pub status: String,
    pub tx_hash: String,
    pub block_number: Option<i64>,
    pub timestamp: String,
    pub received_at: String,
    pub circle_transfer_id: Option<String>,
    pub deposit_tx_hash: Option<String>,
    pub outbound_tx_hash: Option<String>,
    pub failed_status: Option<String>,
    pub failure_reason: Option<String>,
    pub fee_amount: Option<String>,
    pub transfer_amount: Option<String>,
    pub raw_data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedTransactions {
    pub data: Vec<Transaction>,
    pub page: i32,
    pub pages: i32,
    pub per_page: i32,
    pub total: i32,
}
