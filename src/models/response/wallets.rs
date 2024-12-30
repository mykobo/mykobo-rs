use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct WalletProfile {
    pub profile_id: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct WalletData {
    pub id: String,
    pub memo: Option<String>,
    pub network: String,
    pub public_key: String,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserWallet {
    pub profile_id: String,
    pub created_at: NaiveDateTime,
    pub wallets: Vec<WalletData>,
}
