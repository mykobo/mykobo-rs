use crate::message_bus::MetaData;
use aws_sdk_sqs::Client;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug)]
pub struct SQSMessage {
    pub body: String,
    pub group: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SQSError {
    pub message: String,
}

impl SQSError {
    pub fn new(message: String) -> Self {
        SQSError { message }
    }
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub client: Client,
    pub queue_endpoint: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde_with::skip_serializing_none]
pub struct MessageEnvelope<T>
where
    T: Serialize,
{
    pub meta_data: MetaData,
    pub payload: T,
}

impl Display for MessageEnvelope<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::json!(self))
    }
}
