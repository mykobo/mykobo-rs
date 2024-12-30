pub mod request;
pub mod response;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AuthError {
    pub message: String,
}

impl AuthError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
