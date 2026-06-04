use mykobo_rs::message_bus::models::base::EventType;
use mykobo_rs::notification_contract::{Audience, Entry, REGISTRY, Severity};
use serde_json::json;

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

#[test]
fn mint_completed_is_customer_notification() {
    assert!(REGISTRY.is_notification(EventType::MintCompleted));
    assert_eq!(REGISTRY.audience_of(EventType::MintCompleted), Some(Audience::Customer));
    assert_eq!(REGISTRY.severity_of(EventType::MintCompleted), None);
}

#[test]
fn burn_completed_is_customer_notification() {
    assert!(REGISTRY.is_notification(EventType::BurnCompleted));
    assert_eq!(REGISTRY.audience_of(EventType::BurnCompleted), Some(Audience::Customer));
    assert_eq!(REGISTRY.severity_of(EventType::BurnCompleted), None);
}

#[test]
fn mint_held_is_customer_notification() {
    assert!(REGISTRY.is_notification(EventType::MintHeld));
    assert_eq!(REGISTRY.audience_of(EventType::MintHeld), Some(Audience::Customer));
    assert_eq!(REGISTRY.severity_of(EventType::MintHeld), None);
}

#[test]
fn burn_held_is_customer_notification() {
    assert!(REGISTRY.is_notification(EventType::BurnHeld));
    assert_eq!(REGISTRY.audience_of(EventType::BurnHeld), Some(Audience::Customer));
    assert_eq!(REGISTRY.severity_of(EventType::BurnHeld), None);
}

#[test]
fn mint_held_alert_is_warning_platform() {
    assert_eq!(REGISTRY.audience_of(EventType::MintHeldAlert), Some(Audience::Platform));
    assert_eq!(REGISTRY.severity_of(EventType::MintHeldAlert), Some(Severity::Warning));
}

#[test]
fn burn_held_alert_is_warning_platform() {
    assert_eq!(REGISTRY.audience_of(EventType::BurnHeldAlert), Some(Audience::Platform));
    assert_eq!(REGISTRY.severity_of(EventType::BurnHeldAlert), Some(Severity::Warning));
}

#[test]
fn customer_notify_failed_is_warning_platform() {
    assert_eq!(REGISTRY.audience_of(EventType::CustomerNotifyFailed), Some(Audience::Platform));
    assert_eq!(REGISTRY.severity_of(EventType::CustomerNotifyFailed), Some(Severity::Warning));
}

#[test]
fn mint_info_is_info_platform() {
    assert_eq!(REGISTRY.audience_of(EventType::MintInfo), Some(Audience::Platform));
    assert_eq!(REGISTRY.severity_of(EventType::MintInfo), Some(Severity::Info));
}

#[test]
fn burn_info_is_info_platform() {
    assert_eq!(REGISTRY.audience_of(EventType::BurnInfo), Some(Audience::Platform));
    assert_eq!(REGISTRY.severity_of(EventType::BurnInfo), Some(Severity::Info));
}

#[test]
fn transaction_failed_alert_is_platform_critical() {
    assert!(REGISTRY.is_notification(EventType::TransactionFailedAlert));
    assert_eq!(REGISTRY.audience_of(EventType::TransactionFailedAlert), Some(Audience::Platform));
    assert_eq!(REGISTRY.severity_of(EventType::TransactionFailedAlert), Some(Severity::Critical));
}

#[test]
fn transaction_held_alert_is_platform_warning() {
    assert!(REGISTRY.is_notification(EventType::TransactionHeldAlert));
    assert_eq!(REGISTRY.audience_of(EventType::TransactionHeldAlert), Some(Audience::Platform));
    assert_eq!(REGISTRY.severity_of(EventType::TransactionHeldAlert), Some(Severity::Warning));
}

#[test]
fn transaction_status_update_failed_fires_failed_alert() {
    let payload = json!({"status": "FAILED"});
    let fires = REGISTRY.notifications_for(EventType::TransactionStatusUpdate, &payload);
    assert_eq!(fires, vec![EventType::TransactionFailedAlert]);
}

#[test]
fn transaction_status_update_held_fires_held_alert() {
    let payload = json!({"status": "HELD"});
    let fires = REGISTRY.notifications_for(EventType::TransactionStatusUpdate, &payload);
    assert_eq!(fires, vec![EventType::TransactionHeldAlert]);
}

#[test]
fn transaction_status_update_other_status_fires_nothing() {
    let payload = json!({"status": "FUNDS_RECEIVED"});
    let fires = REGISTRY.notifications_for(EventType::TransactionStatusUpdate, &payload);
    assert_eq!(fires, Vec::<EventType>::new());
}
