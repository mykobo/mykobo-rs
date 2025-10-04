use mykobo_rs::message_bus::kafka::configure_consumer;
use std::env;

#[test]
#[should_panic(expected = "Missing KAFKA_API_KEY environment variable")]
fn test_configure_consumer_missing_api_key() {
    env::remove_var("KAFKA_API_KEY");
    env::set_var("KAFKA_API_SECRET", "test_secret");

    let brokers = "localhost:9092";
    let group_id = "test_group";

    let _ = configure_consumer(brokers, group_id, None);
}

#[test]
#[should_panic(expected = "Missing KAFKA_API_SECRET environment variable")]
fn test_configure_consumer_missing_api_secret() {
    env::set_var("KAFKA_API_KEY", "test_key");
    env::remove_var("KAFKA_API_SECRET");

    let brokers = "localhost:9092";
    let group_id = "test_group";

    let _ = configure_consumer(brokers, group_id, None);
}
