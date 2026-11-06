# UI_PARITY - StudioCraft L3

- L3 `StudioApp` calls `studiocraft_core::dispatch` directly (no child process, no port, no JSON-RPC hop).
- `--smoke` runs: status ok, parity (app == core version/contract), caps == 2 editable, photo->vector allowed.
- Test `app_dispatch_matches_core_dispatch` fails the build if GUI drifts from core.
