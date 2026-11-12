# studiocraft v0.2.0

Shared `.studiocraft` project with a universal canvas host. One project, natives
preserved both ways, nothing flattened. Pure-Rust ArtCraft, Windows x64.

## Downloads

- `studiocraft.exe`: the desktop window (L3 egui host, in-process dispatch, no ports).
- `studiocraft-cli.exe`: CLI and MCP stdio server (`studiocraft-cli mcp`).
- `studiocraft-0.2.0-setup.exe`: per-user installer for both (no admin rights).
- `studiocraft-0.2.0-windows-x64-portable.zip`: launcher-only extra (the same two exes).

## What is new

- Real studio, not a scaffold: 7 views (Projects, Handoff, Adapters, Chat, Skills,
  Help, Logs), harsh dark visuals, Ctrl+Wheel zoom, and a glyph allowlist test that
  fails on tofu.
- 12 MCP tools plus matching CLI verbs over one `dispatch` bus: `project_list`,
  `companion_attach`, `composite_bump`, `skills_list`, `skill_show`, `llm_chat` and
  `llm_models` join the original five.
- 6 guided skills, documented workflows (round-trip, blocked, companion,
  chat-grounded, hygiene), and contract `studiocraft/v1` plus chat docs.
- Fleet LLM card: ollama, lmstudio, openai, anthropic and custom providers, a live
  models picker, memory-only keys, 150 s chat timeout.
- Repo-level launcher: `start.ps1` / `start.bat` open the native desktop app by
  default, with `--smoke` and `--mcp` switches.
- Release scaffolding: `justfile` gates, `scripts/release.ps1` (gates, release build,
  smoke, artifacts, SHA256SUMS; publish is opt-in), and a per-user NSIS installer.
- BookCraft-compiled brochure (HTML, EPUB, PDF) and the PRD plus tiered plan,
  including the Craft Menu Bar Standard and Help Suite.

## Notes

- Handoff is photo <-> vector only. Light is settings + LUT preview; Design, Pdf,
  Word, Deck, Grid, Effect, Film, Cad and Sound are preview-linked companions.
- A handoff that cannot preserve identity, names, ordering, visibility or transforms
  blocks with `success: false` and a reason. It never flattens.
- A missing Photo/Vector engine reports `status: down`; it is never faked.
- Verification this release: `cargo fmt --check`, clippy `-D warnings` and the full
  test suite green; app `--smoke`, CLI `status` and the MCP `initialize` handshake
  all passed from the built release binaries.
