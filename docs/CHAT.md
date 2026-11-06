# CHAT - StudioCraft LLM setup (Fleet standard card)

Providers: ollama (`http://127.0.0.1:11434/v1`, default `gemma4:12b`),
lmstudio (`http://127.0.0.1:1234/v1`), openai (`https://api.openai.com/v1`,
default `gpt-4o-mini`), anthropic (`https://api.anthropic.com`,
default `claude-sonnet-4-20250514`), custom (base required).

- Chat view: provider picker, editable base + model, Models button (fills picker
  from live `/v1/models` or Ollama `/api/tags`), Send (single non-streaming turn).
- Keys: explicit field > `LLM_API_KEY` > `OPENAI_API_KEY`/`ANTHROPIC_API_KEY`.
  Memory/env only. Never written to manifest, ui.json, or disk.
- Local 27B models think slowly: 150s timeout on chat, 30s on models.
- Anthropic has no list endpoint: type the model id.
- Grounding recipe: system = caps + handoff JSON (trimmed to 2000 chars),
  message = your question. The reply advises; `handoff_check` decides.

CLI:

```powershell
cargo run -q -p studiocraft-mcp -- llm-models --provider ollama
cargo run -q -p studiocraft-mcp -- llm-chat --message "Ping" --provider ollama
```
