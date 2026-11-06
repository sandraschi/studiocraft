# CONTRACT - StudioCraft v1 (`studiocraft/v1`)

One `.studiocraft` dir:

```
<project>.studiocraft/
  manifest.json   contract, project, revision, editors, preserved, composite
  painting/       native Photo raster layers
  vectors/        native Vector objects (id-stable smart layer)
  companions/     <kind>.json pointers (linked preview, v1)
  composite.png   rev-cached preview for companions only (not authoritative)
```

Rules:

- Natives are authoritative. Composite never overwrites natives.
- v1 editable set is photo<->vector. Everything else is a companion target
  and blocks with `allowed:false` + reason.
- Handoff blocks on: unsaved=true, unsupported=true, from==to, non-editable side.
- Save/reopen keeps natives. Revision is `checked_add(1)` via composite_bump
  after a verified round-trip, never before.
- Missing engine = `status: down`. Missing manifest = `not_found`, not empty.
- Keys (LLM) live in memory/env only, never in manifest or ui.json.
