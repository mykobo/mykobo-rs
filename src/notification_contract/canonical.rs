//! Stable JSON projection of a Registry used for cross-library byte equivalence.

use serde_json::{json, Map, Value};

use crate::notification_contract::predicates::{Literal, Predicate};
use crate::notification_contract::registry::{Audience, Entry, Registry, Severity, REGISTRY_VERSION};

pub fn to_canonical_value(reg: &Registry) -> Value {
    let mut variants = Map::new();
    for (event, entry) in &reg.entries {
        let value = match entry {
            Entry::Domain { notifies, reason } => {
                let mut notifies_json = Vec::new();
                for rule in notifies {
                    notifies_json.push(json!({
                        "when": rule.when.as_ref().map(predicate_to_json).unwrap_or(Value::Null),
                        "fires": rule.fires.iter().map(|e| e.as_str()).collect::<Vec<_>>(),
                    }));
                }
                json!({
                    "kind": "domain",
                    "notifies": notifies_json,
                    "reason": reason.clone().map(Value::String).unwrap_or(Value::Null),
                })
            }
            Entry::Notification { audience, severity } => {
                let aud = match audience {
                    Audience::Customer => "customer",
                    Audience::Platform => "platform",
                };
                let sev = severity.map(|s| match s {
                    Severity::Info => "info",
                    Severity::Warning => "warning",
                    Severity::Critical => "critical",
                });
                json!({
                    "kind": "notification",
                    "audience": aud,
                    "severity": sev.map(Value::from).unwrap_or(Value::Null),
                })
            }
        };
        variants.insert(event.as_str().to_string(), value);
    }
    json!({
        "version": REGISTRY_VERSION,
        "variants": Value::Object(variants),
    })
}

pub fn to_canonical_json(reg: &Registry) -> String {
    let value = to_canonical_value(reg);
    let sorted = canonicalize(&value);
    let mut out = serde_json::to_string_pretty(&sorted).expect("serialize");
    out.push('\n');
    out
}

fn canonicalize(v: &Value) -> Value {
    match v {
        Value::Object(m) => {
            let mut keys: Vec<&String> = m.keys().collect();
            keys.sort();
            let mut out = Map::new();
            for k in keys {
                out.insert(k.clone(), canonicalize(&m[k]));
            }
            Value::Object(out)
        }
        Value::Array(a) => Value::Array(a.iter().map(canonicalize).collect()),
        other => other.clone(),
    }
}

fn predicate_to_json(p: &Predicate) -> Value {
    match p {
        Predicate::Equals { field, value } => json!({"op": "eq", "field": field, "value": literal_to_json(value)}),
        Predicate::NotEquals { field, value } => json!({"op": "ne", "field": field, "value": literal_to_json(value)}),
        Predicate::In { field, values } => json!({"op": "in", "field": field, "values": values.iter().map(literal_to_json).collect::<Vec<_>>()}),
        Predicate::NotIn { field, values } => json!({"op": "not_in", "field": field, "values": values.iter().map(literal_to_json).collect::<Vec<_>>()}),
        Predicate::And(l, r) => json!({"op": "and", "left": predicate_to_json(l), "right": predicate_to_json(r)}),
        Predicate::Or(l, r) => json!({"op": "or", "left": predicate_to_json(l), "right": predicate_to_json(r)}),
    }
}

fn literal_to_json(l: &Literal) -> Value {
    match l {
        Literal::Str(s) => Value::String(s.clone()),
        Literal::Int(i) => Value::Number((*i).into()),
    }
}
