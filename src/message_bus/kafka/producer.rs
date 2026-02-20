use crate::models::error::{KafkaError, KafkaResult};
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;
use std::env;
use std::time::Duration;

pub struct EventProducer {
    producer: FutureProducer,
    topic: String,
    timeout: Duration,
}

impl EventProducer {
    pub fn new(brokers: &str, timeout_in_secs: u64, topic: &str) -> KafkaResult<Self> {
        let sasl_username = env::var("KAFKA_API_KEY").ok();
        let sasl_password = env::var("KAFKA_API_SECRET").ok();
        let protocol = env::var("KAFKA_API_PROTOCOL").unwrap_or("SASL_SSL".to_string());
        let mechanism = env::var("KAFKA_API_SASL_MECHANISM").unwrap_or("PLAIN".to_string());

        let mut config = ClientConfig::new();
        config
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", timeout_in_secs.to_string())
            .set("request.timeout.ms", "60000")
            .set("compression.type", "gzip")
            .set("retries", "3")
            .set("retry.backoff.ms", "500")
            .set("request.required.acks", "all")
            .set("queue.buffering.max.messages", "100000")
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

        let producer: FutureProducer = config
            .set_log_level(RDKafkaLogLevel::Info)
            .create()
            .map_err(|e| KafkaError::ClientCreation(e.to_string()))?;

        Ok(EventProducer {
            producer,
            topic: topic.to_string(),
            timeout: Duration::from_secs(timeout_in_secs),
        })
    }

    pub async fn send_event<T: Serialize>(&self, key: String, payload: T) -> KafkaResult<()> {
        let payload_json =
            serde_json::to_string(&payload).map_err(|e| KafkaError::MessageSend(e.to_string()))?;
        let record: FutureRecord<String, String> = FutureRecord::to(&self.topic)
            .headers(OwnedHeaders::new().insert(Header {
                key: "source",
                value: Some("kafka_test_rs/producer.rs"),
            }))
            .payload(&payload_json)
            .key(&key);

        self.producer
            .send(record, self.timeout)
            .await
            .map_err(|(err, _)| KafkaError::MessageSend(err.to_string()))?;

        Ok(())
    }
}
