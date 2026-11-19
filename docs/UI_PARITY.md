# UI_PARITY - StudioCraft L3

Pattern 2 workstation. The window is the shared-project host, not a second paint app.

- Top bar: `File` `Edit` `View` `Insert` `Modify` `Commands` `Window` `Help`, with `switch_on_hover`.
- `Help` opens five dialogs: Command Palette, Keyboard Shortcuts, Project Files & Docs, Licenses & Notices, About StudioCraft.
- Left sidebar `Tasks` lists the six registered skills and runs them through `dispatch`.
- Center stage stays up: project path, revision, painting / vectors / composite lanes.
- Right dock: handoff gate, companion pointer, adapter status, chat. Bottom: session log.
- L3 calls `studiocraft_core::dispatch` in-process. No child process, no port, no JSON-RPC hop.
- `--smoke` checks the menu titles, the five help items, task ids against `skills_list`, status parity, editable caps, and photo->vector allow.
- Test `app_dispatch_matches_core_dispatch` fails the build if GUI dispatch drifts from core.
