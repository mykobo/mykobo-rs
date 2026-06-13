//! Predicate AST for the notification registry's `when:` rules.

use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Predicate {
    Equals { field: String, value: Literal },
    NotEquals { field: String, value: Literal },
    In { field: String, values: Vec<Literal> },
    NotIn { field: String, values: Vec<Literal> },
    And(Box<Predicate>, Box<Predicate>),
    Or(Box<Predicate>, Box<Predicate>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Str(String),
    Int(i64),
}

#[derive(Debug, thiserror::Error)]
pub enum MatchError {
    #[error("predicate references missing field {0:?}")]
    MissingField(String),
}

impl Predicate {
    pub fn matches(&self, payload: &Value) -> Result<bool, MatchError> {
        match self {
            Self::Equals { field, value } => {
                let v = lookup(payload, field)?;
                Ok(literal_eq(value, v))
            }
            Self::NotEquals { field, value } => {
                let v = lookup(payload, field)?;
                Ok(!literal_eq(value, v))
            }
            Self::In { field, values } => {
                let v = lookup(payload, field)?;
                Ok(values.iter().any(|lit| literal_eq(lit, v)))
            }
            Self::NotIn { field, values } => {
                let v = lookup(payload, field)?;
                Ok(!values.iter().any(|lit| literal_eq(lit, v)))
            }
            Self::And(l, r) => Ok(l.matches(payload)? && r.matches(payload)?),
            Self::Or(l, r) => Ok(l.matches(payload)? || r.matches(payload)?),
        }
    }
}

fn lookup<'a>(payload: &'a Value, field: &str) -> Result<&'a Value, MatchError> {
    payload
        .as_object()
        .and_then(|m| m.get(field))
        .ok_or_else(|| MatchError::MissingField(field.to_string()))
}

fn literal_eq(lit: &Literal, v: &Value) -> bool {
    match (lit, v) {
        (Literal::Str(s), Value::String(vs)) => s == vs,
        (Literal::Int(i), Value::Number(vn)) => vn.as_i64() == Some(*i),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn equals_matches() {
        let p = Predicate::Equals {
            field: "status".into(),
            value: Literal::Str("FUNDS_RECEIVED".into()),
        };
        assert_eq!(p.matches(&json!({"status": "FUNDS_RECEIVED"})).unwrap(), true);
        assert_eq!(p.matches(&json!({"status": "PENDING"})).unwrap(), false);
    }

    #[test]
    fn equals_missing_field_errors() {
        let p = Predicate::Equals {
            field: "status".into(),
            value: Literal::Str("X".into()),
        };
        assert!(matches!(p.matches(&json!({})), Err(MatchError::MissingField(_))));
    }

    #[test]
    fn in_matches() {
        let p = Predicate::In {
            field: "status".into(),
            values: vec![Literal::Str("A".into()), Literal::Str("B".into())],
        };
        assert!(p.matches(&json!({"status": "A"})).unwrap());
        assert!(!p.matches(&json!({"status": "C"})).unwrap());
    }

    #[test]
    fn and_short_circuits() {
        let p = Predicate::And(
            Box::new(Predicate::Equals { field: "x".into(), value: Literal::Str("1".into()) }),
            Box::new(Predicate::Equals { field: "y".into(), value: Literal::Str("2".into()) }),
        );
        assert!(p.matches(&json!({"x": "1", "y": "2"})).unwrap());
        assert!(!p.matches(&json!({"x": "0", "y": "2"})).unwrap());
    }

    #[test]
    fn or_matches_either() {
        let p = Predicate::Or(
            Box::new(Predicate::Equals { field: "s".into(), value: Literal::Str("A".into()) }),
            Box::new(Predicate::Equals { field: "s".into(), value: Literal::Str("B".into()) }),
        );
        assert!(p.matches(&json!({"s": "A"})).unwrap());
        assert!(p.matches(&json!({"s": "B"})).unwrap());
        assert!(!p.matches(&json!({"s": "C"})).unwrap());
    }
}
