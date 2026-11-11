# PRD - StudioCraft v0.1

## Problem
Paint in PhotoCraft, edit vectors in VectorCraft, return to painting without
export/import/reconcile. From `storytold/craft-launcher#1233`.

## v1 scope (Tier 1)
- One `.studiocraft` project: `manifest.json (contract studiocraft/v1)` +
  `painting/ + vectors/ + companions/ + composite.png (rev-cached preview)`.
- Editable: photo<->vector only. Preserved: identity, names, ordering,
  visibility, transforms.
- Safety: unsaved / unsupported / from==to / non-editable target blocks with
  explicit reason. Save/reopen keeps natives. No global undo in v1.
- Provenance: storytold-ref engines + fleet-owned host, same adapter shape so
  our own crafts plug in later.

## Non-goals v1
12-app editable canvas, RAW/HDR/EXR, crop/mask adapters, global undo,
release/installer flows for the suite.

## Fit test
Single model (shared project). Headless verbs (status/caps/init/open/handoff).
Files in/out (.studiocraft dir). 3 recipes: init+open, photo->vector clean,
vector->photo blocked-on-unsaved.

## UI/UX Architecture & Menu Bar Standard

In accordance with [`CRAFT_MENU_BAR_STANDARD.md`](../../mcp-central-docs/standards/rust-artcraft/CRAFT_MENU_BAR_STANDARD.md):
- **Pattern 2 (Viewport-Centric Studio Workstation):** Persistent multi-craft project canvas integrating painting, vector, and composite viewports. Top quick button row acts as accelerators into the `View` menu. To add a new studio adapter or handoff tool:
  1. Register a visibility toggle in the `View` or `Window` menu.
  2. Add an entry in the sidebar Craft Handoff / Layer toolbox.
  3. Wire the action directly into the universal `Command::dispatch` bus.
- **Top Menu Bar Hierarchy:**
  - `File`: New Studio Project (`Ctrl+N`), Open `.studiocraft` Project (`Ctrl+O`), Open Recent ▸, Save Project (`Ctrl+S`), Export Composite (`Ctrl+E`), Close Project (`Ctrl+W`), Quit (`Ctrl+Q`).
  - `Edit`: Undo (`Ctrl+Z`), Redo (`Ctrl+Y`), Cut, Copy, Paste, Select All, Preferences (`Ctrl+,`).
  - `View`: Unified Studio Canvas, PhotoCraft Layer Viewport, VectorCraft Vector Viewport, Composite Preview, Theme (`Dark`/`Light`).
  - `Insert`: Add Painting Layer, Add Vector Layer, Import Asset Companion, Link External Craft Asset.
  - `Modify`: Transform Layer, Reorder Layers, Update Layer Visibility, Reconcile Manifest.
  - `Commands`: Handoff to PhotoCraft, Handoff to VectorCraft, Synthesize Composite, Validate Project Schema.
  - `Window`: Reset Layout, Canvas Viewport, Layers Inspector, Handoff Status Dock.
  - `Help` (Mandatory 5-Dialog Suite): `Command Palette...` (`Ctrl+K`), `Keyboard Shortcuts...` (`F1`), `Project Files & Docs...`, `Licenses & Notices...`, `About StudioCraft`.
- **Behavioral Mechanics:** Smooth `switch_on_hover` between top-level menus, zero-unsafe immediate-mode rendering, and headless `--smoke` parity.
