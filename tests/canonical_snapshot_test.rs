use std::path::Path;

use mykobo_rs::notification_contract::{to_canonical_json, REGISTRY};

#[test]
fn registry_matches_committed_snapshot() {
    let snapshot = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/notification_contract/registry.canonical.json");
    let expected = std::fs::read_to_string(&snapshot).expect("read snapshot");
    let actual = to_canonical_json(&REGISTRY);
    assert_eq!(
        actual, expected,
        "Registry no longer matches the committed canonical snapshot. \
         Run `cargo run --bin regenerate_snapshot` and commit the result."
    );
}
