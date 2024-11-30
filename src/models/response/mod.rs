use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub mod auth;
pub mod identity;
pub mod wallets;

#[derive(Debug, Serialize, Deserialize)]
pub enum MykoboStatusCode {
    NotFound,
    BadRequest,
    Unauthorised,
    DependencyFailed,
}

impl From<StatusCode> for MykoboStatusCode {
    fn from(status: StatusCode) -> Self {
        match status {
            StatusCode::NOT_FOUND => MykoboStatusCode::NotFound,
            StatusCode::BAD_REQUEST => MykoboStatusCode::BadRequest,
            StatusCode::UNAUTHORIZED => MykoboStatusCode::Unauthorised,
            _ => MykoboStatusCode::DependencyFailed,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceError {
    pub error: Option<String>,
    pub message: Option<String>,
    pub status: MykoboStatusCode,
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = self
            .error
            .clone()
            .unwrap_or("Unknown error - U".to_string());
        write!(f, "{}", message)
    }
}

impl From<reqwest::Error> for ServiceError {
    fn from(error: reqwest::Error) -> Self {
        let r_message = if error.is_connect() || error.is_timeout() {
            Some("Connection error".to_string())
        } else if error.is_request() {
            Some("Bad Request".to_string())
        } else {
            Some("An unknown error occurred".to_string())
        };

        ServiceError {
            error: r_message.clone().unwrap().to_string().into(),
            message: r_message,
            status: MykoboStatusCode::from(error.status().unwrap_or(StatusCode::BAD_REQUEST)),
        }
    }
}
