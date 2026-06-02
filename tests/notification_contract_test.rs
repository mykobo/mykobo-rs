use mykobo_rs::message_bus::models::base::EventType;
use mykobo_rs::notification_contract::{Audience, Entry, REGISTRY, Severity};

#[test]
fn registry_loads() {
    let _ = &*REGISTRY;
}

#[test]
fn every_enum_variant_classified() {
    for ev in EventType::ALL {
        assert!(REGISTRY.entries.contains_key(ev), "missing entry: {:?}", ev);
    }
}

#[test]
fn relay_initiated_is_customer_notification() {
    assert!(REGISTRY.is_notification(EventType::RelayInitiated));
    assert_eq!(REGISTRY.audience_of(EventType::RelayInitiated), Some(Audience::Customer));
}

#[test]
fn relay_failed_is_critical_platform() {
    assert_eq!(REGISTRY.audience_of(EventType::RelayFailed), Some(Audience::Platform));
    assert_eq!(REGISTRY.severity_of(EventType::RelayFailed), Some(Severity::Critical));
}

#[test]
fn new_transaction_is_domain() {
    assert!(!REGISTRY.is_notification(EventType::NewTransaction));
    let payload = serde_json::json!({});
    assert!(REGISTRY.notifications_for(EventType::NewTransaction, &payload).is_empty());
}

#[test]
fn kyc_event_has_reason() {
    match REGISTRY.entries.get(&EventType::KycEvent) {
        Some(Entry::Domain { notifies, reason }) => {
            assert!(notifies.is_empty());
            assert!(reason.as_ref().map(|r| r.len() > 10).unwrap_or(false));
        }
        other => panic!("expected domain kyc entry, got {:?}", other),
    }
}
