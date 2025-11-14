use reqwest::{Error, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum MykoboStatusCode {
    NotFound,
    BadRequest,
    Unauthorised,
    #[default]
    DependencyFailed,
    InternalServerError,
    Conflict,
}

impl From<StatusCode> for MykoboStatusCode {
    fn from(status: StatusCode) -> Self {
        match status {
            StatusCode::NOT_FOUND => MykoboStatusCode::NotFound,
            StatusCode::BAD_REQUEST => MykoboStatusCode::BadRequest,
            StatusCode::UNAUTHORIZED => MykoboStatusCode::Unauthorised,
            StatusCode::INTERNAL_SERVER_ERROR => MykoboStatusCode::InternalServerError,
            StatusCode::CONFLICT => MykoboStatusCode::Conflict,
            _ => MykoboStatusCode::DependencyFailed,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceError {
    pub error: Option<String>,
    pub message: Option<String>,
    pub description: Option<String>,
    #[serde(default = "MykoboStatusCode::default")]
    pub status: MykoboStatusCode,
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = self
            .error
            .clone()
            .unwrap_or("Unknown error - U".to_string());
        write!(f, "{message}")
    }
}

impl From<Error> for ServiceError {
    fn from(error: Error) -> Self {
        let r_message = if error.is_connect() || error.is_timeout() {
            Some("Connection error".to_string())
        } else if error.is_request() {
            Some("Bad Request".to_string())
        } else {
            Some(format!("{error}"))
        };

        ServiceError {
            error: r_message.clone().unwrap().to_string().into(),
            message: r_message,
            description: None,
            status: MykoboStatusCode::from(error.status().unwrap_or(StatusCode::BAD_REQUEST)),
        }
    }
}

#[derive(Debug, Error)]
pub enum KafkaError {
    #[error("Failed to create Kafka client: {0}")]
    ClientCreation(String),

    #[error("Failed to send message: {0}")]
    MessageSend(String),

    #[error("Message delivery failed: {0}")]
    MessageDelivery(String),

    #[error("Failed to deserialize message: {0}")]
    Deserialization(#[from] serde_json::Error),

    #[error("Connection timeout: {0}")]
    Timeout(String),
}

pub type KafkaResult<T> = Result<T, KafkaError>;
