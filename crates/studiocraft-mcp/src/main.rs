//! L2 StudioCraft CLI and MCP stdio server.
//!
//! One binary: `studiocraft-cli <verb>` for humans/scripts, `studiocraft-cli mcp`
//! for agents. Both surfaces call `studiocraft_core::dispatch`, so they agree.
//!
//! Stdio invariant: in `mcp` mode stdout is 100% line-delimited JSON-RPC.
//! Diagnostics go to stderr only.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use clap::{Parser, Subcommand};
use serde_json::{json, Map, Value};
use std::io::{BufRead, Write};
use std::path::PathBuf;
use studiocraft_core::{dispatch, StudioError, APP_NAME, APP_VERSION};

#[derive(Parser)]
#[command(name = "studiocraft-cli", about = "StudioCraft shared-project host (CLI-first, MCP over stdio)", version = APP_VERSION)]
struct Cli {
    #[arg(long, global = true, env = "STUDIOCRAFT_ROOT")]
    root: Option<PathBuf>,
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Report version, contract, engine probes
    Status,
    /// Show v1 adapter caps with provenance tags
    Caps,
    /// Create a new .studiocraft project dir
    Init {
        #[arg(long)]
        path: PathBuf,
        #[arg(long, default_value = "studio")]
        name: String,
    },
    /// Open and validate an existing project
    Open {
        #[arg(long)]
        path: PathBuf,
    },
    /// Scan a root dir for .studiocraft projects
    List {
        #[arg(long)]
        root: Option<PathBuf>,
    },
    /// Safety check for one handoff (never touches natives)
    HandoffCheck {
        #[arg(long)]
        project: PathBuf,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long, default_value_t = false)]
        unsaved: bool,
        #[arg(long, default_value_t = false)]
        unsupported: bool,
    },
    /// Attach a companion pointer (never copies bytes)
    CompanionAttach {
        #[arg(long)]
        project: PathBuf,
        #[arg(long)]
        path: PathBuf,
        #[arg(long)]
        kind: String,
    },
    /// Bump manifest revision after a verified round-trip
    CompositeBump {
        #[arg(long)]
        project: PathBuf,
    },
    /// List guided skills
    Skills,
    /// Show one skill recipe
    SkillShow {
        #[arg(long)]
        id: String,
    },
    /// One LLM chat turn (Ollama/LM Studio/OpenAI/Anthropic/Custom)
    LlmChat {
        #[arg(long)]
        message: String,
        #[arg(long)]
        provider: Option<String>,
        #[arg(long)]
        base_url: Option<String>,
        #[arg(long)]
        model: Option<String>,
        #[arg(long)]
        api_key: Option<String>,
        #[arg(long)]
        system: Option<String>,
    },
    /// List models for a provider (Anthropic has none: type the id)
    LlmModels {
        #[arg(long)]
        provider: Option<String>,
        #[arg(long)]
        base_url: Option<String>,
        #[arg(long)]
        api_key: Option<String>,
    },
    /// List the MCP tool surface as JSON
    Tools,
    /// Run the MCP 2024-11-05 stdio server
    Mcp,
}

fn emit(text: &str) {
    let mut out = std::io::stdout().lock();
    match writeln!(out, "{text}") {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => std::process::exit(0),
        Err(e) => {
            eprintln!("error: failed to write stdout: {e}");
            std::process::exit(1);
        }
    }
}

macro_rules! out {
    ($($t:tt)*) => {
        emit(&format!($($t)*))
    };
}

fn base_args(cli: &Cli) -> Map<String, Value> {
    let mut args = Map::new();
    if let Some(root) = &cli.root {
        args.insert("root".into(), json!(root.to_string_lossy()));
    }
    args
}

fn error_type(error: &StudioError) -> &'static str {
    match error {
        StudioError::NotFound(_) => "not_found",
        StudioError::Invalid(_) => "invalid",
        StudioError::Outside(_) => "outside",
        StudioError::Auth(_) => "auth",
        StudioError::Io(_) => "io",
        StudioError::Json(_) => "manifest",
    }
}

fn error_payload(error: &StudioError) -> Value {
    json!({
        "success": false,
        "error": error.to_string(),
        "error_type": error_type(error),
    })
}

fn run(cli: &Cli) -> i32 {
    let mut args = base_args(cli);
    let (op, inserts): (&str, Vec<(&str, Value)>) = match &cli.command {
        Commands::Status => ("status", Vec::new()),
        Commands::Caps => ("adapter_caps", Vec::new()),
        Commands::Init { path, name } => (
            "project_init",
            vec![
                ("path", json!(path.to_string_lossy())),
                ("name", json!(name)),
            ],
        ),
        Commands::Open { path } => (
            "project_open",
            vec![("path", json!(path.to_string_lossy()))],
        ),
        Commands::List { root } => {
            let mut pairs = Vec::new();
            if let Some(r) = root {
                pairs.push(("root", json!(r.to_string_lossy())));
            }
            ("project_list", pairs)
        }
        Commands::HandoffCheck {
            project,
            from,
            to,
            unsaved,
            unsupported,
        } => (
            "handoff_check",
            vec![
                ("project", json!(project.to_string_lossy())),
                ("from", json!(from)),
                ("to", json!(to)),
                ("unsaved", json!(unsaved)),
                ("unsupported", json!(unsupported)),
            ],
        ),
        Commands::CompanionAttach {
            project,
            path,
            kind,
        } => (
            "companion_attach",
            vec![
                ("project", json!(project.to_string_lossy())),
                ("path", json!(path.to_string_lossy())),
                ("kind", json!(kind)),
            ],
        ),
        Commands::CompositeBump { project } => (
            "composite_bump",
            vec![("project", json!(project.to_string_lossy()))],
        ),
        Commands::Skills => ("skills_list", Vec::new()),
        Commands::SkillShow { id } => ("skill_show", vec![("id", json!(id))]),
        Commands::LlmChat {
            message,
            provider,
            base_url,
            model,
            api_key,
            system,
        } => {
            let mut pairs = vec![("message", json!(message))];
            if let Some(v) = provider {
                pairs.push(("provider", json!(v)));
            }
            if let Some(v) = base_url {
                pairs.push(("base_url", json!(v)));
            }
            if let Some(v) = model {
                pairs.push(("model", json!(v)));
            }
            if let Some(v) = api_key {
                pairs.push(("api_key", json!(v)));
            }
            if let Some(v) = system {
                pairs.push(("system", json!(v)));
            }
            ("llm_chat", pairs)
        }
        Commands::LlmModels {
            provider,
            base_url,
            api_key,
        } => {
            let mut pairs = Vec::new();
            if let Some(v) = provider {
                pairs.push(("provider", json!(v)));
            }
            if let Some(v) = base_url {
                pairs.push(("base_url", json!(v)));
            }
            if let Some(v) = api_key {
                pairs.push(("api_key", json!(v)));
            }
            ("llm_models", pairs)
        }
        Commands::Tools => {
            let tools = tools_list();
            out!(
                "{}",
                serde_json::to_string_pretty(&tools).unwrap_or_else(|_| "[]".to_string())
            );
            return 0;
        }
        Commands::Mcp => {
            run_mcp_stdio(&args);
            return 0;
        }
    };

    for (key, value) in inserts {
        args.insert(key.to_string(), value);
    }

    match dispatch(op, &Value::Object(args)) {
        Ok(value) => {
            out!(
                "{}",
                serde_json::to_string_pretty(&value).unwrap_or_else(|_| "{}".to_string())
            );
            0
        }
        Err(error) => {
            if cli.json {
                out!("{}", error_payload(&error));
            } else {
                eprintln!("error: {error}");
            }
            1
        }
    }
}

fn main() {
    let cli = Cli::parse();
    std::process::exit(run(&cli));
}

fn schema(properties: Value, required: Vec<&str>) -> Value {
    json!({
        "type": "object",
        "properties": properties,
        "required": required,
        "additionalProperties": true
    })
}

pub fn tools_list() -> Value {
    let root =
        || json!({"type": "string", "description": "Confinement root (default: STUDIOCRAFT_ROOT)"});
    json!([
        {
            "name": "studio_status",
            "description": "Report StudioCraft version, contract v1, engine probes (down when missing, never faked).",
            "inputSchema": schema(json!({"root": root()}), vec![])
        },
        {
            "name": "studio_caps",
            "description": "Show v1 adapter caps: photo<->vector editable, rest preview-linked, with provenance.",
            "inputSchema": schema(json!({"root": root()}), vec![])
        },
        {
            "name": "studio_init",
            "description": "Create a new .studiocraft project dir with manifest v1 + native subdirs.",
            "inputSchema": schema(json!({"root": root(), "path": {"type": "string"}, "name": {"type": "string"}}), vec!["path"])
        },
        {
            "name": "studio_open",
            "description": "Open and validate an existing .studiocraft project (contract must be studiocraft/v1).",
            "inputSchema": schema(json!({"root": root(), "path": {"type": "string"}}), vec!["path"])
        },
        {
            "name": "studio_list",
            "description": "Scan a root dir for .studiocraft projects (max 100 entries).",
            "inputSchema": schema(json!({"root": root()}), vec![])
        },
        {
            "name": "studio_handoff_check",
            "description": "Safety gate for one handoff: blocks unsaved/unsupported/non-editable instead of flattening.",
            "inputSchema": schema(json!({
                "root": root(),
                "project": {"type": "string"},
                "from": {"type": "string"},
                "to": {"type": "string"},
                "unsaved": {"type": "boolean"},
                "unsupported": {"type": "boolean"}
            }), vec!["project", "from", "to"])
        },
        {
            "name": "studio_companion_attach",
            "description": "Attach a companion pointer (design/pdf/word/deck/grid/effect/film/cad/sound). Pointer only, never copies bytes.",
            "inputSchema": schema(json!({"root": root(), "project": {"type": "string"}, "path": {"type": "string"}, "kind": {"type": "string"}}), vec!["project", "path", "kind"])
        },
        {
            "name": "studio_composite_bump",
            "description": "Bump manifest revision by 1 after a verified round-trip (checked_add, no silent reset).",
            "inputSchema": schema(json!({"root": root(), "project": {"type": "string"}}), vec!["project"])
        },
        {
            "name": "studio_skills",
            "description": "List guided StudioCraft skills (round-trip, safe gate, companion, rescue, hygiene, chat recipe).",
            "inputSchema": schema(json!({"root": root()}), vec![])
        },
        {
            "name": "studio_skill_show",
            "description": "Show one skill recipe with steps and safety notes.",
            "inputSchema": schema(json!({"root": root(), "id": {"type": "string"}}), vec!["id"])
        },
        {
            "name": "studio_llm_chat",
            "description": "One LLM chat turn (ollama/lmstudio/openai/anthropic/custom). Keys via args or LLM_API_KEY env, never on disk.",
            "inputSchema": schema(json!({
                "provider": {"type": "string"},
                "base_url": {"type": "string"},
                "model": {"type": "string"},
                "api_key": {"type": "string"},
                "system": {"type": "string"},
                "message": {"type": "string"}
            }), vec!["message"])
        },
        {
            "name": "studio_llm_models",
            "description": "List models for a provider (Anthropic has none: type the model id).",
            "inputSchema": schema(json!({
                "provider": {"type": "string"},
                "base_url": {"type": "string"},
                "api_key": {"type": "string"}
            }), vec![])
        }
    ])
}

pub fn call_tool(name: &str, args: &Value) -> Value {
    let op = match name {
        "studio_status" | "studio.status" => "status",
        "studio_caps" | "studio.caps" | "studio_adapter_caps" => "adapter_caps",
        "studio_init" | "studio.init" | "studio_project_init" => "project_init",
        "studio_open" | "studio.open" | "studio_project_open" => "project_open",
        "studio_list" | "studio.list" | "studio_project_list" => "project_list",
        "studio_handoff_check" | "studio.handoff_check" => "handoff_check",
        "studio_companion_attach" | "studio.companion_attach" => "companion_attach",
        "studio_composite_bump" | "studio.composite_bump" => "composite_bump",
        "studio_skills" | "studio.skills" | "studio_skills_list" => "skills_list",
        "studio_skill_show" | "studio.skill_show" => "skill_show",
        "studio_llm_chat" | "studio.llm_chat" => "llm_chat",
        "studio_llm_models" | "studio.llm_models" => "llm_models",
        other => {
            return json!({
                "success": false,
                "error": format!("unknown tool '{other}'"),
                "error_type": "invalid"
            })
        }
    };
    match dispatch(op, args) {
        Ok(value) => value,
        Err(error) => error_payload(&error),
    }
}

fn merge_defaults(defaults: &Map<String, Value>, arguments: Value) -> Value {
    let mut merged = defaults.clone();
    if let Some(obj) = arguments.as_object() {
        for (key, value) in obj {
            merged.insert(key.clone(), value.clone());
        }
    }
    Value::Object(merged)
}

fn run_mcp_stdio(defaults: &Map<String, Value>) {
    eprintln!("[studiocraft-mcp] starting MCP stdio loop (2024-11-05)");
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    for line in stdin.lock().lines() {
        let text = match line {
            Ok(t) => t,
            Err(_) => break,
        };
        let trimmed = text.trim();
        if trimmed.is_empty() {
            continue;
        }
        let request: Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(error) => {
                eprintln!("[studiocraft-mcp] JSON parse error: {error}");
                continue;
            }
        };
        let method = request["method"].as_str().unwrap_or("");
        let id = request.get("id").cloned();
        let response = match method {
            "initialize" => json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": { "tools": { "listChanged": false } },
                    "serverInfo": { "name": APP_NAME, "version": APP_VERSION }
                }
            }),
            "notifications/initialized" | "notifications/cancelled" => continue,
            "ping" => json!({"jsonrpc": "2.0", "id": id, "result": {}}),
            "tools/list" => json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": { "tools": tools_list() }
            }),
            "tools/call" => {
                let params = request.get("params").cloned().unwrap_or(Value::Null);
                let tool_name = params["name"].as_str().unwrap_or("");
                let arguments = merge_defaults(
                    defaults,
                    params.get("arguments").cloned().unwrap_or(json!({})),
                );
                let result = call_tool(tool_name, &arguments);
                let is_error = result
                    .get("success")
                    .and_then(Value::as_bool)
                    .map(|ok| !ok)
                    .unwrap_or(true);
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "content": [{ "type": "text", "text": result.to_string() }],
                        "isError": is_error
                    }
                })
            }
            _ => {
                if id.is_some() {
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": { "code": -32601, "message": format!("method not found: {method}") }
                    })
                } else {
                    continue;
                }
            }
        };
        if writeln!(stdout, "{response}").is_err() {
            break;
        }
        let _ = stdout.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_surface_is_complete_and_valid() {
        let tools = tools_list();
        let tools = tools.as_array().cloned().unwrap_or_default();
        assert_eq!(tools.len(), 12);
        for tool in &tools {
            let name = tool["name"].as_str().unwrap_or("");
            assert!(name.starts_with("studio_"), "{name}");
            assert_eq!(tool["inputSchema"]["type"], "object");
        }
    }

    #[test]
    fn unknown_tool_is_an_error_payload() {
        let result = call_tool("studio_explode", &json!({}));
        assert_eq!(result["success"], false);
        assert_eq!(result["error_type"], "invalid");
    }

    #[test]
    fn dotted_and_underscore_names_dispatch_identically() {
        let dotted = call_tool("studio.status", &json!({}));
        let underscore = call_tool("studio_status", &json!({}));
        assert_eq!(dotted["success"], true);
        assert_eq!(underscore["success"], true);
        assert_eq!(dotted["version"], underscore["version"]);
    }
}
