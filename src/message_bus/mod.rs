use aws_config::meta::region::RegionProviderChain;
use aws_config::Region;
use aws_sdk_sqs::error::SdkError;
use aws_sdk_sqs::operation::RequestId;
use aws_sdk_sqs::types::{Message, MessageAttributeValue};
use aws_sdk_sqs::Client;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{debug, info, warn};

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

pub async fn configure_client() -> ClientConfig {
    let queue_endpoint = env::var("QUEUE_ENDPOINT").expect("QUEUE_ENDPOINT must be set");
    let region = env::var("AWS_REGION").ok();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("eu-west-1"));

    let shared_config = aws_config::from_env()
        .endpoint_url(queue_endpoint.to_owned())
        .region(region_provider)
        .load()
        .await;
    let client = Client::new(&shared_config);

    ClientConfig {
        client,
        queue_endpoint,
    }
}
pub async fn send_message(
    client_config: &Arc<ClientConfig>,
    queue: &str,
    message: &SQSMessage,
    attributes: HashMap<String, String>,
) -> Result<Option<String>, SQSError> {
    let queue_url = format!("{}/{}", client_config.queue_endpoint, queue);
    println!("Sending message to queue with URL: {}", queue_url);

    let msg = client_config
        .client
        .send_message()
        .queue_url(queue_url)
        .message_body(&message.body);

    let msg = attributes.into_iter().fold(msg, |acc, (k, v)| {
        let attr = MessageAttributeValue::builder()
            .string_value(v)
            .data_type("String".to_string())
            .build();
        match attr {
            Ok(attribute) => acc.message_attributes(k, attribute),
            Err(err) => {
                warn!("Could not create message attribute: {}", err);
                acc
            }
        }
    });

    let rsp = match &message.group {
        Some(group) => msg.message_group_id(group.to_string()),
        None => msg,
    };

    rsp.clone()
        .send()
        .await
        .map(|output| output.message_id)
        .map_err(|err| SQSError::new(format!("Error sending message: {:#?}", err)))
}

pub async fn receive(client_config: &Arc<ClientConfig>, queue: &str, tx: &mut Sender<Message>) {
    debug!(
        "Attempting retrieval of messages from queue with url: {}/{}",
        client_config.queue_endpoint, queue
    );
    match client_config
        .client
        .receive_message()
        .queue_url(format!("{}/{}", client_config.queue_endpoint, queue))
        .send()
        .await
    {
        Ok(received_messages) => {
            if received_messages.messages.is_some() {
                debug!("Successfully retrieved message ");
                for m in received_messages.messages.unwrap_or_default() {
                    if (tx.send(m.clone()).await).is_ok() {
                        info!("Message sent to channel for processing...")
                    } else {
                        warn!("Could not send message to channel")
                    }
                }
            } else {
                debug!("No messages yet")
            }
        }
        Err(e) => match e {
            SdkError::ConstructionFailure(_) => {
                warn!("We did not construct this request properly")
            }
            SdkError::TimeoutError(_) => warn!("Request timed out"),
            SdkError::DispatchFailure(_) => {
                warn!("There might have been an error sending this request")
            }
            SdkError::ResponseError(response_error) => {
                let error = response_error.raw();
                let error_body = error.body();
                if error.status().is_client_error() {
                    warn!("We did something wrong response returned: {:?}", error_body)
                } else {
                    warn!("Issue with the service {}", response_error.raw().status())
                }
            }
            SdkError::ServiceError(service_error) => {
                if service_error.raw().status().is_client_error() {
                    warn!(
                        "We did something wrong [QUEUE: {}], {:?}",
                        queue,
                        service_error.raw().body()
                    )
                } else {
                    warn!("Issue with the service {}", service_error.raw().status())
                }
            }
            _ => warn!("Unknown Error"),
        },
    }
}

pub async fn delete_message(client_config: &Arc<ClientConfig>, queue: &str, msg_id: &str) {
    let queue_url = format!("{}/{}", client_config.queue_endpoint, queue);
    match client_config
        .client
        .delete_message()
        .queue_url(queue_url.clone())
        .receipt_handle(msg_id)
        .send()
        .await
    {
        Ok(c) => info!(
            "Message deleted {}",
            c.request_id().unwrap_or("Unknown Request ID")
        ),
        Err(err) => warn!("Could not delete message {}{}", queue_url, err),
    }
}

pub fn create_channel(channel_size: usize) -> (Sender<Message>, Receiver<Message>) {
    tokio::sync::mpsc::channel(channel_size)
}
