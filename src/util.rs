use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde::de::DeserializeOwned;
use uuid::Uuid;

use crate::models::response::{auth::ServiceToken, MykoboStatusCode, ServiceError};

pub fn generate_headers(token: Option<ServiceToken>, client_id: Option<String>) -> HeaderMap {
    let mut headers = HeaderMap::new();

    if let Some(token) = token {
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", token.token).parse().unwrap(),
        );
    }

    if let Some(client_id) = client_id {
        headers.insert(USER_AGENT, client_id.parse().unwrap());
    };

    headers.insert(
        CONTENT_TYPE,
        "application/json".to_string().parse().unwrap(),
    );

    headers
}

pub async fn parse_response<T: DeserializeOwned>(
    response: Result<reqwest::Response, reqwest::Error>,
) -> Result<T, ServiceError> {
    match response {
        Ok(response) => {
            if response.status().is_success() {
                Ok(response.json::<T>().await?)
            } else {
                let status = response.status();
                let error = response.json::<ServiceError>().await?;
                let updated = ServiceError {
                    status: MykoboStatusCode::from(status),
                    ..error
                };
                Err(updated)
            }
        }
        Err(e) => Err(ServiceError::from(e)),
    }
}

/**
 * Generates a unique identifier with the given prefix. NOTE no trailing colon.
 */
pub fn generate_id(prefix: &str) -> String {
    format!("{}:{}", prefix, Uuid::new_v4().to_string().replace('-', ""))
}

pub const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%SZ";
