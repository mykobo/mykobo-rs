use crate::message_bus::kafka::models::IncomingMessage;
use crate::models::error::{KafkaError, KafkaResult};
use futures::StreamExt;
use log::{debug, error, info, warn};
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
        let sasl_username = env::var("KAFKA_API_KEY").ok();
        let sasl_password = env::var("KAFKA_API_SECRET").ok();
        let protocol = env::var("KAFKA_API_PROTOCOL").unwrap_or("SASL_SSL".to_string());
        let mechanism = env::var("KAFKA_API_SASL_MECHANISM").unwrap_or("PLAIN".to_string());

        let mut config = ClientConfig::new();
        config
            .set("auto.offset.reset", "earliest")
            .set("client.id", client_id)
            .set("group.id", group_id)
            .set("bootstrap.servers", brokers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "45000")
            .set("heartbeat.interval.ms", "3000")
            .set("enable.auto.commit", "true")
            .set("socket.keepalive.enable", "true")
            .set("socket.connection.setup.timeout.ms", "10000")
            .set("connections.max.idle.ms", "540000")
            .set("security.protocol", &protocol)
            .set("sasl.mechanisms", mechanism);

        if protocol == "SASL_SSL" {
            let username = sasl_username.ok_or_else(|| {
                KafkaError::ClientCreation(
                    "KAFKA_API_KEY is required when protocol is SASL_SSL".into(),
                )
            })?;
            let password = sasl_password.ok_or_else(|| {
                KafkaError::ClientCreation(
                    "KAFKA_API_SECRET is required when protocol is SASL_SSL".into(),
                )
            })?;
            config
                .set("sasl.username", username)
                .set("sasl.password", password);
        }

        let stream_consumer: StreamConsumer = config
            .set_log_level(RDKafkaLogLevel::Info)
            .create()
            .map_err(|e| KafkaError::ClientCreation(e.to_string()))?;

        stream_consumer
            .subscribe(topics)
            .expect("Can't subscribe to specified topics");

        Ok(EventConsumer {
            consumer: stream_consumer,
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
                            info!(
                                "Message processed successfully, committing offset [{}]",
                                message.offset()
                            );
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
