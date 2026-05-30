use crate::message_bus::models::base::EventType;
use chrono::{DateTime, Duration, TimeZone, Utc};

pub struct IdempotencyKey;

impl IdempotencyKey {
    /// Build a deterministic idempotency key for an event with a subject.
    ///
    /// Format: `{producer}:{event_lower}:{subject_id}`
    ///
    /// The event segment is the serialized EventType value lowercased.
    pub fn for_event(producer: &str, event: EventType, subject_id: &str) -> String {
        let event_lower = Self::event_to_lower(event);
        format!("{}:{}:{}", producer, event_lower, subject_id)
    }

    /// Build a deterministic idempotency key for an event bucketed by time window.
    ///
    /// This is useful for rate-limiting or batching notifications that occur multiple times
    /// within a fixed window. Events at `now`, `now + 5 min`, and `now + 55 min` all collapse
    /// to the same bucket within a 1-hour window.
    ///
    /// Format:
    /// - With subject: `{producer}:{event_lower}:{subject_id}:{bucket}`
    /// - Without subject: `{producer}:{event_lower}:{bucket}`
    ///
    /// The bucket is the floored UTC timestamp rendered as `YYYY-MM-DDTHH:MM:SS+00:00`.
    ///
    /// # Panics
    ///
    /// Panics if `window` has a non-positive duration (in whole seconds).
    pub fn for_bucket(
        producer: &str,
        event: EventType,
        subject_id: Option<&str>,
        window: Duration,
        now: DateTime<Utc>,
    ) -> String {
        let window_secs = window.num_seconds();
        assert!(
            window_secs > 0,
            "window must be a positive whole-second duration"
        );

        // Compute floored bucket timestamp
        let epoch = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();
        let delta = now.signed_duration_since(epoch);
        let delta_secs = delta.num_seconds();
        let floored_secs = (delta_secs / window_secs) * window_secs;
        let bucket = epoch + Duration::seconds(floored_secs);
        let bucket_str = bucket.format("%Y-%m-%dT%H:%M:%S+00:00").to_string();

        let event_lower = Self::event_to_lower(event);

        match subject_id {
            Some(id) => format!("{}:{}:{}:{}", producer, event_lower, id, bucket_str),
            None => format!("{}:{}:{}", producer, event_lower, bucket_str),
        }
    }

    fn event_to_lower(event: EventType) -> String {
        serde_json::to_value(event)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_lowercase()))
            .unwrap_or_default()
    }
}
