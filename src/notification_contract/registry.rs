//! Registry: typed model, loader, query helpers.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::Value;

use crate::message_bus::models::base::EventType;
use crate::notification_contract::parser::{parse_predicate, ParseError};
use crate::notification_contract::predicates::Predicate;

pub const REGISTRY_VERSION: u32 = 1;

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("registry: {0}")]
    Msg(String),
    #[error("registry parse: {0}")]
    Parse(#[from] ParseError),
    #[error("registry yaml: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("registry io: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariantKind { Domain, Notification }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Audience { Customer, Platform }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity { Info, Warning, Critical }

#[derive(Debug, Clone, PartialEq)]
pub struct NotificationRule {
    pub when: Option<Predicate>,
    pub fires: Vec<EventType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Entry {
    Domain { notifies: Vec<NotificationRule>, reason: Option<String> },
    Notification { audience: Audience, severity: Option<Severity> },
}

impl Entry {
    pub fn kind(&self) -> VariantKind {
        match self {
            Self::Domain { .. } => VariantKind::Domain,
            Self::Notification { .. } => VariantKind::Notification,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Registry {
    pub entries: BTreeMap<EventType, Entry>,
}

impl Registry {
    pub fn from_str(yaml: &str) -> Result<Self, RegistryError> {
        let raw: RawRegistry = serde_yaml::from_str(yaml)?;
        if raw.version != REGISTRY_VERSION {
            return Err(RegistryError::Msg(format!(
                "unsupported registry version {}; expected {REGISTRY_VERSION}", raw.version
            )));
        }

        // Phase 1: parse variants.
        let mut parsed = BTreeMap::new();
        for (name, raw_entry) in &raw.variants {
            let event = EventType::try_from(name.as_str()).map_err(|_| {
                RegistryError::Msg(format!("YAML key {name:?} does not resolve to an EventType variant"))
            })?;
            parsed.insert(event, parse_entry(&event, raw_entry)?);
        }

        // Phase 2: enum coverage.
        let missing: Vec<&'static str> = EventType::ALL.iter()
            .filter(|e| !parsed.contains_key(*e))
            .map(|e| e.as_str())
            .collect();
        if !missing.is_empty() {
            return Err(RegistryError::Msg(format!(
                "EventType variants missing from registry: {:?}", missing
            )));
        }

        // Phase 3: fires targets resolve to notification entries.
        for (event, entry) in &parsed {
            if let Entry::Domain { notifies, .. } = entry {
                for rule in notifies {
                    for target in &rule.fires {
                        match parsed.get(target) {
                            Some(Entry::Notification { .. }) => {}
                            _ => return Err(RegistryError::Msg(format!(
                                "{}: fires target {} must be a kind:notification entry",
                                event_str(event), event_str(target),
                            ))),
                        }
                    }
                }
            }
        }

        Ok(Self { entries: parsed })
    }

    pub fn load_from_path(p: &Path) -> Result<Self, RegistryError> {
        Self::from_str(&std::fs::read_to_string(p)?)
    }

    pub fn is_notification(&self, e: EventType) -> bool {
        matches!(self.entries.get(&e), Some(Entry::Notification { .. }))
    }

    pub fn audience_of(&self, e: EventType) -> Option<Audience> {
        match self.entries.get(&e) {
            Some(Entry::Notification { audience, .. }) => Some(*audience),
            _ => None,
        }
    }

    pub fn severity_of(&self, e: EventType) -> Option<Severity> {
        match self.entries.get(&e) {
            Some(Entry::Notification { severity, .. }) => *severity,
            _ => None,
        }
    }

    pub fn notifications_for(&self, e: EventType, payload: &Value) -> Vec<EventType> {
        match self.entries.get(&e) {
            Some(Entry::Domain { notifies, .. }) => {
                let mut out = Vec::new();
                for rule in notifies {
                    let matches = match &rule.when {
                        None => true,
                        Some(p) => p.matches(payload).unwrap_or(false),
                    };
                    if matches { out.extend(rule.fires.iter().copied()); }
                }
                out
            }
            _ => Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct RawRegistry {
    version: u32,
    variants: BTreeMap<String, RawEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawEntry {
    kind: String,
    #[serde(default)]
    notifies: Option<Vec<RawRule>>,
    #[serde(default)]
    reason: Option<String>,
    #[serde(default)]
    audience: Option<String>,
    #[serde(default)]
    severity: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawRule {
    #[serde(default)]
    when: Option<String>,
    fires: Vec<String>,
}

fn parse_entry(event: &EventType, raw: &RawEntry) -> Result<Entry, RegistryError> {
    match raw.kind.as_str() {
        "domain" => parse_domain(event, raw),
        "notification" => parse_notification(event, raw),
        other => Err(RegistryError::Msg(format!("{}: invalid kind {other:?}", event_str(event)))),
    }
}

fn parse_domain(event: &EventType, raw: &RawEntry) -> Result<Entry, RegistryError> {
    let rules_raw = raw.notifies.as_ref().ok_or_else(|| RegistryError::Msg(
        format!("{}: domain entry must declare `notifies`", event_str(event))
    ))?;
    let mut rules = Vec::new();
    for (idx, item) in rules_raw.iter().enumerate() {
        if item.fires.is_empty() {
            return Err(RegistryError::Msg(format!(
                "{}: notifies[{idx}] must declare a non-empty `fires` list", event_str(event)
            )));
        }
        let fires: Result<Vec<EventType>, _> = item.fires.iter().map(|n| {
            EventType::try_from(n.as_str()).map_err(|_| RegistryError::Msg(format!(
                "{}: notifies[{idx}] fires unknown variant {n:?}", event_str(event)
            )))
        }).collect();
        let when = item.when.as_deref().map(parse_predicate).transpose()?;
        rules.push(NotificationRule { when, fires: fires? });
    }
    if rules.is_empty() && raw.reason.as_deref().unwrap_or("").is_empty() {
        return Err(RegistryError::Msg(format!(
            "{}: empty notifies requires a non-empty `reason`", event_str(event)
        )));
    }
    if !rules.is_empty() && raw.reason.is_some() {
        return Err(RegistryError::Msg(format!(
            "{}: `reason` is only valid when notifies is empty", event_str(event)
        )));
    }
    Ok(Entry::Domain { notifies: rules, reason: raw.reason.clone() })
}

fn parse_notification(event: &EventType, raw: &RawEntry) -> Result<Entry, RegistryError> {
    let audience = match raw.audience.as_deref() {
        Some("customer") => Audience::Customer,
        Some("platform") => Audience::Platform,
        Some(other) => return Err(RegistryError::Msg(format!(
            "{}: invalid audience {other:?}", event_str(event)
        ))),
        None => return Err(RegistryError::Msg(format!(
            "{}: notification entry missing audience", event_str(event)
        ))),
    };
    let severity = match (audience, raw.severity.as_deref()) {
        (Audience::Platform, Some("info")) => Some(Severity::Info),
        (Audience::Platform, Some("warning")) => Some(Severity::Warning),
        (Audience::Platform, Some("critical")) => Some(Severity::Critical),
        (Audience::Platform, None) => return Err(RegistryError::Msg(format!(
            "{}: platform-audience notification requires a `severity`", event_str(event)
        ))),
        (Audience::Customer, None) => None,
        (Audience::Customer, Some(_)) => return Err(RegistryError::Msg(format!(
            "{}: customer-audience notification must not declare `severity`", event_str(event)
        ))),
        (_, Some(other)) => return Err(RegistryError::Msg(format!(
            "{}: invalid severity {other:?}", event_str(event)
        ))),
    };
    Ok(Entry::Notification { audience, severity })
}

fn event_str(e: &EventType) -> &'static str { e.as_str() }

pub static REGISTRY: Lazy<Registry> = Lazy::new(|| {
    let path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/notification_contract/registry.yaml");
    Registry::load_from_path(&path).unwrap_or_else(|e| {
        panic!("failed to load notification_contract registry: {e}");
    })
});
