use mykobo_rs::message_bus::models::base::EventType;

#[test]
fn new_relay_stuck_variants_serialize() {
    assert_eq!(serde_json::to_string(&EventType::RelayStuckDepositing).unwrap(), "\"RELAY_STUCK_DEPOSITING\"");
    assert_eq!(serde_json::to_string(&EventType::RelayStuckBridging).unwrap(), "\"RELAY_STUCK_BRIDGING\"");
    assert_eq!(serde_json::to_string(&EventType::RelayStuckForwarding).unwrap(), "\"RELAY_STUCK_FORWARDING\"");
}

#[test]
fn relay_failed_serializes() {
    assert_eq!(serde_json::to_string(&EventType::RelayFailed).unwrap(), "\"RELAY_FAILED\"");
}

#[test]
fn circle_health_variants_serialize() {
    assert_eq!(serde_json::to_string(&EventType::CircleApi5xxBurst).unwrap(), "\"CIRCLE_API_5XX_BURST\"");
    assert_eq!(serde_json::to_string(&EventType::WebhookReprocessorBacklog).unwrap(), "\"WEBHOOK_REPROCESSOR_BACKLOG\"");
}

#[test]
fn deserializes_from_screaming_snake() {
    let ev: EventType = serde_json::from_str("\"RELAY_STUCK_DEPOSITING\"").unwrap();
    assert!(matches!(ev, EventType::RelayStuckDepositing));
}
