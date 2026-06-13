use mykobo_rs::message_bus::models::base::EventType;
use mykobo_rs::notification::NOTIFICATION_EVENTS;

#[test]
fn includes_relay_lifecycle() {
    assert!(NOTIFICATION_EVENTS.contains(&EventType::RelayInitiated));
    assert!(NOTIFICATION_EVENTS.contains(&EventType::RelayCompleted));
    assert!(NOTIFICATION_EVENTS.contains(&EventType::RelayOnboarded));
}

#[test]
fn includes_relay_stuck() {
    assert!(NOTIFICATION_EVENTS.contains(&EventType::RelayStuckDepositing));
    assert!(NOTIFICATION_EVENTS.contains(&EventType::RelayStuckBridging));
    assert!(NOTIFICATION_EVENTS.contains(&EventType::RelayStuckForwarding));
}

#[test]
fn includes_relay_failed() {
    assert!(NOTIFICATION_EVENTS.contains(&EventType::RelayFailed));
}

#[test]
fn includes_circle_health() {
    assert!(NOTIFICATION_EVENTS.contains(&EventType::CircleApi5xxBurst));
    assert!(NOTIFICATION_EVENTS.contains(&EventType::WebhookReprocessorBacklog));
}

#[test]
fn count_is_nine() {
    assert_eq!(NOTIFICATION_EVENTS.len(), 9);
}
