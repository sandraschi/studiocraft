# StudioCraft

![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)
![Rust 1.90+](https://img.shields.io/badge/rust-1.90%2B-orange?logo=rust)
![MCP stdio](https://img.shields.io/badge/MCP-stdio-purple)
![ArtCraft harness](https://img.shields.io/badge/ArtCraft-harness-green)

**Shared project + universal canvas host (Pure-Rust ArtCraft, v0.2.0)**

One `.studiocraft` project, natives preserved both ways. v0.2 is a real studio:
7-view egui host (Projects / Handoff / Adapters / Chat / Skills / Help / Logs),
12 MCP tools + CLI verbs over one `dispatch`, 6 guided skills, Fleet LLM card,
contract + workflows + chat docs.

- Photo <-> Vector editable (native). Light = settings + LUT preview.
  Design/Pdf/Word/Deck/Grid/Effect/Film/Cad/Sound = companions (pointer JSON, v1).
- Failed handoff / cancel / unsaved / unsupported / companion-target blocks with
  reason. Never flattens. Revision is `checked_add` bump after verified trip.
- Storytold engines stay official (`storytold-ref`). StudioCraft is `fleet-owned`.
  No forks. Never edit `D:\Dev\repos\_upstream\craft-launcher`.

## Quickstart

```powershell
just check
just test
cargo run -q -p studiocraft-mcp -- status
cargo run -q -p studiocraft-mcp -- caps
cargo run -q -p studiocraft-mcp -- skills
cargo run -q -p studiocraft-mcp -- init --path D:/tmp/demo.studiocraft --name demo
cargo run -q -p studiocraft-mcp -- open --path D:/tmp/demo.studiocraft
cargo run -q -p studiocraft-mcp -- handoff-check --project D:/tmp/demo.studiocraft --from photo --to vector
cargo run -q -p studiocraft-mcp -- llm-models --provider ollama
cargo run -q -p studiocraft-app -- --smoke
```

MCP stdio: `studiocraft-cli mcp` (`initialize` -> `tools/list` -> `tools/call`).
Dotted aliases accepted (`studio.status`, ...).

## Layout

- `crates/studiocraft-core` L1 `dispatch` (project, adapters, handoff, skills, workflows, llm)
- `crates/studiocraft-mcp` L2 CLI + stdio (12 tools)
- `crates/studiocraft-app` L3 egui host (in-process, `--smoke`, no ports)
- `docs/` PRD + TOOLS + PARITY + UI_PARITY + CONTRACT + WORKFLOWS + SKILLS + CHAT
- `installer/installer.nsi` current-user NSIS, no admin
