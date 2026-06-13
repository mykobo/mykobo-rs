use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde_with::skip_serializing_none]
pub struct TransactionFilterRequest {
    pub sources: Option<Vec<String>>,
    pub transaction_types: Option<Vec<String>>,
    pub statuses: Option<Vec<String>>,
    pub currencies: Option<Vec<String>>,
    /// Start date for filtering transactions (ISO formatted datetime string
    #[serde(rename = "from")]
    pub from_date: Option<String>,
    /// End date for filtering transactions (ISO formatted datetime string
    #[serde(rename = "to")]
    pub to_date: Option<String>,
    pub payee: Option<String>,
    pub payer: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

impl Default for TransactionFilterRequest {
    fn default() -> Self {
        Self {
            sources: None,
            transaction_types: None,
            statuses: None,
            currencies: None,
            from_date: None,
            to_date: None,
            payee: None,
            payer: None,
            page: Some(1),
            limit: Some(10),
        }
    }
}
