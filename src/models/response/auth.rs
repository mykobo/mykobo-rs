use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceToken {
    pub subject_id: String,
    pub token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenCheckResponse {
    pub authorised: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub aud: String,
    pub scope: Vec<String>,
}
