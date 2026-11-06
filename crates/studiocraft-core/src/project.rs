//! `.studiocraft` project format v1: manifest + native dirs, never flattened.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use crate::{StudioError, CONTRACT_V1};
use serde_json::{json, Value};
use std::path::Path;

fn manifest_path(project: &Path) -> std::path::PathBuf {
    project.join("manifest.json")
}

fn write_file(path: &Path, bytes: &[u8]) -> Result<(), StudioError> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    std::fs::write(path, bytes)?;
    Ok(())
}

/// Create a new v1 project dir with manifest + native subdirs.
pub fn init(path: &Path, name: &str) -> Result<Value, StudioError> {
    if name.is_empty() || name.len() > 128 {
        return Err(StudioError::Invalid("name must be 1..128 chars".into()));
    }
    for sub in ["painting", "vectors", "companions"] {
        let dir = path.join(sub);
        std::fs::create_dir_all(&dir)?;
    }
    let manifest = json!({
        "contract": CONTRACT_V1,
        "project": name,
        "revision": 1,
        "editors": {
            "photo": {"writes": "raster layers (native)"},
            "vector": {"writes": "vector objects id-stable (native smart layer)"},
        },
        "preserved": ["identity", "names", "ordering", "visibility", "transforms"],
        "composite": {"file": "composite.png", "max_mp": 4, "note": "rev-cached preview for companions only"},
    });
    let text = serde_json::to_string_pretty(&manifest)?;
    write_file(&manifest_path(path), text.as_bytes())?;
    Ok(json!({
        "path": path.to_string_lossy(),
        "manifest": manifest,
    }))
}

/// Open and validate an existing v1 project.
pub fn open(path: &Path) -> Result<Value, StudioError> {
    let raw = std::fs::read_to_string(manifest_path(path))
        .map_err(|_| StudioError::NotFound(format!("no manifest.json under {}", path.display())))?;
    if raw.len() > 1_048_576 {
        return Err(StudioError::Invalid("manifest too large".into()));
    }
    let manifest: Value = serde_json::from_str(&raw)?;
    let contract = manifest
        .get("contract")
        .and_then(Value::as_str)
        .unwrap_or("");
    if contract != CONTRACT_V1 {
        return Err(StudioError::Invalid(format!(
            "unsupported contract '{contract}', want '{CONTRACT_V1}'"
        )));
    }
    Ok(json!({
        "path": path.to_string_lossy(),
        "manifest": manifest,
        "editable": ["photo", "vector"],
        "note": "companions are preview-linked in v1",
    }))
}
