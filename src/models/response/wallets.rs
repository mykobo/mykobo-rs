use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct WalletProfile {
    pub profile_id: String,
}
