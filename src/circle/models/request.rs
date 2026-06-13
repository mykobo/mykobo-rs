use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RelayAddressSide {
    pub chain: String,
    pub address: String,
    pub private_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRelayAddressPairRequest {
    pub source: RelayAddressSide,
    pub destination: RelayAddressSide,
}
