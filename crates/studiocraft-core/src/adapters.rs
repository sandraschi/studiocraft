//! Adapter capability registry for StudioCraft v1.
//!
//! Photo + Vector are editable. Light is settings + LUT preview. Companions
//! (design/pdf/word/deck/grid/...) are preview-linked in v1 and must never be
//! presented as editable handoff targets.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use serde_json::{json, Value};
use std::path::PathBuf;

/// Probe an engine binary without assuming one machine's install layout.
/// Precedence: explicit env var > PATH. Never fakes: missing is `down`.
pub fn engine_probe(engine: &str) -> Value {
    let env_var = match engine {
        "photocraft" => "PHOTOCRAFT_EXE",
        "vectorcraft" => "VECTORCRAFT_EXE",
        "lightcraft" => "LIGHTCRAFT_EXE",
        _ => "",
    };
    if !env_var.is_empty() {
        if let Some(explicit) = std::env::var_os(env_var) {
            let p = PathBuf::from(explicit);
            if p.is_file() {
                return json!({"status": "ok", "path": p.to_string_lossy()});
            }
            return json!({"status": "down", "message": "explicit path not found"});
        }
    }
    let exe = if cfg!(windows) {
        format!("{engine}.exe")
    } else {
        engine.to_string()
    };
    if let Some(paths) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&paths) {
            let candidate = dir.join(&exe);
            if candidate.is_file() {
                return json!({"status": "ok", "path": candidate.to_string_lossy()});
            }
        }
    }
    // Storytold default install roots (best-effort, no hardcoding of user dirs).
    let mut roots: Vec<PathBuf> = Vec::new();
    for var in ["ProgramFiles", "ProgramFiles(x86)", "LOCALAPPDATA"] {
        if let Some(base) = std::env::var_os(var) {
            let base = PathBuf::from(base);
            roots.push(base.join("ArtCraft").join(&exe));
            roots.push(base.join("Storytold").join(&exe));
        }
    }
    for p in roots {
        if p.is_file() {
            return json!({"status": "ok", "path": p.to_string_lossy()});
        }
    }
    json!({"status": "down", "message": "engine not found on PATH"})
}

/// Static v1 capability table with provenance tags.
pub fn caps() -> Value {
    json!({
        "contract": crate::CONTRACT_V1,
        "editable": [
            {
                "id": "photo",
                "label": "PhotoCraft",
                "origin": "storytold-ref",
                "writes": "raster layers (native)",
                "preserved": ["identity", "names", "ordering", "visibility", "transforms"],
            },
            {
                "id": "vector",
                "label": "VectorCraft",
                "origin": "storytold-ref",
                "writes": "vector objects id-stable (native smart layer)",
                "preserved": ["identity", "names", "ordering", "visibility", "transforms"],
            },
        ],
        "preview": [
            {
                "id": "light",
                "label": "LightCraft",
                "origin": "storytold-ref",
                "writes": "settings + reversible LUT (v1 preview)",
                "preserved": ["settings"],
            },
            {
                "id": "design",
                "label": "DesignCraft",
                "origin": "storytold-ref",
                "writes": "companion only, linked composite (v1)",
                "preserved": ["native companion doc"],
            },
        ],
        "fleet_owned": [
            {
                "id": "studiocraft",
                "label": "StudioCraft host",
                "origin": "fleet-owned",
                "writes": "manifest + composite revision, never flattens natives",
                "preserved": ["manifest", "revision"],
            },
        ],
        "rule": "v1 editable set is photo<->vector only; everything else blocks as handoff target with explicit reason",
    })
}
