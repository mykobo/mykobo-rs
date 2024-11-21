use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RefreshToken {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenCheckRequest {
    pub token: String,
    pub scope: Option<String>,
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidateToken {
    pub token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Credentials {
    pub access_key: String,
    pub secret_key: String,
}

impl Credentials {
    pub fn new(access_key: String, secret_key: String) -> Self {
        Self {
            access_key,
            secret_key,
        }
    }
}
