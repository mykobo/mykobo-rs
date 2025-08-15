use reqwest::{Error, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub mod anchor;
pub mod auth;
pub mod identity;
pub mod sumsub;
pub mod wallets;

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
