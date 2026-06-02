use std::path::PathBuf;

use mykobo_rs::notification_contract::{to_canonical_json, REGISTRY};

fn main() {
    let out: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/notification_contract/registry.canonical.json");
    std::fs::create_dir_all(out.parent().unwrap()).expect("mkdir");
    std::fs::write(&out, to_canonical_json(&REGISTRY)).expect("write");
    println!("wrote {}", out.display());
}
