//! Les schémas JSON versionnés dans `schemas/` doivent correspondre aux types Rust (ADR-001).
//! Régénération : `ORI_UPDATE_SCHEMAS=1 cargo test -p ori-protocol --test schemas`.

#![allow(clippy::unwrap_used, clippy::panic)] // fichier de test uniquement

use std::path::PathBuf;

use ori_protocol::error::ErrorBody;
use ori_protocol::tool::{ToolCall, ToolDescriptor, ToolResult};
use ori_protocol::ws::{ClientMessage, ServerMessage};
use schemars::{Schema, schema_for};

fn generated() -> Vec<(&'static str, Schema)> {
    vec![
        ("client-message", schema_for!(ClientMessage)),
        ("server-message", schema_for!(ServerMessage)),
        ("error-body", schema_for!(ErrorBody)),
        ("tool-descriptor", schema_for!(ToolDescriptor)),
        ("tool-call", schema_for!(ToolCall)),
        ("tool-result", schema_for!(ToolResult)),
    ]
}

fn schemas_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../schemas")
}

#[test]
fn committed_schemas_match_the_rust_types() {
    let update = std::env::var("ORI_UPDATE_SCHEMAS").is_ok_and(|v| v == "1");
    let mut stale = Vec::new();
    for (name, schema) in generated() {
        let path = schemas_dir().join(format!("{name}.schema.json"));
        let expected = format!("{}\n", serde_json::to_string_pretty(&schema).unwrap());
        if update {
            std::fs::create_dir_all(schemas_dir()).unwrap();
            std::fs::write(&path, &expected).unwrap();
        } else if std::fs::read_to_string(&path).ok().as_deref() != Some(expected.as_str()) {
            stale.push(name);
        }
    }
    if !stale.is_empty() {
        panic!(
            "schémas obsolètes : {stale:?} ; lancez ORI_UPDATE_SCHEMAS=1 cargo test -p ori-protocol --test schemas"
        );
    }
}
