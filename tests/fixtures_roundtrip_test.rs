use std::fs;
use std::path::PathBuf;

use mykobo_rs::message_bus::models::base::EventType;
use mykobo_rs::message_bus::models::message::{MessageBusMessage, MetaData, Payload};
use mykobo_rs::message_bus::models::notification::{
    CustomerNotificationPayload, NotificationSubject, PlatformNotificationPayload, Severity,
};
use serde_json::json;

const FIXED_CREATED_AT: &str = "2026-05-30T12:00:00Z";
const FIXED_TOKEN: &str = "test-service-token";

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("notification")
}

fn build_customer(event: EventType, idem: &str, data: serde_json::Value) -> MessageBusMessage {
    let payload = Payload::CustomerNotification(CustomerNotificationPayload {
        subject: NotificationSubject::Relay {
            id: "abc-123".into(),
            source_chain: "stellar".into(),
            destination_chain: "solana".into(),
        },
        data,
    });
    let meta = MetaData::new(
        "circle".into(),
        FIXED_CREATED_AT.into(),
        FIXED_TOKEN.into(),
        idem.into(),
        None,
        Some(event),
        None,
    )
    .unwrap();
    MessageBusMessage::new(meta, payload).unwrap()
}

fn build_platform(
    event: EventType,
    idem: &str,
    severity: Severity,
    subject: &str,
    data: serde_json::Value,
) -> MessageBusMessage {
    let payload = Payload::PlatformNotification(PlatformNotificationPayload {
        severity,
        data,
        subject: Some(subject.into()),
    });
    let meta = MetaData::new(
        "circle".into(),
        FIXED_CREATED_AT.into(),
        FIXED_TOKEN.into(),
        idem.into(),
        None,
        Some(event),
        None,
    )
    .unwrap();
    MessageBusMessage::new(meta, payload).unwrap()
}

fn assert_fixture_matches(filename: &str, msg: &MessageBusMessage) {
    let reference = fs::read_to_string(fixtures_dir().join(filename))
        .unwrap_or_else(|e| panic!("failed to read fixture {filename}: {e}"));
    let generated = format!("{}\n", serde_json::to_string_pretty(msg).unwrap());
    assert_eq!(generated, reference, "byte drift in {filename}");
}

#[test]
fn customer_relay_initiated_byte_equal() {
    let data = json!({
        "email": "user@example.com",
        "first_name": "Ada",
        "amount": "100.00",
        "currency": "USDC"
    });
    let msg = build_customer(EventType::RelayInitiated, "circle:relay_initiated:abc-123", data);
    assert_fixture_matches("customer_relay_initiated.json", &msg);
}

#[test]
fn customer_relay_completed_byte_equal() {
    let data = json!({
        "email": "user@example.com",
        "first_name": "Ada",
        "amount": "100.00",
        "currency": "USDC"
    });
    let msg = build_customer(EventType::RelayCompleted, "circle:relay_completed:abc-123", data);
    assert_fixture_matches("customer_relay_completed.json", &msg);
}

#[test]
fn customer_relay_onboarded_byte_equal() {
    let data = json!({
        "email": "user@example.com",
        "first_name": "Ada",
        "amount": "100.00",
        "currency": "USDC"
    });
    let msg = build_customer(EventType::RelayOnboarded, "circle:relay_onboarded:abc-123", data);
    assert_fixture_matches("customer_relay_onboarded.json", &msg);
}

#[test]
fn platform_relay_stuck_depositing_byte_equal() {
    let data = json!({
        "relay_id": "abc-123",
        "stuck_for_minutes": 22,
        "threshold_minutes": 15,
        "source_chain": "stellar"
    });
    let msg = build_platform(
        EventType::RelayStuckDepositing,
        "circle:relay_stuck_depositing:abc-123:2026-05-30T12:00:00+00:00",
        Severity::Warning,
        "relay:abc-123",
        data,
    );
    assert_fixture_matches("platform_relay_stuck_depositing.json", &msg);
}

#[test]
fn platform_relay_forwarding_failed_byte_equal() {
    let data = json!({
        "relay_id": "abc-123",
        "attempts": 5,
        "last_error": "timeout"
    });
    let msg = build_platform(
        EventType::RelayForwardingFailed,
        "circle:relay_forwarding_failed:abc-123",
        Severity::Critical,
        "relay:abc-123",
        data,
    );
    assert_fixture_matches("platform_relay_forwarding_failed.json", &msg);
}

#[test]
fn platform_circle_api_5xx_burst_byte_equal() {
    let data = json!({
        "errors_in_window": 7,
        "threshold": 5,
        "window_minutes": 15
    });
    let msg = build_platform(
        EventType::CircleApi5xxBurst,
        "circle:circle_api_5xx_burst:2026-05-30T12:30:00+00:00",
        Severity::Warning,
        "service:circle:circle_api",
        data,
    );
    assert_fixture_matches("platform_circle_api_5xx_burst.json", &msg);
}

#[test]
fn platform_webhook_reprocessor_backlog_byte_equal() {
    let data = json!({
        "queue_depth": 142,
        "threshold": 100,
        "window_minutes": 60
    });
    let msg = build_platform(
        EventType::WebhookReprocessorBacklog,
        "circle:webhook_reprocessor_backlog:2026-05-30T12:00:00+00:00",
        Severity::Warning,
        "service:circle:webhook_reprocessor",
        data,
    );
    assert_fixture_matches("platform_webhook_reprocessor_backlog.json", &msg);
}
