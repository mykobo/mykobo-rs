use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Amount {
    pub amount: String,
    pub asset: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FeeDetail {
    pub name: String,
    pub description: String,
    pub amount: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FeeDetails {
    pub total: String,
    pub asset: String,
    #[serde(default = "Vec::new")]
    pub details: Vec<FeeDetail>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Customer {
    pub account: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Customers {
    pub sender: Customer,
    pub receiver: Customer,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Creator {
    pub account: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub sep: String,
    pub kind: String,
    pub status: String,
    pub amount_expected: Amount,
    pub amount_in: Amount,
    pub amount_out: Amount,
    pub fee_details: FeeDetails,
    pub started_at: String,
    pub updated_at: Option<String>,
    pub external_transaction_id: Option<String>,
    pub message: Option<String>,
    pub destination_account: Option<String>,
    pub customers: Customers,
    pub creator: Creator,
    pub client_domain: Option<String>,
    pub client_name: Option<String>,
    pub funding_method: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum AnchorRpcResponseResult {
    Transaction(Transaction),
}

#[derive(Deserialize, Debug, Clone)]
pub struct AnchorRpcResponse {
    pub jsonrpc: String,
    pub result: AnchorRpcResponseResult,
}
