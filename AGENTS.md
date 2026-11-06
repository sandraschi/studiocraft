# StudioCraft - AGENTS.md

**Parent hub:** [`mcp-central-docs/standards/RUST_ARTCRAFT_STANDARD.md`](../mcp-central-docs/standards/RUST_ARTCRAFT_STANDARD.md)
**Containment:** [`mcp-central-docs/standards/rust-artcraft/CONTAINMENT_AND_CRATES.md`](../mcp-central-docs/standards/rust-artcraft/CONTAINMENT_AND_CRATES.md)
**Fleet hub:** [`mcp-central-docs/projects/craft-fleet/README.md`](../mcp-central-docs/projects/craft-fleet/README.md)
**Origin:** fleet-owned (new). Upstream reference: `D:\Dev\repos\_upstream\craft-launcher` (read-only, never edit).

StudioCraft is a **Pure-Rust ArtCraft craft**: one shared project with a universal
canvas host. It does not fork the launcher and does not reimplement Photo/Vector.
Storytold engines stay official; StudioCraft orchestrates native files they already
read/write plus our own fleet crafts behind the same adapter shape. Bucket A
(files + CLI + headless), so Python, FastMCP, Starlette, React, Vite and
port-claiming standards do **not** apply here.

## Layout

| Crate | Tier | Contents |
|---|---|---|
| `crates/studiocraft-core` | L1 | `project.rs` (.studiocraft manifest v1), `adapters.rs` (caps registry), `handoff.rs` (safety gate), `dispatch` |
| `crates/studiocraft-mcp` | L2 | `studiocraft-cli` - clap CLI + hand-rolled MCP 2024-11-05 stdio loop |
| `crates/studiocraft-app` | L3 | `studiocraft-app` - eframe/egui host calling `dispatch` in-process, `--smoke` headless checks |

There is an L3 app. It must never spawn `studiocraft-cli.exe`, never open a port,
and never route GUI actions through JSON-RPC. Every view calls
`StudioApp::dispatch`, and `app_dispatch_matches_core_dispatch` proves the GUI
and core payloads are identical.

## Non-negotiable rules

1. **Zero panic.** `unwrap_used`, `expect_used`, `panic`, `todo`, `unimplemented`
   are denied at workspace level and repeated at every crate root.
   Return `Result<T, StudioError>`.
2. **Never flatten silently.** A handoff that cannot preserve identity/names/
   ordering/visibility/transforms blocks with `success: false` and a reason.
   No JPEG/PNG export path may overwrite a native project dir.
3. **Explicit safety.** `handoff_check` refuses when `unsaved=true` or
   `unsupported=true`, when `from == to`, and when either side is not in the v1
   editable set (`photo`, `vector`). Companions are preview-linked in v1.
4. **Stdio invariant.** In `mcp` mode, `stdout` is 100% line-delimited JSON-RPC.
   Logs go to `stderr`. One `println!` in L1 breaks the handshake.
5. **One dispatch bus.** CLI verbs and MCP tools must both construct arguments
   for `studiocraft_core::dispatch`. Do not add a capability to only one surface.
6. **No silent downgrade.** A missing Photo/Vector engine is `status: down`,
   not faked. A missing manifest is `error_type: invalid`, not an empty project.
7. **Hostile input.** Cap input sizes (<= 4096 chars for names/paths), validate
   contract string equals `studiocraft/v1`, never index strings by byte, use
   `checked_add` for revision math, lexical `within()` for root confinement.
8. **Upstream hygiene.** Never edit `D:\Dev\repos\_upstream\craft-launcher`.
   Official `storytold/*` sources only; no personal forks.

## Multi-agent rules

- Check `git status`; do not commit fleet-wide by accident.
- Max 10 files per commit and per script invocation. 3+ file edits need a
  timestamped `.bak` per file first (`*.bak` is gitignored).
- Do not start two `cargo check` runs in this workspace; wait for the peer lock.
- Shared `CARGO_TARGET_DIR` recommended; never delete another agent's lock.

## Verification (must pass before claiming done)

```powershell
just fmt-check
just check
just test
```

Then the MCP probe: pipe `initialize`, `tools/list` and one `tools/call` into
`studiocraft-cli mcp` and confirm every stdout line parses as JSON and `stderr`
holds only diagnostics. A gate that reports success having examined nothing is
a failure, not a pass.
