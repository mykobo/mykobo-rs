/// Notification-specific types that plug into the MessageBusMessage envelope.
///
/// Mirrors `mykobo_py.message_bus.models.notification`.
/// Severity, Subjects, and NotificationPayloads are registered against the
/// notification EventType variants in the Payload enum (message.rs).
use serde::{Deserialize, Serialize};

/// Importance gradient for PlatformNotifications.
///
/// Serializes as lowercase strings: `"info"`, `"warning"`, `"critical"`.
/// Ordered: Info < Warning < Critical.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

/// Discriminated subject union for CustomerNotificationPayload.
///
/// Serialized with an internal `type` tag (lowercase variant name).
/// Field order within each variant matches the Python declaration order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum NotificationSubject {
    Relay {
        id: String,
        source_chain: String,
        destination_chain: String,
    },
    Transaction {
        reference: String,
    },
    Profile {
        user_id: String,
    },
}

/// Payload for customer-directed notifications (email-by-default).
///
/// Carries a typed subject reference and a fully-rendered template-data dict.
/// Field order: `subject` then `data` (matching Python declaration order).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerNotificationPayload {
    pub subject: NotificationSubject,
    pub data: serde_json::Value,
}

/// Payload for admin-directed notifications (Slack-by-default).
///
/// `severity` grades importance; `subject` is free-form (e.g. `"relay:abc-123"`);
/// `data` is fully rendered.
/// Field order: `severity`, `data`, `subject` (matching Python declaration order).
/// `subject: None` is omitted from JSON (mirrors Python `exclude_none=True`).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlatformNotificationPayload {
    pub severity: Severity,
    pub data: serde_json::Value,
    pub subject: Option<String>,
}
