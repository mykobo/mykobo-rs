use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReferenceRequest {
    pub profile_id: String,
    pub wallet_address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_domain: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceResponse {
    pub id: String,
    pub profile_id: String,
    pub reference: String,
    pub is_active: bool,
    pub wallet_address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_domain: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
}
