# PARITY - StudioCraft honesty log

- v0.1 proves the contract + safety gate headless. No live Photo/Vector engine
  driving yet; `engine_probe` reports `down` when not on PATH (never faked).
- Composite refresh is manifest-level in v1 (no 4MP PNG writer yet).
- L3 calls L1 in-process; `app_dispatch_matches_core_dispatch` locks it.
- Not claimed: 12-app canvas, RAW/HDR, global undo, suite installer.
