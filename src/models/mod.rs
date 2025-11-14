pub mod error;

use serde::Serialize;

// Re-export commonly used error types
pub use error::{KafkaError, KafkaResult, MykoboStatusCode, ServiceError};

#[derive(Debug, Clone, Serialize)]
pub struct AuthError {
    pub message: String,
}

impl AuthError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
