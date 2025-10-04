use chrono::{SecondsFormat, Utc};
use serde::Serialize;
use uuid::Uuid;
pub mod kafka;
pub mod sqs;

#[derive(Serialize, Debug, Clone)]
pub struct Metadata {
    pub source: String,
    pub event: String,
    pub created_at: String,
    pub token: String,
    pub idempotency_key: String,
}

pub fn generate_meta_data(
    event: &str,
    source: &str,
    token: &str,
    idempotency_key: Option<String>,
    date_time_override: Option<String>,
) -> Metadata {
    Metadata {
        event: event.to_string(),
        source: source.to_string(),
        created_at: date_time_override
            .unwrap_or(Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)),
        token: token.to_string(),
        idempotency_key: idempotency_key.unwrap_or(Uuid::new_v4().to_string()),
    }
}
