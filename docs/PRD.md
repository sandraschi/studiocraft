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
