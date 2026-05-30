use chrono::{Duration, TimeZone, Utc};
use mykobo_rs::message_bus::models::base::EventType;
use mykobo_rs::notification::IdempotencyKey;

#[test]
fn for_event_with_subject() {
    let k = IdempotencyKey::for_event("circle", EventType::RelayInitiated, "abc-123");
    assert_eq!(k, "circle:relay_initiated:abc-123");
}

#[test]
fn for_event_screaming_snake_lowercased() {
    let k = IdempotencyKey::for_event("circle", EventType::CircleApi5xxBurst, "x");
    assert_eq!(k, "circle:circle_api_5xx_burst:x");
}

#[test]
fn for_bucket_with_subject_hour_window() {
    let now = Utc.with_ymd_and_hms(2026, 5, 30, 12, 34, 0).unwrap();
    let k = IdempotencyKey::for_bucket(
        "circle",
        EventType::RelayStuckDepositing,
        Some("abc-123"),
        Duration::hours(1),
        now,
    );
    assert_eq!(k, "circle:relay_stuck_depositing:abc-123:2026-05-30T12:00:00+00:00");
}

#[test]
fn for_bucket_without_subject_skips_segment() {
    let now = Utc.with_ymd_and_hms(2026, 5, 30, 12, 30, 0).unwrap();
    let k = IdempotencyKey::for_bucket(
        "circle",
        EventType::CircleApi5xxBurst,
        None,
        Duration::minutes(15),
        now,
    );
    assert_eq!(k, "circle:circle_api_5xx_burst:2026-05-30T12:30:00+00:00");
}

#[test]
fn for_bucket_collapses_within_window() {
    let base = Utc.with_ymd_and_hms(2026, 5, 30, 12, 0, 0).unwrap();
    let k1 = IdempotencyKey::for_bucket(
        "circle",
        EventType::RelayStuckDepositing,
        Some("r-1"),
        Duration::hours(1),
        base + Duration::minutes(5),
    );
    let k2 = IdempotencyKey::for_bucket(
        "circle",
        EventType::RelayStuckDepositing,
        Some("r-1"),
        Duration::hours(1),
        base + Duration::minutes(55),
    );
    assert_eq!(k1, k2);
}

#[test]
fn for_bucket_distinct_across_windows() {
    let base = Utc.with_ymd_and_hms(2026, 5, 30, 12, 0, 0).unwrap();
    let k1 = IdempotencyKey::for_bucket(
        "circle",
        EventType::RelayStuckDepositing,
        Some("r-1"),
        Duration::hours(1),
        base + Duration::minutes(30),
    );
    let k2 = IdempotencyKey::for_bucket(
        "circle",
        EventType::RelayStuckDepositing,
        Some("r-1"),
        Duration::hours(1),
        base + Duration::hours(1) + Duration::minutes(30),
    );
    assert_ne!(k1, k2);
}

#[test]
#[should_panic(expected = "window must be a positive whole-second duration")]
fn for_bucket_rejects_zero_window() {
    IdempotencyKey::for_bucket(
        "circle",
        EventType::RelayStuckDepositing,
        Some("r-1"),
        Duration::zero(),
        Utc.with_ymd_and_hms(2026, 5, 30, 12, 0, 0).unwrap(),
    );
}

#[test]
#[should_panic(expected = "window must be a positive whole-second duration")]
fn for_bucket_rejects_subsecond_window() {
    IdempotencyKey::for_bucket(
        "circle",
        EventType::RelayStuckDepositing,
        Some("r-1"),
        Duration::milliseconds(500),
        Utc.with_ymd_and_hms(2026, 5, 30, 12, 0, 0).unwrap(),
    );
}
