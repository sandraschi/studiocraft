//! Handoff safety gate: block instead of flattening, always explicit.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use crate::StudioError;
use serde_json::{json, Value};
use std::path::Path;

fn is_editable(editor: &str) -> bool {
    matches!(editor, "photo" | "vector")
}

fn norm(editor: &str) -> String {
    editor.trim().to_lowercase()
}

/// Safety check for one handoff. Never touches natives; only decides allow/block.
pub fn check(
    project: &Path,
    from: &str,
    to: &str,
    unsaved: bool,
    unsupported: bool,
) -> Result<Value, StudioError> {
    let from_n = norm(from);
    let to_n = norm(to);
    if from_n.is_empty() || to_n.is_empty() || from_n.len() > 64 || to_n.len() > 64 {
        return Err(StudioError::Invalid("from/to must be 1..64 chars".into()));
    }
    if from_n == to_n {
        return Ok(json!({
            "allowed": false,
            "reason": "nothing to hand off: from == to",
            "project": project.to_string_lossy(),
        }));
    }
    if unsaved {
        return Ok(json!({
            "allowed": false,
            "reason": "blocked: unsaved work pending, save or discard first",
            "project": project.to_string_lossy(),
            "from": from_n,
            "to": to_n,
        }));
    }
    if unsupported {
        return Ok(json!({
            "allowed": false,
            "reason": "blocked: unsupported content present, resolve or remove first",
            "project": project.to_string_lossy(),
            "from": from_n,
            "to": to_n,
        }));
    }
    if !is_editable(&from_n) || !is_editable(&to_n) {
        return Ok(json!({
            "allowed": false,
            "reason": "blocked: v1 editable set is photo<->vector only; companion is preview-linked",
            "project": project.to_string_lossy(),
            "from": from_n,
            "to": to_n,
            "editable": ["photo", "vector"],
        }));
    }
    Ok(json!({
        "allowed": true,
        "project": project.to_string_lossy(),
        "from": from_n,
        "to": to_n,
        "preserved": ["identity", "names", "ordering", "visibility", "transforms"],
        "note": "native editability retained both ways; composite refresh is preview only",
    }))
}
