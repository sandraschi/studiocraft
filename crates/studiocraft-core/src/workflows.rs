//! Project workflows: list, companion pointers, composite revision.
//!
//! Pointers only: companion files are never copied into the project in v1.
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

fn read_manifest(project: &Path) -> Result<Value, StudioError> {
    let raw = std::fs::read_to_string(manifest_path(project)).map_err(|_| {
        StudioError::NotFound(format!("no manifest.json under {}", project.display()))
    })?;
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
    Ok(manifest)
}

/// Scan `root` (max 100 entries) for dirs containing manifest.json.
pub fn list_projects(root: &Path) -> Result<Value, StudioError> {
    let entries = std::fs::read_dir(root)
        .map_err(|e| StudioError::NotFound(format!("cannot list {}: {e}", root.display())))?;
    let mut projects: Vec<Value> = Vec::new();
    for entry in entries.take(100) {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        if manifest_path(&path).is_file() {
            if let Ok(manifest) = read_manifest(&path) {
                let name = manifest
                    .get("project")
                    .and_then(Value::as_str)
                    .unwrap_or("studio");
                let revision = manifest
                    .get("revision")
                    .and_then(Value::as_i64)
                    .unwrap_or(0);
                projects.push(json!({
                    "path": path.to_string_lossy(),
                    "project": name,
                    "revision": revision,
                }));
            }
        }
    }
    Ok(json!({ "root": root.to_string_lossy(), "count": projects.len(), "projects": projects }))
}

fn valid_kind(kind: &str) -> bool {
    matches!(
        kind,
        "design" | "pdf" | "word" | "deck" | "grid" | "effect" | "film" | "cad" | "sound"
    )
}

/// Attach a companion pointer: writes companions/<kind>.json, never copies bytes.
pub fn companion_attach(project: &Path, file: &Path, kind: &str) -> Result<Value, StudioError> {
    let kind_n = kind.trim().to_lowercase();
    if !valid_kind(&kind_n) {
        return Err(StudioError::Invalid(format!(
            "unknown companion kind '{kind}': want design|pdf|word|deck|grid|effect|film|cad|sound"
        )));
    }
    let _manifest = read_manifest(project)?;
    let dir = project.join("companions");
    std::fs::create_dir_all(&dir)?;
    let pointer = dir.join(format!("{kind_n}.json"));
    let body = json!({
        "kind": kind_n,
        "path": file.to_string_lossy(),
        "mode": "linked preview (v1: never becomes a Photo layer)",
    });
    std::fs::write(&pointer, serde_json::to_string_pretty(&body)?)?;
    Ok(json!({ "pointer": pointer.to_string_lossy(), "attached": body }))
}

/// Bump manifest revision by 1 (checked_add, no silent reset).
pub fn composite_bump(project: &Path) -> Result<Value, StudioError> {
    let manifest = read_manifest(project)?;
    let revision = manifest
        .get("revision")
        .and_then(Value::as_i64)
        .unwrap_or(0);
    let next = revision
        .checked_add(1)
        .ok_or_else(|| StudioError::Invalid("revision overflow".into()))?;
    let mut updated = manifest;
    if let Some(obj) = updated.as_object_mut() {
        obj.insert("revision".into(), json!(next));
    }
    std::fs::write(
        manifest_path(project),
        serde_json::to_string_pretty(&updated)?,
    )?;
    Ok(json!({ "path": project.to_string_lossy(), "revision": next }))
}
