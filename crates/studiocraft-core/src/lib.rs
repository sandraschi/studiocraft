//! L1 StudioCraft core: shared `.studiocraft` project, adapter caps, handoff safety,
//! skills, workflows, LLM chat.
//!
//! v1 scope is the Photo <-> Vector safe round-trip from craft-launcher#1233:
//! preserve identity/names/ordering/visibility/transforms, block instead of
//! flattening, explicit caps. Companions are preview-linked in v1.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

pub mod adapters;
pub mod handoff;
pub mod llm;
pub mod project;
pub mod skills;
pub mod workflows;

use serde_json::{json, Value};
use std::fmt;
use std::path::{Component, Path, PathBuf};

pub const APP_NAME: &str = "studiocraft";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CONTRACT_V1: &str = "studiocraft/v1";

#[derive(Debug)]
pub enum StudioError {
    NotFound(String),
    Invalid(String),
    Outside(String),
    Auth(String),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for StudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StudioError::NotFound(m) => write!(f, "not found: {m}"),
            StudioError::Invalid(m) => write!(f, "invalid argument: {m}"),
            StudioError::Outside(m) => write!(f, "path outside allowed root: {m}"),
            StudioError::Auth(m) => write!(f, "auth required: {m}"),
            StudioError::Io(e) => write!(f, "io error: {e}"),
            StudioError::Json(e) => write!(f, "manifest error: {e}"),
        }
    }
}

impl std::error::Error for StudioError {}

impl From<std::io::Error> for StudioError {
    fn from(e: std::io::Error) -> Self {
        StudioError::Io(e)
    }
}

impl From<serde_json::Error> for StudioError {
    fn from(e: serde_json::Error) -> Self {
        StudioError::Json(e)
    }
}

fn clean(path: &Path) -> PathBuf {
    let abs = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
    let mut out = PathBuf::new();
    for comp in abs.components() {
        match comp {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

fn fold(path: &Path) -> String {
    let s = path.to_string_lossy().replace('\\', "/");
    if cfg!(windows) {
        s.to_lowercase()
    } else {
        s
    }
}

/// True when `path` is `root` or lies beneath it (lexical, `..`-safe).
pub fn within(root: &Path, path: &Path) -> bool {
    let r = fold(&clean(root));
    let p = fold(&clean(path));
    p == r || p.starts_with(&format!("{}/", r.trim_end_matches('/')))
}

pub(crate) fn ensure_within(root: Option<&Path>, path: &Path) -> Result<(), StudioError> {
    match root {
        Some(r) if !within(r, path) => Err(StudioError::Outside(format!(
            "{} is not under {}",
            path.display(),
            r.display()
        ))),
        _ => Ok(()),
    }
}

fn arg_str<'a>(args: &'a Value, key: &str) -> Option<&'a str> {
    args.get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty() && s.len() <= 4096)
}

fn arg_bool(args: &Value, key: &str) -> bool {
    args.get(key).and_then(Value::as_bool).unwrap_or(false)
}

fn require_str<'a>(args: &'a Value, key: &str) -> Result<&'a str, StudioError> {
    arg_str(args, key).ok_or_else(|| StudioError::Invalid(format!("missing '{key}'")))
}

fn opt_string(args: &Value, key: &str) -> Option<String> {
    arg_str(args, key).map(str::to_string)
}

fn root_of(args: &Value) -> Option<PathBuf> {
    arg_str(args, "root")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("STUDIOCRAFT_ROOT").map(PathBuf::from))
}

/// Single entry point shared by the CLI subcommands and the MCP tools.
pub fn dispatch(op: &str, args: &Value) -> Result<Value, StudioError> {
    let root = root_of(args);
    let root = root.as_deref();

    let mut body = match op {
        "status" => {
            json!({
                "app": APP_NAME,
                "version": APP_VERSION,
                "contract": CONTRACT_V1,
                "tier": "pure-rust artcraft L1",
                "engines": {
                    "photo": adapters::engine_probe("photocraft"),
                    "vector": adapters::engine_probe("vectorcraft"),
                    "light": adapters::engine_probe("lightcraft"),
                },
            })
        }
        "adapter_caps" => adapters::caps(),
        "project_init" => {
            let path = PathBuf::from(require_str(args, "path")?);
            ensure_within(root, &path)?;
            let name = arg_str(args, "name").unwrap_or("studio");
            if name.len() > 128 {
                return Err(StudioError::Invalid("name too long (max 128)".into()));
            }
            project::init(&path, name)?
        }
        "project_open" => {
            let path = PathBuf::from(require_str(args, "path")?);
            ensure_within(root, &path)?;
            project::open(&path)?
        }
        "project_list" => {
            let dir = arg_str(args, "root")
                .map(PathBuf::from)
                .or_else(|| root.map(PathBuf::from))
                .or_else(|| std::env::current_dir().ok())
                .ok_or_else(|| StudioError::Invalid("no root to scan".into()))?;
            workflows::list_projects(&dir)?
        }
        "handoff_check" => {
            let proj = PathBuf::from(require_str(args, "project")?);
            ensure_within(root, &proj)?;
            let from = require_str(args, "from")?;
            let to = require_str(args, "to")?;
            handoff::check(
                &proj,
                from,
                to,
                arg_bool(args, "unsaved"),
                arg_bool(args, "unsupported"),
            )?
        }
        "companion_attach" => {
            let proj = PathBuf::from(require_str(args, "project")?);
            ensure_within(root, &proj)?;
            let file = PathBuf::from(require_str(args, "path")?);
            let kind = require_str(args, "kind")?;
            workflows::companion_attach(&proj, &file, kind)?
        }
        "composite_bump" => {
            let proj = PathBuf::from(require_str(args, "project")?);
            ensure_within(root, &proj)?;
            workflows::composite_bump(&proj)?
        }
        "skills_list" => skills::list(),
        "skill_show" => {
            let id = require_str(args, "id")?;
            skills::show(id).ok_or_else(|| StudioError::NotFound(format!("no skill '{id}'")))?
        }
        "llm_chat" => {
            let message = require_str(args, "message")?.to_string();
            let reply = llm::chat_blocking(
                opt_string(args, "provider"),
                opt_string(args, "base_url"),
                opt_string(args, "model"),
                opt_string(args, "api_key"),
                opt_string(args, "system"),
                message,
            )?;
            json!({ "reply": reply })
        }
        "llm_models" => {
            let models = llm::models_blocking(
                opt_string(args, "provider"),
                opt_string(args, "base_url"),
                opt_string(args, "api_key"),
            )?;
            json!({ "count": models.len(), "models": models })
        }
        other => return Err(StudioError::Invalid(format!("unknown operation '{other}'"))),
    };

    if let Some(obj) = body.as_object_mut() {
        obj.insert("success".into(), json!(true));
    }
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn within_blocks_dotdot_escape() {
        let root = Path::new("D:/Dev/repos");
        assert!(within(root, Path::new("D:/Dev/repos/studiocraft/x")));
        assert!(within(root, Path::new("D:/Dev/repos")));
        assert!(!within(root, Path::new("D:/Dev/repos/../secrets")));
        assert!(!within(root, Path::new("D:/Dev/repos-evil/x")));
    }

    #[test]
    fn unknown_op_is_an_error() {
        assert!(matches!(
            dispatch("explode", &json!({})),
            Err(StudioError::Invalid(_))
        ));
    }

    #[test]
    fn status_reports_contract() {
        let v = dispatch("status", &json!({})).unwrap_or_default();
        assert_eq!(v["contract"], CONTRACT_V1);
        assert_eq!(v["success"], true);
    }

    #[test]
    fn caps_lists_photo_vector_editable() {
        let v = dispatch("adapter_caps", &json!({})).unwrap_or_default();
        let editable = v["editable"].as_array().cloned().unwrap_or_default();
        let names: Vec<String> = editable
            .iter()
            .filter_map(|e| e.get("id"))
            .filter_map(Value::as_str)
            .map(str::to_string)
            .collect();
        assert!(names.contains(&"photo".to_string()));
        assert!(names.contains(&"vector".to_string()));
    }

    #[test]
    fn skills_registry_has_roundtrip() {
        let v = dispatch("skills_list", &json!({})).unwrap_or_default();
        assert!(v["count"].as_i64().unwrap_or(0) >= 6);
        let one =
            dispatch("skill_show", &json!({"id": "photo-vector-roundtrip"})).unwrap_or_default();
        assert_eq!(one["id"], "photo-vector-roundtrip");
    }

    #[test]
    fn handoff_allows_photo_vector_clean() {
        let ok = dispatch(
            "handoff_check",
            &json!({"project": "D:/tmp/p", "from": "photo", "to": "vector"}),
        )
        .unwrap_or_default();
        assert_eq!(ok["allowed"], true);
        assert_eq!(ok["preserved"].as_array().map(Vec::len).unwrap_or(0), 5);
    }
}
