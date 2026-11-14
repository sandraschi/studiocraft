//! StudioApp: 7-view studio host proving GUI/MCP parity in-process.
//!
//! Views: Projects | Handoff | Adapters | Chat | Skills | Help | Logs.
//! Every action flows through StudioApp::app_dispatch (same core dispatch).
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use eframe::egui;
use serde_json::{json, Value};
use studiocraft_core::dispatch;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum View {
    Projects,
    Handoff,
    Adapters,
    Chat,
    Skills,
    Help,
    Logs,
}

impl View {
    fn all() -> [View; 7] {
        [
            View::Projects,
            View::Handoff,
            View::Adapters,
            View::Chat,
            View::Skills,
            View::Help,
            View::Logs,
        ]
    }
    fn label(self) -> &'static str {
        match self {
            View::Projects => "Projects",
            View::Handoff => "Handoff",
            View::Adapters => "Adapters",
            View::Chat => "Chat",
            View::Skills => "Skills",
            View::Help => "Help",
            View::Logs => "Logs",
        }
    }
}

fn harsh_visuals() -> egui::Visuals {
    let mut v = egui::Visuals::dark();
    let ink = egui::Color32::from_rgb(242, 246, 252);
    v.override_text_color = Some(ink);
    v.weak_text_color = Some(egui::Color32::from_rgb(203, 213, 225));
    v.panel_fill = egui::Color32::from_rgb(10, 13, 18);
    v.extreme_bg_color = egui::Color32::from_rgb(10, 13, 18);
    v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, ink);
    v.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, ink);
    v.widgets.hovered.fg_stroke = egui::Stroke::new(1.5, egui::Color32::WHITE);
    v.widgets.active.fg_stroke = egui::Stroke::new(1.5, egui::Color32::WHITE);
    v.widgets.noninteractive.bg_stroke =
        egui::Stroke::new(1.0, egui::Color32::from_rgb(38, 54, 84));
    v.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(48, 66, 102));
    v
}

fn handle_zoom(ctx: &egui::Context) {
    let scroll_y = ctx.input(|i| {
        if i.modifiers.ctrl || i.modifiers.command {
            i.smooth_scroll_delta.y
        } else {
            0.0
        }
    });
    if scroll_y.abs() > 0.0 {
        let step = if scroll_y > 0.0 {
            1.1_f32
        } else {
            1.0 / 1.1_f32
        };
        let next = (ctx.zoom_factor() * step).clamp(0.6, 2.5);
        ctx.set_zoom_factor(next);
    }
}

pub struct StudioApp {
    view: View,
    status: Value,
    caps: Value,
    skills: Value,
    log: Vec<String>,
    // projects
    scan_root: String,
    new_path: String,
    new_name: String,
    project_path: String,
    last_open: String,
    // handoff
    hf_from: String,
    hf_to: String,
    hf_unsaved: bool,
    hf_unsupported: bool,
    hf_result: String,
    // chat + LLM settings (memory only, never on disk)
    llm_provider: String,
    llm_base: String,
    llm_model: String,
    llm_key: String,
    llm_models: Vec<String>,
    llm_system: String,
    llm_message: String,
    llm_reply: String,
    // skills
    skill_id: String,
    skill_body: String,
    // companion
    comp_path: String,
    comp_kind: String,
}

impl StudioApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(harsh_visuals());
        let status =
            dispatch("status", &json!({})).unwrap_or_else(|e| json!({"error": e.to_string()}));
        let caps = dispatch("adapter_caps", &json!({}))
            .unwrap_or_else(|e| json!({"error": e.to_string()}));
        let skills =
            dispatch("skills_list", &json!({})).unwrap_or_else(|e| json!({"error": e.to_string()}));
        Self {
            view: View::Projects,
            status,
            caps,
            skills,
            log: vec!["studiocraft v0.2 ready: photo<->vector editable, rest preview-linked".to_string()],
            scan_root: "D:/tmp".to_string(),
            new_path: "D:/tmp/demo.studiocraft".to_string(),
            new_name: "demo".to_string(),
            project_path: "D:/tmp/demo.studiocraft".to_string(),
            last_open: String::new(),
            hf_from: "photo".to_string(),
            hf_to: "vector".to_string(),
            hf_unsaved: false,
            hf_unsupported: false,
            hf_result: String::new(),
            llm_provider: "ollama".to_string(),
            llm_base: "http://127.0.0.1:11434/v1".to_string(),
            llm_model: "gemma4:12b".to_string(),
            llm_key: String::new(),
            llm_models: Vec::new(),
            llm_system: "You advise on StudioCraft handoffs. The gate decides; chat never overrides allowed:false.".to_string(),
            llm_message: "How do I do a safe photo->vector round-trip?".to_string(),
            llm_reply: String::new(),
            skill_id: "photo-vector-roundtrip".to_string(),
            skill_body: String::new(),
            comp_path: String::new(),
            comp_kind: "design".to_string(),
        }
    }

    pub fn app_dispatch(op: &str, args: &Value) -> Result<Value, String> {
        dispatch(op, args).map_err(|e| e.to_string())
    }

    fn push_log(&mut self, line: String) {
        self.log.push(line);
        while self.log.len() > 200 {
            self.log.remove(0);
        }
    }
}

impl eframe::App for StudioApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        handle_zoom(ui.ctx());
        egui::Panel::top("top").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("StudioCraft");
                ui.label("shared project host - v0.2");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!(
                        "contract {}",
                        self.status
                            .get("contract")
                            .and_then(Value::as_str)
                            .unwrap_or("unknown")
                    ));
                });
            });
            ui.horizontal(|ui| {
                for view in View::all() {
                    ui.selectable_value(&mut self.view, view, view.label());
                }
            });
        });
        egui::CentralPanel::default().show(ui, |ui| match self.view {
            View::Projects => self.show_projects(ui),
            View::Handoff => self.show_handoff(ui),
            View::Adapters => self.show_adapters(ui),
            View::Chat => self.show_chat(ui),
            View::Skills => self.show_skills(ui),
            View::Help => self.show_help(ui),
            View::Logs => self.show_logs(ui),
        });
        egui::Panel::bottom("statusbar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("{} log lines", self.log.len()));
                ui.label("keys: memory/env only, never on disk");
            });
        });
    }
}

impl StudioApp {
    fn show_projects(&mut self, ui: &mut egui::Ui) {
        ui.heading("Projects");
        ui.horizontal(|ui| {
            ui.label("new path:");
            ui.text_edit_singleline(&mut self.new_path);
            ui.label("name:");
            ui.text_edit_singleline(&mut self.new_name);
            if ui.button("Init").clicked() {
                let args = json!({"path": self.new_path, "name": self.new_name});
                match Self::app_dispatch("project_init", &args) {
                    Ok(v) => {
                        self.push_log(format!("init ok: {}", v["path"]));
                        self.last_open = v.to_string();
                    }
                    Err(e) => self.push_log(format!("init error: {e}")),
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("open path:");
            ui.text_edit_singleline(&mut self.project_path);
            if ui.button("Open").clicked() {
                let args = json!({"path": self.project_path});
                match Self::app_dispatch("project_open", &args) {
                    Ok(v) => {
                        self.push_log(format!("open ok rev={}", v["manifest"]["revision"]));
                        self.last_open = v.to_string();
                    }
                    Err(e) => self.push_log(format!("open error: {e}")),
                }
            }
            if ui.button("Bump revision").clicked() {
                let args = json!({"project": self.project_path});
                match Self::app_dispatch("composite_bump", &args) {
                    Ok(v) => self.push_log(format!("bump rev={}", v["revision"])),
                    Err(e) => self.push_log(format!("bump error: {e}")),
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("scan root:");
            ui.text_edit_singleline(&mut self.scan_root);
            if ui.button("List").clicked() {
                let args = json!({"root": self.scan_root});
                match Self::app_dispatch("project_list", &args) {
                    Ok(v) => self.push_log(format!("list: {}", v)),
                    Err(e) => self.push_log(format!("list error: {e}")),
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("companion file:");
            ui.text_edit_singleline(&mut self.comp_path);
            ui.label("kind:");
            ui.text_edit_singleline(&mut self.comp_kind);
            if ui.button("Attach").clicked() {
                let args =
                    json!({"project": self.project_path, "path": self.comp_path, "kind": self.comp_kind});
                match Self::app_dispatch("companion_attach", &args) {
                    Ok(v) => self.push_log(format!("attach ok: {}", v["pointer"])),
                    Err(e) => self.push_log(format!("attach error: {e}")),
                }
            }
        });
        if !self.last_open.is_empty() {
            ui.separator();
            ui.label("last result:");
            ui.code(&self.last_open);
        }
    }

    fn show_handoff(&mut self, ui: &mut egui::Ui) {
        ui.heading("Handoff gate (block, never flatten)");
        ui.horizontal(|ui| {
            ui.label("from:");
            ui.text_edit_singleline(&mut self.hf_from);
            ui.label("to:");
            ui.text_edit_singleline(&mut self.hf_to);
            ui.checkbox(&mut self.hf_unsaved, "unsaved");
            ui.checkbox(&mut self.hf_unsupported, "unsupported");
            if ui.button("Check").clicked() {
                let args = json!({
                    "project": self.project_path,
                    "from": self.hf_from,
                    "to": self.hf_to,
                    "unsaved": self.hf_unsaved,
                    "unsupported": self.hf_unsupported,
                });
                match Self::app_dispatch("handoff_check", &args) {
                    Ok(v) => {
                        self.hf_result = v.to_string();
                        self.push_log(format!(
                            "handoff {}->{} allowed={}",
                            self.hf_from, self.hf_to, v["allowed"]
                        ));
                    }
                    Err(e) => {
                        self.hf_result = format!("error: {e}");
                        self.push_log(format!("handoff error: {e}"));
                    }
                }
            }
        });
        ui.label("v1 editable: photo <-> vector. Companions are preview-linked.");
        if !self.hf_result.is_empty() {
            ui.code(&self.hf_result);
        }
    }

    fn show_adapters(&mut self, ui: &mut egui::Ui) {
        ui.heading("Adapters (wrap, do not reimplement)");
        ui.label("Storytold engines stay official. Missing engine = down, never faked.");
        ui.code(self.caps.to_string());
        ui.separator();
        ui.label("Engine probes:");
        ui.code(self.status.to_string());
    }

    fn show_chat(&mut self, ui: &mut egui::Ui) {
        ui.heading("Chat (local-first LLM)");
        ui.horizontal(|ui| {
            ui.label("provider:");
            egui::ComboBox::from_id_salt("provider")
                .selected_text(&self.llm_provider)
                .show_ui(ui, |ui| {
                    for p in ["ollama", "lmstudio", "openai", "anthropic", "custom"] {
                        ui.selectable_value(&mut self.llm_provider, p.to_string(), p);
                    }
                });
            ui.label("base:");
            ui.text_edit_singleline(&mut self.llm_base);
        });
        ui.horizontal(|ui| {
            ui.label("model:");
            ui.text_edit_singleline(&mut self.llm_model);
            if ui.button("Models").clicked() {
                let args = json!({"provider": self.llm_provider, "base_url": self.llm_base});
                match Self::app_dispatch("llm_models", &args) {
                    Ok(v) => {
                        self.llm_models = v
                            .get("models")
                            .and_then(Value::as_array)
                            .map(|a| {
                                a.iter()
                                    .filter_map(Value::as_str)
                                    .map(str::to_string)
                                    .collect()
                            })
                            .unwrap_or_default();
                        self.push_log(format!("models: {} found", self.llm_models.len()));
                    }
                    Err(e) => self.push_log(format!("models error: {e}")),
                }
            }
            if !self.llm_models.is_empty() {
                let mut pick = self.llm_model.clone();
                egui::ComboBox::from_id_salt("model")
                    .selected_text(&self.llm_model)
                    .show_ui(ui, |ui| {
                        for m in &self.llm_models {
                            ui.selectable_value(&mut pick, m.clone(), m);
                        }
                    });
                self.llm_model = pick;
            }
        });
        ui.horizontal(|ui| {
            ui.label("api key (memory only):");
            ui.text_edit_singleline(&mut self.llm_key);
            ui.label("cloud needs key; local does not");
        });
        ui.label("system:");
        ui.text_edit_multiline(&mut self.llm_system);
        ui.label("message:");
        ui.text_edit_multiline(&mut self.llm_message);
        if ui.button("Send (may take a while on local 27B)").clicked() {
            let args = json!({
                "provider": self.llm_provider,
                "base_url": self.llm_base,
                "model": self.llm_model,
                "api_key": if self.llm_key.trim().is_empty() { Value::Null } else { json!(self.llm_key) },
                "system": self.llm_system,
                "message": self.llm_message,
            });
            // Strip the null key so core falls back to env (avoids sending empty key).
            let mut obj = args.as_object().cloned().unwrap_or_default();
            if self.llm_key.trim().is_empty() {
                obj.remove("api_key");
            }
            match Self::app_dispatch("llm_chat", &Value::Object(obj)) {
                Ok(v) => {
                    self.llm_reply = v
                        .get("reply")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string();
                    self.push_log("chat: reply ok".to_string());
                }
                Err(e) => {
                    self.llm_reply = format!("error: {e}");
                    self.push_log(format!("chat error: {e}"));
                }
            }
        }
        if !self.llm_reply.is_empty() {
            ui.separator();
            ui.label("reply:");
            ui.code(&self.llm_reply);
        }
    }

    fn show_skills(&mut self, ui: &mut egui::Ui) {
        ui.heading("Skills (guided, real recipes)");
        ui.horizontal(|ui| {
            ui.label("skill id:");
            ui.text_edit_singleline(&mut self.skill_id);
            if ui.button("Show").clicked() {
                let args = json!({"id": self.skill_id});
                match Self::app_dispatch("skill_show", &args) {
                    Ok(v) => {
                        self.skill_body = v.to_string();
                        self.push_log(format!("skill: {}", v["title"]));
                    }
                    Err(e) => self.push_log(format!("skill error: {e}")),
                }
            }
        });
        ui.code(self.skills.to_string());
        if !self.skill_body.is_empty() {
            ui.separator();
            ui.code(&self.skill_body);
        }
    }

    fn show_help(&mut self, ui: &mut egui::Ui) {
        ui.heading("Help (contract + safety)");
        ui.label("One project, natives preserved. Block instead of flattening.");
        ui.code(include_str!("../../../docs/CONTRACT.md"));
        ui.separator();
        ui.code(include_str!("../../../docs/WORKFLOWS.md"));
    }

    fn show_logs(&mut self, ui: &mut egui::Ui) {
        ui.heading("Logs (this session, capped at 200)");
        egui::ScrollArea::vertical().show(ui, |ui| {
            for line in &self.log {
                ui.label(line);
            }
        });
        if ui.button("Clear").clicked() {
            self.log.clear();
        }
    }
}

/// Headless parity + smoke checks for CI (no window).
pub fn run_smoke_checks() -> Result<Vec<String>, String> {
    let mut lines: Vec<String> = Vec::new();
    let status = dispatch("status", &json!({})).map_err(|e| e.to_string())?;
    if status
        .get("success")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        lines.push("status: ok".to_string());
    } else {
        return Err("status missing success".to_string());
    }
    let via_app = StudioApp::app_dispatch("status", &json!({}))?;
    if via_app.get("version") == status.get("version") {
        lines.push("parity: app_dispatch == core dispatch".to_string());
    } else {
        return Err("parity mismatch".to_string());
    }
    let caps = dispatch("adapter_caps", &json!({})).map_err(|e| e.to_string())?;
    let n = caps
        .get("editable")
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or(0);
    if n == 2 {
        lines.push("caps: photo+vector editable".to_string());
    } else {
        return Err("caps editable != 2".to_string());
    }
    let gate = dispatch(
        "handoff_check",
        &json!({"project": "D:/tmp/smoke", "from": "photo", "to": "vector"}),
    )
    .map_err(|e| e.to_string())?;
    if gate
        .get("allowed")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        lines.push("handoff: photo->vector allowed".to_string());
    } else {
        return Err("handoff photo->vector should allow".to_string());
    }
    let skills = dispatch("skills_list", &json!({})).map_err(|e| e.to_string())?;
    if skills.get("count").and_then(Value::as_i64).unwrap_or(0) >= 6 {
        lines.push("skills: 6 registered".to_string());
    } else {
        return Err("skills < 6".to_string());
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_dispatch_matches_core_dispatch() {
        let args = json!({});
        let a = StudioApp::app_dispatch("status", &args).unwrap_or_default();
        let b = dispatch("status", &args).unwrap_or_default();
        assert_eq!(a["version"], b["version"]);
        assert_eq!(a["contract"], b["contract"]);
    }

    #[test]
    fn glyph_allowlist_no_tofu() {
        // egui 0.37 default font lacks geometric/emoji glyphs (tofu boxes).
        // Keep UI ASCII + the verified safe set only.
        let src = include_str!("app.rs");
        for ch in src.chars() {
            if ch.is_ascii() {
                continue;
            }
            assert!(
                matches!(
                    ch,
                    '\u{00B0}' | '\u{00D7}' | '\u{00B7}' | '\u{2026}' | '\u{2014}'
                ),
                "non-allowlisted char {ch:?} in app.rs (tofu risk)"
            );
        }
    }
}
