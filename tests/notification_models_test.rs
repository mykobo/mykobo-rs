use mykobo_rs::message_bus::models::notification::{
    CustomerNotificationPayload, NotificationSubject, PlatformNotificationPayload, Severity,
};
use serde_json::json;

#[test]
fn relay_subject_serializes_with_type_tag_first() {
    let s = NotificationSubject::Relay {
        id: "abc".into(),
        source_chain: "stellar".into(),
        destination_chain: "solana".into(),
    };
    let v = serde_json::to_value(&s).unwrap();
    assert_eq!(v, json!({
        "type": "relay",
        "id": "abc",
        "source_chain": "stellar",
        "destination_chain": "solana"
    }));
    // Field order matters for byte-equivalence with mykobo-py.
    let s_str = serde_json::to_string(&s).unwrap();
    assert!(s_str.starts_with("{\"type\":\"relay\""), "type tag must come first, got: {s_str}");
}

#[test]
fn transaction_subject_serializes() {
    let s = NotificationSubject::Transaction { reference: "ref-1".into() };
    assert_eq!(
        serde_json::to_value(&s).unwrap(),
        json!({"type": "transaction", "reference": "ref-1"})
    );
}

#[test]
fn profile_subject_serializes() {
    let s = NotificationSubject::Profile { user_id: "u-1".into() };
    assert_eq!(
        serde_json::to_value(&s).unwrap(),
        json!({"type": "profile", "user_id": "u-1"})
    );
}

#[test]
fn severity_serializes_lowercase_and_orders() {
    assert_eq!(serde_json::to_string(&Severity::Info).unwrap(), "\"info\"");
    assert_eq!(serde_json::to_string(&Severity::Warning).unwrap(), "\"warning\"");
    assert_eq!(serde_json::to_string(&Severity::Critical).unwrap(), "\"critical\"");
    assert!(Severity::Info < Severity::Warning);
    assert!(Severity::Warning < Severity::Critical);
}

#[test]
fn customer_payload_has_no_discriminator_field() {
    let p = CustomerNotificationPayload {
        subject: NotificationSubject::Relay {
            id: "abc".into(),
            source_chain: "stellar".into(),
            destination_chain: "solana".into(),
        },
        data: json!({"email": "u@e.com"}),
    };
    let v = serde_json::to_value(&p).unwrap();
    assert!(v.get("type").is_none(), "CustomerNotificationPayload must not emit a type discriminator");
    assert_eq!(v["subject"]["type"], "relay");
    assert_eq!(v["data"]["email"], "u@e.com");
    // Field order: subject before data.
    let s = serde_json::to_string(&p).unwrap();
    assert!(s.starts_with("{\"subject\""), "subject must come first, got: {s}");
}

#[test]
fn platform_payload_with_subject_field_order() {
    let p = PlatformNotificationPayload {
        severity: Severity::Warning,
        data: json!({"k": "v"}),
        subject: Some("relay:abc".into()),
    };
    let s = serde_json::to_string(&p).unwrap();
    // Order must be severity, data, subject (matching Python declaration order).
    assert_eq!(
        s,
        r#"{"severity":"warning","data":{"k":"v"},"subject":"relay:abc"}"#
    );
}

#[test]
fn platform_payload_without_subject_drops_field() {
    let p = PlatformNotificationPayload {
        severity: Severity::Critical,
        data: json!({"k": "v"}),
        subject: None,
    };
    let v = serde_json::to_value(&p).unwrap();
    assert!(v.get("subject").is_none(), "subject: None must drop from JSON (skip_serializing_none)");
}
