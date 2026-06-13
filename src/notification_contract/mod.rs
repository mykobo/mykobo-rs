//! Producer-intended notification registry.

pub mod predicates;
pub mod parser;
pub mod registry;
pub mod canonical;

pub use registry::{
    Audience, Entry, NotificationRule, Registry, RegistryError, Severity, VariantKind,
    REGISTRY, REGISTRY_VERSION,
};
pub use predicates::Predicate;
pub use parser::parse_predicate;
pub use canonical::{to_canonical_json, to_canonical_value};
