use crate::message_bus::models::base::EventType;

pub const NOTIFICATION_EVENTS: &[EventType] = &[
    EventType::RelayInitiated,
    EventType::RelayCompleted,
    EventType::RelayOnboarded,
    EventType::RelayStuckDepositing,
    EventType::RelayStuckBridging,
    EventType::RelayStuckForwarding,
    EventType::RelayForwardingFailed,
    EventType::CircleApi5xxBurst,
    EventType::WebhookReprocessorBacklog,
];
