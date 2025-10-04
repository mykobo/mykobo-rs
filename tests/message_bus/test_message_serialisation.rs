use mykobo_rs::message_bus::generate_meta_data;
use mykobo_rs::message_bus::sqs::models::{MessageEnvelope, SQSMessage};
use pretty_assertions::assert_eq;

#[test]
fn test_message_serialisation() {
    let message = MessageEnvelope {
        meta_data: generate_meta_data(
            "TestTriggeredEvent",
            "TestFunction",
            "eyw83485yh9wehf89wr9gbsdfgksfg",
            Some("02fb1ad1-103e-49a3-bdcc-4fba95f0f573".to_string()),
            Some("2025-07-26T18:16:52Z".to_string()),
        ),
        payload: "Test Payload".to_string(),
    };

    let sqs_message = SQSMessage {
        body: message.to_string(),
        group: None,
    };

    let json = r#"{"meta_data":{"created_at":"2025-07-26T18:16:52Z","event":"TestTriggeredEvent","idempotency_key":"02fb1ad1-103e-49a3-bdcc-4fba95f0f573","source":"TestFunction","token":"eyw83485yh9wehf89wr9gbsdfgksfg"},"payload":"Test Payload"}"#;
    assert_eq!(sqs_message.body, json);
}
