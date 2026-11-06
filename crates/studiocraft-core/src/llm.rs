//! LLM chat for StudioCraft: Ollama, LM Studio, and OpenAI-compatible clouds.
//!
//! One shape serves Ollama (:11434), LM Studio (:1234) and clouds
//! (POST {base}/chat/completions). Anthropic rides a native path
//! (/v1/messages, x-api-key). Non-streaming only (robust first).
//! Provider/base/model persist in the UI; API keys never persist:
//! explicit arg > LLM_API_KEY > OPENAI_API_KEY / ANTHROPIC_API_KEY.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use crate::StudioError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    Ollama,
    LmStudio,
    OpenAi,
    Anthropic,
    Custom,
}

impl Provider {
    pub fn parse(raw: &str) -> Result<Self, StudioError> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "ollama" => Ok(Self::Ollama),
            "lmstudio" | "lm-studio" | "lm_studio" => Ok(Self::LmStudio),
            "openai" => Ok(Self::OpenAi),
            "anthropic" | "claude" => Ok(Self::Anthropic),
            "custom" | "compatible" | "openai-compatible" => Ok(Self::Custom),
            _ => Err(StudioError::Invalid(format!(
                "unknown LLM provider '{raw}': want ollama|lmstudio|openai|anthropic|custom"
            ))),
        }
    }

    pub fn default_base(&self) -> &'static str {
        match self {
            Self::Ollama => "http://127.0.0.1:11434/v1",
            Self::LmStudio => "http://127.0.0.1:1234/v1",
            Self::OpenAi => "https://api.openai.com/v1",
            Self::Anthropic => "https://api.anthropic.com",
            Self::Custom => "",
        }
    }

    pub fn default_model(&self) -> &'static str {
        match self {
            Self::Ollama => "gemma4:12b",
            Self::LmStudio => "local-model",
            Self::OpenAi => "gpt-4o-mini",
            Self::Anthropic => "claude-sonnet-4-20250514",
            Self::Custom => "",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }
}

fn clean(value: Option<String>) -> Option<String> {
    value
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn env_non_empty(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn http_client() -> Result<reqwest::Client, StudioError> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|e| StudioError::Invalid(format!("llm http client: {e}")))
}

pub struct ResolvedProvider {
    pub kind: Provider,
    pub base: String,
    pub model: String,
    pub api_key: Option<String>,
}

pub fn resolve(
    provider: Option<String>,
    base_url: Option<String>,
    model: Option<String>,
    api_key: Option<String>,
) -> Result<ResolvedProvider, StudioError> {
    let kind = match clean(provider).or_else(|| env_non_empty("LLM_PROVIDER")) {
        Some(raw) => Provider::parse(&raw)?,
        None => Provider::Ollama,
    };
    let base = clean(base_url)
        .or_else(|| env_non_empty("LLM_BASE_URL"))
        .unwrap_or_else(|| kind.default_base().to_string());
    if base.trim().is_empty() {
        return Err(StudioError::Invalid(
            "LLM base URL required for provider=custom: --base-url or LLM_BASE_URL".to_string(),
        ));
    }
    let model = clean(model)
        .or_else(|| env_non_empty("LLM_MODEL"))
        .unwrap_or_else(|| kind.default_model().to_string());
    if model.trim().is_empty() {
        return Err(StudioError::Invalid(
            "LLM model required: --model or LLM_MODEL".to_string(),
        ));
    }
    let key = clean(api_key)
        .or_else(|| env_non_empty("LLM_API_KEY"))
        .or_else(|| {
            env_non_empty(match kind {
                Provider::Anthropic => "ANTHROPIC_API_KEY",
                _ => "OPENAI_API_KEY",
            })
        });
    if matches!(
        kind,
        Provider::OpenAi | Provider::Anthropic | Provider::Custom
    ) && key.is_none()
    {
        return Err(StudioError::Auth(
            "LLM API key required for cloud providers: --api-key or LLM_API_KEY".to_string(),
        ));
    }
    Ok(ResolvedProvider {
        kind,
        base: base.trim_end_matches('/').to_string(),
        model,
        api_key: key,
    })
}

fn openai_body(model: &str, messages: &[ChatMessage]) -> serde_json::Value {
    serde_json::json!({ "model": model, "messages": messages, "stream": false })
}

fn openai_reply(body: &serde_json::Value) -> Result<String, StudioError> {
    body.get("choices")
        .and_then(|c| c.as_array())
        .and_then(|a| a.first())
        .and_then(|choice| choice.get("message"))
        .and_then(|msg| msg.get("content"))
        .and_then(|c| c.as_str())
        .map(str::to_string)
        .ok_or_else(|| StudioError::Invalid("llm: chat response carried no message".to_string()))
}

fn anthropic_body(model: &str, messages: &[ChatMessage]) -> serde_json::Value {
    let (system, rest): (Vec<_>, Vec<_>) = messages.iter().partition(|m| m.role == "system");
    let system_text = system
        .iter()
        .map(|m| m.content.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let mut body = serde_json::json!({
        "model": model,
        "max_tokens": 1024,
        "messages": rest.into_iter().map(|m| serde_json::json!({
            "role": if m.role == "assistant" { "assistant" } else { "user" },
            "content": m.content,
        })).collect::<Vec<_>>(),
    });
    if !system_text.is_empty() {
        body["system"] = system_text.into();
    }
    body
}

fn anthropic_reply(body: &serde_json::Value) -> Result<String, StudioError> {
    body.get("content")
        .and_then(|c| c.as_array())
        .and_then(|a| {
            a.iter().find_map(|block| {
                if block.get("type").and_then(|t| t.as_str()) == Some("text") {
                    block.get("text").and_then(|t| t.as_str())
                } else {
                    None
                }
            })
        })
        .map(str::to_string)
        .ok_or_else(|| StudioError::Invalid("llm: anthropic response carried no text".to_string()))
}

fn runtime() -> Result<tokio::runtime::Runtime, StudioError> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| StudioError::Invalid(format!("llm runtime: {e}")))
}

async fn decode(
    op: &str,
    resp: reqwest::Response,
    extract: fn(&serde_json::Value) -> Result<String, StudioError>,
) -> Result<String, StudioError> {
    let status = resp.status();
    let text = resp
        .text()
        .await
        .map_err(|e| StudioError::Invalid(format!("{op} body: {e}")))?;
    if !status.is_success() {
        let snippet: String = text.chars().take(300).collect();
        return Err(StudioError::Invalid(format!(
            "{op} HTTP {status}: {snippet}"
        )));
    }
    let body: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| StudioError::Invalid(format!("{op} decode: {e}")))?;
    extract(&body)
}

/// One chat turn (blocking wrapper over async; local 27B models think slowly).
pub fn chat_blocking(
    provider: Option<String>,
    base_url: Option<String>,
    model: Option<String>,
    api_key: Option<String>,
    system: Option<String>,
    message: String,
) -> Result<String, StudioError> {
    if message.trim().is_empty() {
        return Err(StudioError::Invalid(
            "chat needs a non-empty message".into(),
        ));
    }
    if message.len() > 8000 {
        return Err(StudioError::Invalid(
            "message too long (max 8000 chars)".into(),
        ));
    }
    let rt = runtime()?;
    rt.block_on(chat_async(
        provider, base_url, model, api_key, system, message,
    ))
}

async fn chat_async(
    provider: Option<String>,
    base_url: Option<String>,
    model: Option<String>,
    api_key: Option<String>,
    system: Option<String>,
    message: String,
) -> Result<String, StudioError> {
    let resolved = resolve(provider, base_url, model, api_key)?;
    let http = http_client()?;
    let mut messages: Vec<ChatMessage> = Vec::new();
    if let Some(s) = system {
        let s = s.trim();
        if !s.is_empty() {
            let clipped: String = s.chars().take(2000).collect();
            messages.push(ChatMessage::system(clipped));
        }
    }
    messages.push(ChatMessage::user(message));
    if resolved.kind == Provider::Anthropic {
        let key = resolved.api_key.clone().unwrap_or_default();
        let resp = http
            .post(format!("{}/v1/messages", resolved.base))
            .header("x-api-key", key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&anthropic_body(&resolved.model, &messages))
            .send()
            .await
            .map_err(|e| StudioError::Invalid(format!("llm.chat: {e}")))?;
        return decode("llm.chat", resp, anthropic_reply).await;
    }
    let mut req = http
        .post(format!("{}/chat/completions", resolved.base))
        .header("Content-Type", "application/json")
        .json(&openai_body(&resolved.model, &messages));
    if let Some(key) = resolved.api_key.clone() {
        req = req.bearer_auth(key);
    }
    let resp = req
        .send()
        .await
        .map_err(|e| StudioError::Invalid(format!("llm.chat: {e}")))?;
    decode("llm.chat", resp, openai_reply).await
}

/// Model list for pickers (OpenAI-compatible GET /v1/models + Ollama /api/tags).
pub fn models_blocking(
    provider: Option<String>,
    base_url: Option<String>,
    api_key: Option<String>,
) -> Result<Vec<String>, StudioError> {
    let rt = runtime()?;
    rt.block_on(models_async(provider, base_url, api_key))
}

async fn models_async(
    provider: Option<String>,
    base_url: Option<String>,
    api_key: Option<String>,
) -> Result<Vec<String>, StudioError> {
    let resolved = resolve(provider, base_url, Some("list".to_string()), api_key)?;
    if resolved.kind == Provider::Anthropic {
        return Err(StudioError::Invalid(
            "llm: Anthropic has no list-models endpoint - type the model id".to_string(),
        ));
    }
    let http = http_client()?;
    let mut req = http.get(format!("{}/models", resolved.base));
    if let Some(key) = resolved.api_key.clone() {
        req = req.bearer_auth(key);
    }
    let resp = req
        .send()
        .await
        .map_err(|e| StudioError::Invalid(format!("llm.models: {e}")))?;
    if !resp.status().is_success() {
        return Err(StudioError::Invalid(format!(
            "llm.models HTTP {}",
            resp.status()
        )));
    }
    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| StudioError::Invalid(format!("llm.models decode: {e}")))?;
    let mut names: Vec<String> = Vec::new();
    if let Some(data) = body.get("data").and_then(|v| v.as_array()) {
        for entry in data {
            if let Some(id) = entry.get("id").and_then(|v| v.as_str()) {
                names.push(id.to_string());
            }
        }
    }
    if let Some(models) = body.get("models").and_then(|v| v.as_array()) {
        for entry in models {
            if let Some(name) = entry.get("name").and_then(|v| v.as_str()) {
                names.push(name.to_string());
            }
        }
    }
    // Fallback: Ollama native /api/tags when /v1/models is empty.
    if names.is_empty() {
        let tag_url = resolved.base.replace("/v1", "/api/tags");
        if let Ok(resp) = http.get(tag_url).send().await {
            if let Ok(tag_body) = resp.json::<serde_json::Value>().await {
                if let Some(models) = tag_body.get("models").and_then(|v| v.as_array()) {
                    for entry in models {
                        if let Some(name) = entry.get("name").and_then(|v| v.as_str()) {
                            names.push(name.to_string());
                        }
                    }
                }
            }
        }
    }
    names.sort();
    names.dedup();
    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_parse_aliases() {
        assert!(matches!(
            Provider::parse("LM-Studio"),
            Ok(Provider::LmStudio)
        ));
        assert!(matches!(Provider::parse("claude"), Ok(Provider::Anthropic)));
        assert!(Provider::parse("llama-home-server").is_err());
    }

    #[test]
    fn openai_body_shape() {
        let body = openai_body("m", &[ChatMessage::user("hi")]);
        assert_eq!(body["model"], "m");
        assert_eq!(body["messages"][0]["role"], "user");
        assert_eq!(body["stream"], false);
    }

    #[test]
    fn reply_extractors_reject_garbage() {
        assert!(openai_reply(&serde_json::json!({})).is_err());
        assert!(anthropic_reply(&serde_json::json!({"content": []})).is_err());
    }
}
