use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct WalletProfile {
    pub profile_id: String,
}
