use crate::message_bus::kafka::models::{CustomContext, IncomingMessage};
use log::{error, warn};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer};
use rdkafka::message::Headers;
use rdkafka::{ClientConfig, Message};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

pub mod models;
pub fn configure_consumer(
    brokers: &str,
    group_id: &str,
    assignor: Option<&String>,
) -> StreamConsumer<CustomContext> {
    let context = CustomContext;
    let sasl_username =
        env::var("KAFKA_API_KEY").expect("Missing KAFKA_API_KEY environment variable");
    let sasl_password =
        env::var("KAFKA_API_SECRET").expect("Missing KAFKA_API_SECRET environment variable");
    let mut config = ClientConfig::new();

    config
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", "PLAIN")
        .set("sasl.username", sasl_username)
        .set("sasl.password", sasl_password)
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Info);

    if let Some(assignor) = assignor {
        config
            .set("group.remote.assignor", assignor)
            .set("group.protocol", "consumer")
            .remove("session.timeout.ms");
    }

    config
        .create_with_context(context)
        .expect("Consumer creation failed")
}

pub async fn receive<'a, T: Default + for<'de> Deserialize<'de>>(
    consumer: &'a StreamConsumer<CustomContext>,
    topics: &[&str],
) -> IncomingMessage<T> {
    consumer
        .subscribe(topics)
        .expect("Can't subscribe to specified topics");

    match consumer.recv().await {
        Err(e) => {
            error!("Kafka error: {}", e);
            IncomingMessage::default()
        }
        Ok(m) => {
            let headers = match m.headers().map(|h| h) {
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
            consumer.commit_message(&m, CommitMode::Async).unwrap();
            let owned_message = m.detach();
            let payload = owned_message.clone().payload().unwrap_or_default().to_vec();
            IncomingMessage {
                headers,
                payload: { serde_json::from_slice(payload.as_slice()).unwrap_or_default() },
            }
        }
    }
}
