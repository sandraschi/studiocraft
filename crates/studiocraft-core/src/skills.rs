//! Skill registry for StudioCraft: guided workflows, not stubs.
//!
//! Each skill is a real recipe over `dispatch` ops with explicit safety notes.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use serde_json::{json, Value};

fn skill(id: &str, title: &str, origin: &str, uses: &str, steps: &[&str], safety: &str) -> Value {
    json!({
        "id": id,
        "title": title,
        "origin": origin,
        "uses": uses,
        "steps": steps,
        "safety": safety,
    })
}

pub fn all() -> Vec<Value> {
    vec![
        skill(
            "photo-vector-roundtrip",
            "Photo <-> Vector safe round-trip",
            "fleet-owned",
            "Paint, add/edit vectors, return to painting with natives intact.",
            &[
                "1. studio_init path=<dir> name=<name> (or studio_open an existing project)",
                "2. studio_handoff_check project=<dir> from=photo to=vector (must be allowed:true)",
                "3. Do the vector edit in VectorCraft against vectors/ (native, id-stable)",
                "4. studio_handoff_check project=<dir> from=vector to=photo (must be allowed:true)",
                "5. studio_open path=<dir> to verify manifest + preserved list",
            ],
            "Blocked handoffs never flatten. Resolve unsaved/unsupported first, then retry the check.",
        ),
        skill(
            "safe-handoff-gate",
            "Read a blocked handoff without losing work",
            "fleet-owned",
            "Turn allowed:false into a fix list.",
            &[
                "1. Read reason: unsaved pending vs unsupported content vs non-editable target",
                "2. Unsaved: save or discard in the source editor, re-run the check",
                "3. Unsupported: remove or convert the flagged content, re-run the check",
                "4. Non-editable (companions in v1): keep it as linked preview, do not force it as a target",
            ],
            "No export path may overwrite a native project dir.",
        ),
        skill(
            "companion-link",
            "Attach a Design/Pdf/Word/Deck companion as linked preview",
            "fleet-owned",
            "Keep native companion docs beside the shared painting.",
            &[
                "1. studio_open path=<dir> to confirm contract studiocraft/v1",
                "2. companion_attach project=<dir> path=<file> kind=design|pdf|word|deck|grid",
                "3. Edit the companion in its native editor; composite.png stays preview only",
                "4. studio_open again to verify the pointer survived",
            ],
            "Companion edits never become Photo layers in v1. Pointer file only, no binary copy.",
        ),
        skill(
            "rescue-blocked",
            "Recover after a failed handoff",
            "fleet-owned",
            "Unstick a project after allowed:false without losing work.",
            &[
                "1. studio_open path=<dir> (manifest must still validate)",
                "2. studio_handoff_check with unsaved=false unsupported=false to confirm the gate is green",
                "3. If still blocked, read the reason and apply safe-handoff-gate",
                "4. composite_bump project=<dir> only after both directions check allowed:true",
            ],
            "Never bump revision while a direction is still blocked.",
        ),
        skill(
            "project-hygiene",
            "Keep a project saveable and reopenable",
            "fleet-owned",
            "Small dirs, valid manifest, explicit revision.",
            &[
                "1. studio_open path=<dir> before every session",
                "2. Keep painting/ vectors/ companions/ beside manifest.json (no stray outputs inside)",
                "3. composite_bump project=<dir> after a verified round-trip, not before",
                "4. project_list root=<dir> to confirm it still scans",
            ],
            "Revision uses checked_add; overflow or missing manifest is an error, not a silent reset.",
        ),
        skill(
            "studio-chat-recipe",
            "Ask the local LLM about a handoff with project context",
            "fleet-owned",
            "Grounded chat: caps + handoff state in the system prompt.",
            &[
                "1. studio_caps to get the v1 editable set",
                "2. studio_handoff_check for the direction you plan",
                "3. llm_chat with system=<caps + handoff JSON, trimmed> message=<your question>",
                "4. Apply only what the gate allows; the chat never overrides allowed:false",
            ],
            "Chat advises, the gate decides. Keys stay in memory/env, never on disk.",
        ),
    ]
}

pub fn list() -> Value {
    json!({ "count": all().len(), "skills": all() })
}

pub fn show(id: &str) -> Option<Value> {
    all()
        .into_iter()
        .find(|s| s.get("id").and_then(Value::as_str) == Some(id))
}
