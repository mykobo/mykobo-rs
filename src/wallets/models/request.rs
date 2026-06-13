use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterWalletRequest {
    pub profile_id: String,
    pub public_key: String,
    pub memo: Option<String>,
    pub chain: String,
}
