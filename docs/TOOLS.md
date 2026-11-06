# TOOLS - StudioCraft v0.1

| Tool | Op | Notes |
|---|---|---|
| `studio_status` | `status` | version, contract, engine probes (`down` when missing) |
| `studio_caps` | `adapter_caps` | editable photo+vector, preview light/design, fleet-owned host |
| `studio_init` | `project_init` | `path + name` creates manifest v1 + subdirs |
| `studio_open` | `project_open` | validates `contract == studiocraft/v1` |
| `studio_handoff_check` | `handoff_check` | `project/from/to + unsaved/unsupported`, allow/block + reason |

Dotted aliases (`studio.status`, ...) accepted. CLI verbs mirror tools 1:1
via `studiocraft_core::dispatch`.
