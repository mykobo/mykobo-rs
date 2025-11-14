use mykobo_rs::message_bus::sqs::models::{MessageEnvelope, SQSMessage};
use mykobo_rs::message_bus::{InstructionType, MetaData};
use pretty_assertions::assert_eq;

#[test]
fn test_message_serialisation() {
    let meta_data = MetaData::new(
        "TestFunction".to_string(),
        "2025-07-26T18:16:52Z".to_string(),
        "eyw83485yh9wehf89wr9gbsdfgksfg".to_string(),
        "02fb1ad1-103e-49a3-bdcc-4fba95f0f573".to_string(),
        Some(InstructionType::Payment),
        None,
    )
    .unwrap();

    let message = MessageEnvelope {
        meta_data,
        payload: "Test Payload".to_string(),
    };

    let sqs_message = SQSMessage {
        body: message.to_string(),
        group: None,
    };

    // Parse both as JSON to compare semantically rather than string comparison
    // (field order in JSON doesn't matter semantically)
    let actual: serde_json::Value = serde_json::from_str(&sqs_message.body).unwrap();
    let expected: serde_json::Value = serde_json::from_str(
        r#"{"meta_data":{"source":"TestFunction","created_at":"2025-07-26T18:16:52Z","token":"eyw83485yh9wehf89wr9gbsdfgksfg","idempotency_key":"02fb1ad1-103e-49a3-bdcc-4fba95f0f573","instruction_type":"PAYMENT"},"payload":"Test Payload"}"#,
    )
    .unwrap();

    assert_eq!(actual, expected);
}
