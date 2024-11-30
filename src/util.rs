use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde::de::DeserializeOwned;

use crate::models::response::{auth::ServiceToken, MykoboStatusCode, ServiceError};

pub fn generate_headers(token: Option<ServiceToken>, client_id: Option<String>) -> HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();

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
                let service_error = match response.text().await {
                    Ok(text) => serde_json::from_str::<ServiceError>(text.as_str()).unwrap_or(
                        ServiceError {
                            error: Some(text.to_string()),
                            message: Some(text),
                            status: MykoboStatusCode::from(status),
                        },
                    ),
                    Err(e) => ServiceError {
                        error: Some(format!("{:?}", e).to_string()),
                        message: Some(format!("{:?}", e).to_string()),
                        status: e
                            .status()
                            .map(MykoboStatusCode::from)
                            .unwrap_or(MykoboStatusCode::DependencyFailed),
                    },
                };
                Err(service_error)
            }
        }
        Err(e) => Err(ServiceError::from(e)),
    }
}
