# WORKFLOWS - StudioCraft v0.2

## A. Photo <-> Vector round-trip (core)
1. Projects view: Init `D:/tmp/demo.studiocraft` or Open existing.
2. Handoff view: Check photo->vector (must be allowed:true).
3. Edit vectors/ natively in VectorCraft.
4. Handoff view: Check vector->photo (must be allowed:true).
5. Projects view: Open to verify, Bump revision.

## B. Blocked handoff (safe)
- Read `reason`. Unsaved: save/discard in source, re-check.
- Unsupported: remove/convert flagged content, re-check.
- Companion as target in v1: keep as linked preview via companion_attach.

## C. Companion link
- Projects view: companion file + kind (design|pdf|word|deck|grid|effect|film|cad|sound) -> Attach.
- Edit in native editor. Composite stays preview. Open again to verify pointer.

## D. Chat-grounded handoff
- Adapters view: copy caps. Handoff view: run the check, copy JSON.
- Chat view: provider ollama base http://127.0.0.1:11434/v1 model gemma4:12b,
  system = caps + handoff JSON (trimmed), message = your question.
- Chat advises, gate decides. Never apply what allowed:false forbids.

## E. Hygiene
- Open before every session. Bump only after both directions check green.
- List root to confirm scan. Keep outputs outside painting/vectors/.
