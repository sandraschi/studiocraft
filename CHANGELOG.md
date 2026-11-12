# CHANGELOG - StudioCraft

## [0.2.0] - 2026-11-12
- Real studio: 7-view L3 (Projects/Handoff/Adapters/Chat/Skills/Help/Logs), harsh dark visuals, Ctrl+Wheel zoom, glyph allowlist test.
- 12 MCP tools + CLI verbs: + project_list, companion_attach, composite_bump, skills_list, skill_show, llm_chat, llm_models.
- 6 guided skills, workflows (round-trip/blocked/companion/chat-grounded/hygiene), contract v1 + chat docs.
- Fleet LLM card: ollama/lmstudio/openai/anthropic/custom, live models picker, memory-only keys, 150s chat timeout.
- Repo-level launcher: start.ps1/start.bat open the native app by default, with --smoke and --mcp switches.
- Release scaffold: justfile gates, scripts/release.ps1 (gates + smoke + NSIS + SHA256SUMS, publish opt-in), per-user installer.
- BookCraft-compiled brochure (HTML/EPUB/PDF) and PRD + tiered plan (Craft Menu Bar Standard, Help Suite).

## [0.1.0] - 2026-11-05
- Tier 1 scaffold: workspace + clippy deny gates + Command dispatch + CLI --json + MCP stdio + L3 parity test.
- Contract `studiocraft/v1`: photo<->vector editable, rest preview-linked, block-never-flatten handoff gate.
- 5 tools: studio_status, studio_caps, studio_init, studio_open, studio_handoff_check.
