use crate::message_bus::kafka::models::IncomingMessage;
use crate::models::error::{KafkaError, KafkaResult};
use futures::StreamExt;
use log::{debug, error, warn};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer};
use rdkafka::message::{Headers, OwnedMessage};
use rdkafka::{ClientConfig, Message};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use tokio::sync::mpsc::Sender;

pub struct EventConsumer<T> {
    consumer: StreamConsumer,
    max_retries: u32,
    channel: Sender<IncomingMessage<T>>,
}

impl<T> EventConsumer<T>
where
    for<'a> T: Deserialize<'a>,
{
    pub fn new(
        brokers: &str,
        group_id: &str,
        client_id: &str,
        max_retries: u32,
        topics: &[&str],
        channel: Sender<IncomingMessage<T>>,
    ) -> KafkaResult<Self> {
        let sasl_username =
            env::var("KAFKA_API_KEY").expect("Missing KAFKA_API_KEY environment variable");
        let sasl_password =
            env::var("KAFKA_API_SECRET").expect("Missing KAFKA_API_SECRET environment variable");

        let consumer: StreamConsumer = ClientConfig::new()
            .set("client.id", client_id)
            .set("group.id", group_id)
            .set("bootstrap.servers", brokers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .set("security.protocol", "SASL_SSL")
            .set("sasl.mechanisms", "PLAIN")
            .set("sasl.username", sasl_username)
            .set("sasl.password", sasl_password)
            .set("retries", "3")
            .set_log_level(RDKafkaLogLevel::Info)
            .create()
            .map_err(|e| KafkaError::ClientCreation(e.to_string()))?;

        consumer
            .subscribe(topics)
            .expect("Can't subscribe to specified topics");

        Ok(EventConsumer {
            consumer,
            max_retries,
            channel,
        })
    }

    pub async fn start(&self) -> KafkaResult<()> {
        let mut message_stream = self.consumer.stream();

        while let Some(message_result) = message_stream.next().await {
            match message_result {
                Ok(message) => {
                    match self
                        .process_with_retry(message.detach(), self.channel.clone())
                        .await
                    {
                        Ok(_) => {
                            debug!("Message processed successfully, committing offset");
                            self.consumer
                                .commit_message(&message, CommitMode::Async)
                                .map_err(|e| KafkaError::MessageDelivery(e.to_string()))?;
                        }
                        Err(e) => {
                            error!("Failed to process message: {}", e);
                            // Implement dead letter queue logic here
                        }
                    }
                }
                Err(e) => {
                    error!("Error receiving message: {}", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }

        Ok(())
    }
    async fn process_with_retry(
        &self,
        message: OwnedMessage,
        message_channel: Sender<IncomingMessage<T>>,
    ) -> KafkaResult<()> {
        let mut retries = 0;
        let mut backoff = Duration::from_secs(1);

        while retries < self.max_retries {
            match self.parse_message(&message) {
                Ok(incoming_message) => match message_channel.send(incoming_message).await {
                    Ok(_) => {
                        debug!("Successfully forwarded incoming message to channel");
                        return Ok(());
                    }
                    Err(e) => error!("Failed to send message to channel: {e}"),
                },
                Err(e) => {
                    warn!("Retry {} failed: {}", retries, e);
                    retries += 1;
                    tokio::time::sleep(backoff).await;
                    backoff *= 2;
                }
            }
        }

        Err(KafkaError::MessageDelivery("Max retries exceeded".into()))
    }

    fn parse_message(&self, message: &OwnedMessage) -> KafkaResult<IncomingMessage<T>> {
        let headers = match message.headers() {
            Some(maybe_headers) => maybe_headers
                .iter()
                .map(|h| {
                    (
                        h.key.to_string(),
                        h.value
                            .map(|header_value| {
                                str::from_utf8(header_value).unwrap_or("").to_string()
                            })
                            .unwrap_or_default(),
                    )
                })
                .collect::<HashMap<String, String>>(),
            None => {
                warn!("No headers found in the message");
                HashMap::new()
            }
        };

        let payload = message.clone().payload().unwrap_or_default().to_vec();
        match serde_json::from_slice(payload.as_slice()) {
            Ok(content) => Ok(IncomingMessage {
                headers,
                payload: content,
            }),
            Err(e) => {
                error!("Failed to deserialize message payload: {}", e);
                Err(KafkaError::Deserialization(e))
            }
        }
    }
}
