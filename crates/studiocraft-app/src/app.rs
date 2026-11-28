//! Studio workstation: menu bar, task sidebar, project stage, help dialogs.
//!
//! Every project action goes through `StudioApp::app_dispatch` (core `dispatch`).
//! Layout toggles stay in the window. PhotoCraft and VectorCraft edit natives.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use eframe::egui;
use serde_json::{json, Value};
use studiocraft_core::dispatch;

const MENU_TITLES: [&str; 8] = [
    "File", "Edit", "View", "Insert", "Modify", "Commands", "Window", "Help",
];

const HELP_ITEMS: [&str; 5] = [
    "Command Palette...",
    "Keyboard Shortcuts...",
    "Project Files & Docs...",
    "Licenses & Notices...",
    "About StudioCraft",
];

const COMPANION_KINDS: &[&str] = &[
    "design", "pdf", "word", "deck", "grid", "effect", "film", "cad", "sound",
];

const TASKS: &[(&str, &str)] = &[
    ("photo-vector-roundtrip", "Photo <-> Vector safe round-trip"),
    (
        "safe-handoff-gate",
        "Read a blocked handoff without losing work",
    ),
    (
        "companion-link",
        "Attach a Design/Pdf/Word/Deck companion as linked preview",
    ),
    ("rescue-blocked", "Recover after a failed handoff"),
    ("project-hygiene", "Keep a project saveable and reopenable"),
    (
        "studio-chat-recipe",
        "Ask the local LLM about a handoff with project context",
    ),
];

struct Cmd {
    id: &'static str,
    menu: &'static str,
    label: &'static str,
}

const CMDS: &[Cmd] = &[
    Cmd {
        id: "file.new",
        menu: "File",
        label: "New Project",
    },
    Cmd {
        id: "file.open",
        menu: "File",
        label: "Open Project",
    },
    Cmd {
        id: "file.save",
        menu: "File",
        label: "Save",
    },
    Cmd {
        id: "file.export",
        menu: "File",
        label: "Export Composite Preview",
    },
    Cmd {
        id: "file.close",
        menu: "File",
        label: "Close Project",
    },
    Cmd {
        id: "file.quit",
        menu: "File",
        label: "Quit",
    },
    Cmd {
        id: "edit.undo",
        menu: "Edit",
        label: "Undo",
    },
    Cmd {
        id: "edit.redo",
        menu: "Edit",
        label: "Redo",
    },
    Cmd {
        id: "edit.cut",
        menu: "Edit",
        label: "Cut",
    },
    Cmd {
        id: "edit.copy",
        menu: "Edit",
        label: "Copy",
    },
    Cmd {
        id: "edit.paste",
        menu: "Edit",
        label: "Paste",
    },
    Cmd {
        id: "edit.select",
        menu: "Edit",
        label: "Select All",
    },
    Cmd {
        id: "edit.prefs",
        menu: "Edit",
        label: "Preferences...",
    },
    Cmd {
        id: "view.zoom_in",
        menu: "View",
        label: "Zoom In",
    },
    Cmd {
        id: "view.zoom_out",
        menu: "View",
        label: "Zoom Out",
    },
    Cmd {
        id: "view.fit",
        menu: "View",
        label: "Fit to Window",
    },
    Cmd {
        id: "view.actual",
        menu: "View",
        label: "Reset Zoom (100%)",
    },
    Cmd {
        id: "view.lanes",
        menu: "View",
        label: "Stage Lanes",
    },
    Cmd {
        id: "view.tasks",
        menu: "View",
        label: "Tasks Sidebar",
    },
    Cmd {
        id: "view.dark",
        menu: "View",
        label: "Dark",
    },
    Cmd {
        id: "view.light",
        menu: "View",
        label: "Light",
    },
    Cmd {
        id: "insert.attach",
        menu: "Insert",
        label: "Attach Companion",
    },
    Cmd {
        id: "insert.design",
        menu: "Insert",
        label: "Link design companion",
    },
    Cmd {
        id: "insert.pdf",
        menu: "Insert",
        label: "Link pdf companion",
    },
    Cmd {
        id: "insert.word",
        menu: "Insert",
        label: "Link word companion",
    },
    Cmd {
        id: "insert.deck",
        menu: "Insert",
        label: "Link deck companion",
    },
    Cmd {
        id: "insert.grid",
        menu: "Insert",
        label: "Link grid companion",
    },
    Cmd {
        id: "insert.effect",
        menu: "Insert",
        label: "Link effect companion",
    },
    Cmd {
        id: "insert.film",
        menu: "Insert",
        label: "Link film companion",
    },
    Cmd {
        id: "insert.cad",
        menu: "Insert",
        label: "Link cad companion",
    },
    Cmd {
        id: "insert.sound",
        menu: "Insert",
        label: "Link sound companion",
    },
    Cmd {
        id: "modify.reconcile",
        menu: "Modify",
        label: "Reconcile Manifest",
    },
    Cmd {
        id: "modify.bump",
        menu: "Modify",
        label: "Bump Revision",
    },
    Cmd {
        id: "cmd.to_vector",
        menu: "Commands",
        label: "Handoff to VectorCraft",
    },
    Cmd {
        id: "cmd.to_photo",
        menu: "Commands",
        label: "Handoff to PhotoCraft",
    },
    Cmd {
        id: "cmd.validate",
        menu: "Commands",
        label: "Validate Project",
    },
    Cmd {
        id: "cmd.list",
        menu: "Commands",
        label: "List Projects",
    },
    Cmd {
        id: "cmd.refresh",
        menu: "Commands",
        label: "Refresh Engine Status",
    },
    Cmd {
        id: "win.reset",
        menu: "Window",
        label: "Reset Panel Layout",
    },
    Cmd {
        id: "win.tasks",
        menu: "Window",
        label: "Tasks",
    },
    Cmd {
        id: "win.lanes",
        menu: "Window",
        label: "Stage Lanes",
    },
    Cmd {
        id: "win.handoff",
        menu: "Window",
        label: "Handoff Dock",
    },
    Cmd {
        id: "win.adapters",
        menu: "Window",
        label: "Adapters",
    },
    Cmd {
        id: "win.chat",
        menu: "Window",
        label: "Chat",
    },
    Cmd {
        id: "win.logs",
        menu: "Window",
        label: "Logs",
    },
    Cmd {
        id: "help.palette",
        menu: "Help",
        label: HELP_ITEMS[0],
    },
    Cmd {
        id: "help.keys",
        menu: "Help",
        label: HELP_ITEMS[1],
    },
    Cmd {
        id: "help.docs",
        menu: "Help",
        label: HELP_ITEMS[2],
    },
    Cmd {
        id: "help.licenses",
        menu: "Help",
        label: HELP_ITEMS[3],
    },
    Cmd {
        id: "help.about",
        menu: "Help",
        label: HELP_ITEMS[4],
    },
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dialog {
    Palette,
    Shortcuts,
    Docs,
    Licenses,
    About,
    Preferences,
}

struct Keys {
    escape: bool,
    quit: bool,
    new_project: bool,
    open: bool,
    save: bool,
    close: bool,
    palette: bool,
    help: bool,
    export: bool,
    undo: bool,
    redo: bool,
    cut: bool,
    copy: bool,
    paste: bool,
    select_all: bool,
    prefs: bool,
    zoom_in: bool,
    zoom_out: bool,
    zoom_fit: bool,
    zoom_actual: bool,
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

fn light_visuals() -> egui::Visuals {
    let mut v = egui::Visuals::light();
    let ink = egui::Color32::from_rgb(15, 23, 42);
    v.override_text_color = Some(ink);
    v.weak_text_color = Some(egui::Color32::from_rgb(51, 65, 85));
    v.panel_fill = egui::Color32::from_rgb(248, 250, 252);
    v.extreme_bg_color = egui::Color32::from_rgb(241, 245, 249);
    v
}

/// Hovering another top title while a menu is open switches to that title.
fn switch_on_hover(ctx: &egui::Context, titles: &[egui::Response]) {
    let is_open =
        |t: &egui::Response| egui::Popup::is_id_open(ctx, egui::Popup::default_response_id(t));
    if !titles.iter().any(&is_open) {
        return;
    }
    if let Some(title) = titles
        .iter()
        .find(|t| t.hovered() && !t.clicked() && !is_open(t))
    {
        egui::Popup::open_id(ctx, egui::Popup::default_response_id(title));
    }
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

fn poll_keys(ctx: &egui::Context) -> Keys {
    ctx.input(|i| {
        let cmd = i.modifiers.command;
        let shift = i.modifiers.shift;
        Keys {
            escape: i.key_pressed(egui::Key::Escape),
            quit: cmd && i.key_pressed(egui::Key::Q),
            new_project: cmd && i.key_pressed(egui::Key::N),
            open: cmd && i.key_pressed(egui::Key::O),
            save: cmd && i.key_pressed(egui::Key::S),
            close: cmd && i.key_pressed(egui::Key::W),
            palette: (cmd && i.key_pressed(egui::Key::K))
                || (cmd && shift && i.key_pressed(egui::Key::P)),
            help: i.key_pressed(egui::Key::F1),
            export: cmd && i.key_pressed(egui::Key::E),
            undo: cmd && !shift && i.key_pressed(egui::Key::Z),
            redo: cmd && i.key_pressed(egui::Key::Y),
            cut: cmd && i.key_pressed(egui::Key::X),
            copy: cmd && i.key_pressed(egui::Key::C),
            paste: cmd && i.key_pressed(egui::Key::V),
            select_all: cmd && i.key_pressed(egui::Key::A),
            prefs: cmd && i.key_pressed(egui::Key::Comma),
            zoom_in: cmd && (i.key_pressed(egui::Key::Equals) || i.key_pressed(egui::Key::Plus)),
            zoom_out: cmd && i.key_pressed(egui::Key::Minus),
            zoom_fit: cmd && i.key_pressed(egui::Key::Num0),
            zoom_actual: cmd && i.key_pressed(egui::Key::Num1),
        }
    })
}

fn is_allowed(v: &Value) -> bool {
    v.get("allowed").and_then(Value::as_bool).unwrap_or(false)
}

fn both_allowed(left: &Value, right: &Value) -> bool {
    is_allowed(left) && is_allowed(right)
}

fn json_str<'a>(v: &'a Value, key: &str) -> &'a str {
    v.get(key).and_then(Value::as_str).unwrap_or("")
}

fn engine_status(status: &Value, key: &str) -> String {
    status
        .get("engines")
        .and_then(|e| e.get(key))
        .and_then(|e| e.get("status"))
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .to_string()
}

fn clip_chars(text: &str, max: usize) -> String {
    let mut out = String::new();
    for (i, ch) in text.chars().enumerate() {
        if i >= max {
            out.push_str("...");
            break;
        }
        out.push(ch);
    }
    out
}

fn task_rows(skills: &Value) -> Vec<(String, String)> {
    let mut rows = Vec::new();
    if let Some(arr) = skills.get("skills").and_then(Value::as_array) {
        for skill in arr {
            let id = json_str(skill, "id");
            if id.is_empty() {
                continue;
            }
            let title = skill.get("title").and_then(Value::as_str).unwrap_or(id);
            rows.push((id.to_string(), title.to_string()));
        }
    }
    if rows.is_empty() {
        for (id, title) in TASKS {
            rows.push(((*id).to_string(), (*title).to_string()));
        }
    }
    rows
}

fn shell_check() -> Result<(), String> {
    if MENU_TITLES.len() != 8 {
        return Err("menu bar is not 8 titles".into());
    }
    for title in MENU_TITLES {
        if !CMDS.iter().any(|cmd| cmd.menu == title) {
            return Err(format!("menu {title} has no commands"));
        }
    }
    let help: Vec<&str> = CMDS
        .iter()
        .filter(|cmd| cmd.menu == "Help")
        .map(|cmd| cmd.label)
        .collect();
    if help != HELP_ITEMS {
        return Err(format!("help suite drifted: {help:?}"));
    }
    let skills = dispatch("skills_list", &json!({})).map_err(|e| e.to_string())?;
    let mut ids = Vec::new();
    if let Some(arr) = skills.get("skills").and_then(Value::as_array) {
        for skill in arr {
            ids.push(json_str(skill, "id").to_string());
        }
    }
    let want: Vec<String> = TASKS.iter().map(|(id, _)| (*id).to_string()).collect();
    if ids != want {
        return Err(format!(
            "task sidebar drifted from skills: {ids:?} vs {want:?}"
        ));
    }
    Ok(())
}

fn menu_item(ui: &mut egui::Ui, label: &str) -> bool {
    let clicked = ui.button(label).clicked();
    if clicked {
        ui.close();
    }
    clicked
}

pub struct StudioApp {
    dialog: Option<Dialog>,
    show_tasks: bool,
    show_lanes: bool,
    show_handoff: bool,
    show_logs: bool,
    show_adapters: bool,
    show_chat: bool,
    theme_dark: bool,
    status: Value,
    caps: Value,
    skills: Value,
    log: Vec<String>,
    status_line: String,
    scan_root: String,
    new_path: String,
    new_name: String,
    project_path: String,
    loaded: Value,
    recent: Vec<String>,
    clipboard: String,
    palette_query: String,
    hf_from: String,
    hf_to: String,
    hf_unsaved: bool,
    hf_unsupported: bool,
    hf_result: Value,
    llm_provider: String,
    llm_base: String,
    llm_model: String,
    llm_key: String,
    llm_models: Vec<String>,
    llm_system: String,
    llm_message: String,
    llm_reply: String,
    active_task: String,
    task_detail: Value,
    comp_path: String,
    comp_kind: String,
    last_list: Value,
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
            dialog: None,
            show_tasks: true,
            show_lanes: true,
            show_handoff: true,
            show_logs: true,
            show_adapters: true,
            show_chat: false,
            theme_dark: true,
            status,
            caps,
            skills,
            log: vec!["StudioCraft ready. Shared project host, photo <-> vector editable.".into()],
            status_line: "Shared project host. PhotoCraft and VectorCraft edit the natives.".into(),
            scan_root: "D:/tmp".into(),
            new_path: "D:/tmp/demo.studiocraft".into(),
            new_name: "demo".into(),
            project_path: "D:/tmp/demo.studiocraft".into(),
            loaded: Value::Null,
            recent: Vec::new(),
            clipboard: String::new(),
            palette_query: String::new(),
            hf_from: "photo".into(),
            hf_to: "vector".into(),
            hf_unsaved: false,
            hf_unsupported: false,
            hf_result: Value::Null,
            llm_provider: "ollama".into(),
            llm_base: "http://127.0.0.1:11434/v1".into(),
            llm_model: "gemma4:12b".into(),
            llm_key: String::new(),
            llm_models: Vec::new(),
            llm_system: "You advise on StudioCraft handoffs. The gate decides; chat never overrides allowed:false.".into(),
            llm_message: "How do I do a safe photo to vector round-trip?".into(),
            llm_reply: String::new(),
            active_task: String::new(),
            task_detail: Value::Null,
            comp_path: String::new(),
            comp_kind: "design".into(),
            last_list: Value::Null,
        }
    }

    pub fn app_dispatch(op: &str, args: &Value) -> Result<Value, String> {
        dispatch(op, args).map_err(|e| e.to_string())
    }

    fn note(&mut self, line: impl Into<String>) {
        self.status_line = line.into();
    }

    fn push_log(&mut self, line: String) {
        self.log.push(line);
        while self.log.len() > 200 {
            self.log.remove(0);
        }
    }

    fn remember(&mut self, path: &str) {
        if path.is_empty() {
            return;
        }
        self.recent.retain(|p| p != path);
        self.recent.insert(0, path.to_string());
        self.recent.truncate(8);
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        if self.theme_dark {
            ctx.set_visuals(harsh_visuals());
        } else {
            ctx.set_visuals(light_visuals());
        }
    }

    fn zoom_by(&self, ctx: &egui::Context, step: f32) {
        let next = (ctx.zoom_factor() * step).clamp(0.6, 2.5);
        ctx.set_zoom_factor(next);
    }

    fn reset_layout(&mut self) {
        self.show_tasks = true;
        self.show_lanes = true;
        self.show_handoff = true;
        self.show_logs = true;
        self.show_adapters = true;
        self.show_chat = false;
        self.note("Panel layout reset");
    }

    fn act_new(&mut self) {
        let args = json!({"path": self.new_path, "name": self.new_name});
        match Self::app_dispatch("project_init", &args) {
            Ok(v) => {
                self.project_path.clone_from(&self.new_path);
                self.remember(&self.project_path.clone());
                let shown = json_str(&v, "path").to_string();
                self.loaded = v;
                self.push_log(format!("init ok: {shown}"));
                self.note(format!("Created {shown}"));
            }
            Err(e) => {
                self.push_log(format!("init error: {e}"));
                self.note(format!("Create failed: {e}"));
            }
        }
    }

    fn act_open(&mut self) {
        let args = json!({"path": self.project_path});
        match Self::app_dispatch("project_open", &args) {
            Ok(v) => {
                let rev = v
                    .get("manifest")
                    .and_then(|m| m.get("revision"))
                    .and_then(Value::as_i64)
                    .unwrap_or(0);
                self.remember(&self.project_path.clone());
                self.loaded = v;
                self.push_log(format!("open ok rev={rev}"));
                self.note(format!("Opened {} at revision {rev}", self.project_path));
            }
            Err(e) => {
                self.push_log(format!("open error: {e}"));
                self.note(format!("Open failed: {e}"));
            }
        }
    }

    fn act_close(&mut self) {
        self.loaded = Value::Null;
        self.note("Closed project. The folder on disk is unchanged.");
        self.push_log("close project".into());
    }

    fn act_export(&mut self) {
        let root = self.project_path.trim_end_matches(['/', '\\']);
        self.note(format!(
            "Composite preview is {root}/composite.png. StudioCraft does not flatten natives or overwrite the project."
        ));
    }

    fn act_list(&mut self) {
        let args = json!({"root": self.scan_root});
        match Self::app_dispatch("project_list", &args) {
            Ok(v) => {
                let n = v.get("count").and_then(Value::as_i64).unwrap_or(0);
                self.last_list = v;
                self.push_log(format!("list count={n}"));
                self.note(format!("Found {n} projects under {}", self.scan_root));
            }
            Err(e) => {
                self.push_log(format!("list error: {e}"));
                self.note(format!("List failed: {e}"));
            }
        }
    }

    fn gate(&mut self, from: &str, to: &str) -> Result<Value, String> {
        self.hf_from = from.to_string();
        self.hf_to = to.to_string();
        let args = json!({
            "project": self.project_path,
            "from": from,
            "to": to,
            "unsaved": self.hf_unsaved,
            "unsupported": self.hf_unsupported,
        });
        match Self::app_dispatch("handoff_check", &args) {
            Ok(v) => {
                let allowed = is_allowed(&v);
                self.hf_result = v.clone();
                self.push_log(format!("handoff {from} -> {to} allowed={allowed}"));
                self.note(format!("Handoff {from} -> {to} allowed={allowed}"));
                Ok(v)
            }
            Err(e) => {
                self.push_log(format!("handoff error: {e}"));
                self.note(format!("Handoff failed: {e}"));
                Err(e)
            }
        }
    }

    fn act_handoff(&mut self, from: &str, to: &str) {
        self.show_handoff = true;
        let _ = self.gate(from, to);
    }

    fn act_bump(&mut self) {
        let left = self.gate("photo", "vector");
        let right = self.gate("vector", "photo");
        let (Ok(left), Ok(right)) = (left, right) else {
            self.note("Bump blocked: a handoff check failed");
            return;
        };
        if !both_allowed(&left, &right) {
            self.hf_result = json!({"photo_to_vector": left, "vector_to_photo": right});
            self.note("Bump blocked until photo -> vector and vector -> photo are both allowed");
            self.push_log("bump blocked".into());
            return;
        }
        let args = json!({"project": self.project_path});
        match Self::app_dispatch("composite_bump", &args) {
            Ok(v) => {
                let rev = v.get("revision").and_then(Value::as_i64).unwrap_or(0);
                self.push_log(format!("bump rev={rev}"));
                self.act_open();
                self.note(format!("Revision is now {rev}"));
            }
            Err(e) => {
                self.push_log(format!("bump error: {e}"));
                self.note(format!("Bump failed: {e}"));
            }
        }
    }

    fn act_attach(&mut self) {
        if self.comp_path.trim().is_empty() {
            self.show_handoff = true;
            self.note("Set the companion file path in the handoff dock, then attach.");
            return;
        }
        let args = json!({
            "project": self.project_path,
            "path": self.comp_path,
            "kind": self.comp_kind,
        });
        match Self::app_dispatch("companion_attach", &args) {
            Ok(v) => {
                let pointer = json_str(&v, "pointer").to_string();
                self.push_log(format!("attach ok: {pointer}"));
                self.note(format!("Linked {} companion at {pointer}", self.comp_kind));
            }
            Err(e) => {
                self.push_log(format!("attach error: {e}"));
                self.note(format!("Attach failed: {e}"));
            }
        }
    }

    fn attach_kind(&mut self, kind: &str) {
        self.comp_kind = kind.to_string();
        self.show_handoff = true;
        self.act_attach();
    }

    fn act_refresh(&mut self) {
        self.status =
            Self::app_dispatch("status", &json!({})).unwrap_or_else(|e| json!({"error": e}));
        self.caps =
            Self::app_dispatch("adapter_caps", &json!({})).unwrap_or_else(|e| json!({"error": e}));
        self.skills =
            Self::app_dispatch("skills_list", &json!({})).unwrap_or_else(|e| json!({"error": e}));
        self.note("Refreshed status, adapters, and tasks");
        self.push_log("refresh".into());
    }

    fn select_task(&mut self, id: &str) {
        self.active_task = id.to_string();
        match Self::app_dispatch("skill_show", &json!({"id": id})) {
            Ok(v) => {
                let title = json_str(&v, "title").to_string();
                self.task_detail = v;
                self.note(format!("Task: {title}"));
            }
            Err(e) => {
                self.task_detail = Value::Null;
                self.note(format!("Task failed: {e}"));
            }
        }
    }

    fn run_task_action(&mut self) {
        match self.active_task.as_str() {
            "photo-vector-roundtrip" => {
                self.act_open();
                self.act_handoff("photo", "vector");
            }
            "safe-handoff-gate" => {
                let from = self.hf_from.clone();
                let to = self.hf_to.clone();
                self.act_handoff(&from, &to);
            }
            "companion-link" => self.act_attach(),
            "rescue-blocked" => {
                self.hf_unsaved = false;
                self.hf_unsupported = false;
                self.act_open();
                let _ = self.gate("photo", "vector");
                let _ = self.gate("vector", "photo");
            }
            "project-hygiene" => {
                self.act_open();
                self.act_list();
            }
            "studio-chat-recipe" => {
                self.show_chat = true;
                self.show_handoff = true;
                let caps = clip_chars(&self.caps.to_string(), 800);
                let gate = clip_chars(&self.hf_result.to_string(), 800);
                self.llm_system = format!(
                    "Caps: {caps}\nGate: {gate}\nThe gate decides. Chat never overrides allowed:false."
                );
                self.note("Chat dock open. System prompt filled from caps and the last gate.");
            }
            _ => self.note("Pick a task in the sidebar first"),
        }
    }

    fn act_models(&mut self) {
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
                self.note(format!("Models: {} found", self.llm_models.len()));
                self.push_log(format!("models: {}", self.llm_models.len()));
            }
            Err(e) => {
                self.push_log(format!("models error: {e}"));
                self.note(format!("Models failed: {e}"));
            }
        }
    }

    fn act_chat(&mut self) {
        let mut args = json!({
            "provider": self.llm_provider,
            "base_url": self.llm_base,
            "model": self.llm_model,
            "system": self.llm_system,
            "message": self.llm_message,
        });
        if !self.llm_key.trim().is_empty() {
            if let Some(obj) = args.as_object_mut() {
                obj.insert("api_key".into(), json!(self.llm_key));
            }
        }
        match Self::app_dispatch("llm_chat", &args) {
            Ok(v) => {
                self.llm_reply = json_str(&v, "reply").to_string();
                self.push_log("chat: reply ok".into());
                self.note("Chat reply received. The gate still decides.");
            }
            Err(e) => {
                self.llm_reply = format!("error: {e}");
                self.push_log(format!("chat error: {e}"));
                self.note(format!("Chat failed: {e}"));
            }
        }
    }

    fn edit_copy(&mut self, ctx: &egui::Context) {
        self.clipboard.clone_from(&self.project_path);
        ctx.copy_text(self.clipboard.clone());
        self.note("Copied project path");
    }

    fn edit_cut(&mut self, ctx: &egui::Context) {
        self.edit_copy(ctx);
        self.project_path.clear();
        self.loaded = Value::Null;
        self.note("Cut project path. Folder on disk is unchanged.");
    }

    fn edit_paste(&mut self) {
        if self.clipboard.trim().is_empty() {
            self.note("Nothing copied yet. Copy a project path first.");
            return;
        }
        self.project_path.clone_from(&self.clipboard);
        self.note("Pasted project path. Open Project to load it.");
    }

    fn edit_select(&mut self, ctx: &egui::Context) {
        let rev = self
            .loaded
            .get("manifest")
            .and_then(|m| m.get("revision"))
            .and_then(Value::as_i64)
            .unwrap_or(0);
        let text = format!("{} rev={rev}", self.project_path);
        self.clipboard.clone_from(&text);
        ctx.copy_text(text);
        self.note("Copied project path and revision");
    }

    fn run_cmd(&mut self, ctx: &egui::Context, id: &str) -> bool {
        if let Some(kind) = id.strip_prefix("insert.") {
            if COMPANION_KINDS.contains(&kind) {
                self.attach_kind(kind);
                return true;
            }
        }
        match id {
            "file.new" => self.act_new(),
            "file.open" | "file.save" | "modify.reconcile" | "cmd.validate" => self.act_open(),
            "file.export" => self.act_export(),
            "file.close" => self.act_close(),
            "file.quit" => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            "edit.undo" | "edit.redo" => {
                self.note(
                    "v1 has no global undo. PhotoCraft and VectorCraft keep their own history.",
                );
            }
            "edit.cut" => self.edit_cut(ctx),
            "edit.copy" => self.edit_copy(ctx),
            "edit.paste" => self.edit_paste(),
            "edit.select" => self.edit_select(ctx),
            "edit.prefs" => self.dialog = Some(Dialog::Preferences),
            "view.zoom_in" => self.zoom_by(ctx, 1.1),
            "view.zoom_out" => self.zoom_by(ctx, 1.0 / 1.1),
            "view.fit" | "view.actual" => ctx.set_zoom_factor(1.0),
            "view.lanes" | "win.lanes" => self.show_lanes = !self.show_lanes,
            "view.tasks" | "win.tasks" => self.show_tasks = !self.show_tasks,
            "view.dark" => {
                self.theme_dark = true;
                self.apply_theme(ctx);
            }
            "view.light" => {
                self.theme_dark = false;
                self.apply_theme(ctx);
            }
            "insert.attach" => self.act_attach(),
            "modify.bump" => self.act_bump(),
            "cmd.to_vector" => self.act_handoff("photo", "vector"),
            "cmd.to_photo" => self.act_handoff("vector", "photo"),
            "cmd.list" => self.act_list(),
            "cmd.refresh" => self.act_refresh(),
            "win.reset" => self.reset_layout(),
            "win.handoff" => self.show_handoff = !self.show_handoff,
            "win.adapters" => self.show_adapters = !self.show_adapters,
            "win.chat" => self.show_chat = !self.show_chat,
            "win.logs" => self.show_logs = !self.show_logs,
            "help.palette" => self.dialog = Some(Dialog::Palette),
            "help.keys" => self.dialog = Some(Dialog::Shortcuts),
            "help.docs" => self.dialog = Some(Dialog::Docs),
            "help.licenses" => self.dialog = Some(Dialog::Licenses),
            "help.about" => self.dialog = Some(Dialog::About),
            _ => return false,
        }
        true
    }

    fn handle_keys(&mut self, ctx: &egui::Context) {
        let typing = ctx.egui_wants_keyboard_input();
        let keys = poll_keys(ctx);
        if keys.escape && self.dialog.is_some() {
            self.dialog = None;
            return;
        }
        if keys.palette {
            self.dialog = if self.dialog == Some(Dialog::Palette) {
                None
            } else {
                Some(Dialog::Palette)
            };
            return;
        }
        if keys.quit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }
        if keys.zoom_in {
            self.zoom_by(ctx, 1.1);
        }
        if keys.zoom_out {
            self.zoom_by(ctx, 1.0 / 1.1);
        }
        if keys.zoom_fit || keys.zoom_actual {
            ctx.set_zoom_factor(1.0);
        }
        if self.dialog.is_some() {
            return;
        }
        if keys.help {
            self.dialog = Some(Dialog::Shortcuts);
        }
        if keys.prefs && !typing {
            self.dialog = Some(Dialog::Preferences);
        }
        if keys.new_project {
            self.act_new();
        }
        if keys.open {
            self.act_open();
        }
        if keys.save {
            self.act_open();
        }
        if keys.close {
            self.act_close();
        }
        if keys.export && !typing {
            self.act_export();
        }
        if keys.undo {
            self.note("v1 has no global undo. PhotoCraft and VectorCraft keep their own history.");
        }
        if keys.redo {
            self.note("v1 has no global undo. PhotoCraft and VectorCraft keep their own history.");
        }
        if typing {
            return;
        }
        if keys.cut {
            self.edit_cut(ctx);
        }
        if keys.copy {
            self.edit_copy(ctx);
        }
        if keys.paste {
            self.edit_paste();
        }
        if keys.select_all {
            self.edit_select(ctx);
        }
    }

    fn file_menu(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        if menu_item(ui, "New Project (Ctrl+N)") {
            self.run_cmd(&ctx, "file.new");
        }
        if menu_item(ui, "Open Project... (Ctrl+O)") {
            self.run_cmd(&ctx, "file.open");
        }
        ui.menu_button("Open Recent", |ui| {
            if self.recent.is_empty() {
                ui.label("(empty)");
            } else {
                let paths = self.recent.clone();
                for path in paths {
                    if ui.button(&path).clicked() {
                        self.project_path = path;
                        self.act_open();
                        ui.close();
                    }
                }
                ui.separator();
                if menu_item(ui, "Clear Recent Files") {
                    self.recent.clear();
                    self.note("Cleared recent projects");
                }
            }
        });
        ui.separator();
        if menu_item(ui, "Save (Ctrl+S)") {
            self.run_cmd(&ctx, "file.save");
        }
        ui.add_enabled(false, egui::Button::new("Save As..."))
            .on_disabled_hover_text(
                "The .studiocraft folder is the project. Copy that folder yourself.",
            );
        ui.separator();
        if menu_item(ui, "Export Composite Preview (Ctrl+E)") {
            self.run_cmd(&ctx, "file.export");
        }
        ui.separator();
        if menu_item(ui, "Close Project (Ctrl+W)") {
            self.run_cmd(&ctx, "file.close");
        }
        if menu_item(ui, "Quit (Ctrl+Q)") {
            self.run_cmd(&ctx, "file.quit");
        }
    }

    fn edit_menu(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        if menu_item(ui, "Undo (Ctrl+Z)") {
            self.run_cmd(&ctx, "edit.undo");
        }
        if menu_item(ui, "Redo (Ctrl+Y)") {
            self.run_cmd(&ctx, "edit.redo");
        }
        ui.separator();
        if menu_item(ui, "Cut (Ctrl+X)") {
            self.run_cmd(&ctx, "edit.cut");
        }
        if menu_item(ui, "Copy (Ctrl+C)") {
            self.run_cmd(&ctx, "edit.copy");
        }
        if menu_item(ui, "Paste (Ctrl+V)") {
            self.run_cmd(&ctx, "edit.paste");
        }
        if menu_item(ui, "Select All (Ctrl+A)") {
            self.run_cmd(&ctx, "edit.select");
        }
        ui.separator();
        if menu_item(ui, "Preferences... (Ctrl+,)") {
            self.run_cmd(&ctx, "edit.prefs");
        }
    }

    fn view_menu(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        if menu_item(ui, "Zoom In (Ctrl+=)") {
            self.run_cmd(&ctx, "view.zoom_in");
        }
        if menu_item(ui, "Zoom Out (Ctrl+-)") {
            self.run_cmd(&ctx, "view.zoom_out");
        }
        if menu_item(ui, "Fit to Window (Ctrl+0)") {
            self.run_cmd(&ctx, "view.fit");
        }
        if menu_item(ui, "Reset Zoom (100%) (Ctrl+1)") {
            self.run_cmd(&ctx, "view.actual");
        }
        ui.separator();
        ui.checkbox(&mut self.show_lanes, "Stage Lanes");
        ui.checkbox(&mut self.show_tasks, "Tasks Sidebar");
        ui.separator();
        ui.menu_button("Interface Theme", |ui| {
            if menu_item(ui, "Dark") {
                self.theme_dark = true;
                let ctx = ui.ctx().clone();
                self.apply_theme(&ctx);
            }
            if menu_item(ui, "Light") {
                self.theme_dark = false;
                let ctx = ui.ctx().clone();
                self.apply_theme(&ctx);
            }
        });
    }

    fn insert_menu(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        if menu_item(ui, "Attach Companion...") {
            self.run_cmd(&ctx, "insert.attach");
        }
        ui.separator();
        for kind in COMPANION_KINDS {
            if menu_item(ui, &format!("Link {kind} companion")) {
                self.attach_kind(kind);
            }
        }
    }

    fn modify_menu(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        if menu_item(ui, "Reconcile Manifest") {
            self.run_cmd(&ctx, "modify.reconcile");
        }
        if menu_item(ui, "Bump Revision") {
            self.run_cmd(&ctx, "modify.bump");
        }
    }

    fn commands_menu(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        if menu_item(ui, "Handoff to VectorCraft") {
            self.run_cmd(&ctx, "cmd.to_vector");
        }
        if menu_item(ui, "Handoff to PhotoCraft") {
            self.run_cmd(&ctx, "cmd.to_photo");
        }
        ui.separator();
        if menu_item(ui, "Validate Project") {
            self.run_cmd(&ctx, "cmd.validate");
        }
        if menu_item(ui, "List Projects") {
            self.run_cmd(&ctx, "cmd.list");
        }
        if menu_item(ui, "Refresh Engine Status") {
            self.run_cmd(&ctx, "cmd.refresh");
        }
    }

    fn window_menu(&mut self, ui: &mut egui::Ui) {
        if menu_item(ui, "Reset Panel Layout") {
            self.reset_layout();
        }
        ui.separator();
        ui.checkbox(&mut self.show_tasks, "Tasks");
        ui.checkbox(&mut self.show_lanes, "Stage Lanes");
        ui.checkbox(&mut self.show_handoff, "Handoff Dock");
        ui.checkbox(&mut self.show_adapters, "Adapters");
        ui.checkbox(&mut self.show_chat, "Chat");
        ui.checkbox(&mut self.show_logs, "Logs");
    }

    fn help_menu(&mut self, ui: &mut egui::Ui) {
        if menu_item(ui, "Command Palette... (Ctrl+K)") {
            self.dialog = Some(Dialog::Palette);
        }
        if menu_item(ui, "Keyboard Shortcuts... (F1)") {
            self.dialog = Some(Dialog::Shortcuts);
        }
        if menu_item(ui, "Project Files & Docs...") {
            self.dialog = Some(Dialog::Docs);
        }
        if menu_item(ui, "Licenses & Notices...") {
            self.dialog = Some(Dialog::Licenses);
        }
        ui.separator();
        if menu_item(ui, "About StudioCraft") {
            self.dialog = Some(Dialog::About);
        }
    }

    fn tasks_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Tasks");
        ui.label("Studio jobs. Pick one, then run the gate.");
        let rows = task_rows(&self.skills);
        for (id, title) in rows {
            let on = self.active_task == id;
            if ui.selectable_label(on, title).clicked() {
                self.select_task(&id);
            }
        }
        if self.task_detail.get("id").is_some() {
            ui.separator();
            ui.label(egui::RichText::new(json_str(&self.task_detail, "title")).strong());
            let uses = json_str(&self.task_detail, "uses");
            if !uses.is_empty() {
                ui.label(uses);
            }
            if let Some(steps) = self.task_detail.get("steps").and_then(Value::as_array) {
                for step in steps {
                    if let Some(text) = step.as_str() {
                        ui.label(text);
                    }
                }
            }
            let safety = json_str(&self.task_detail, "safety");
            if !safety.is_empty() {
                ui.label(safety);
            }
            if ui.button("Run this task").clicked() {
                self.run_task_action();
            }
        }
    }

    fn stage(&mut self, ui: &mut egui::Ui) {
        ui.heading("Project stage");
        ui.label("One folder. PhotoCraft writes painting/. VectorCraft writes vectors/. This window checks the handoff and links companions. It does not paint or flatten.");
        ui.separator();
        ui.label("New project path");
        ui.text_edit_singleline(&mut self.new_path);
        ui.label("Project name");
        ui.text_edit_singleline(&mut self.new_name);
        if ui.button("Create project").clicked() {
            self.act_new();
        }
        ui.label("Open path");
        ui.text_edit_singleline(&mut self.project_path);
        ui.horizontal(|ui| {
            if ui.button("Open").clicked() {
                self.act_open();
            }
            if ui.button("Reload").clicked() {
                self.act_open();
            }
            if ui.button("Close").clicked() {
                self.act_close();
            }
        });
        if let Some(manifest) = self.loaded.get("manifest").cloned() {
            let name = json_str(&manifest, "project");
            let rev = manifest
                .get("revision")
                .and_then(Value::as_i64)
                .unwrap_or(0);
            let contract = json_str(&manifest, "contract");
            ui.separator();
            ui.heading(name);
            ui.label(format!("revision {rev}"));
            ui.label(format!("contract {contract}"));
        }
        if self.show_lanes {
            ui.separator();
            let photo = engine_status(&self.status, "photo");
            let vector = engine_status(&self.status, "vector");
            let light = engine_status(&self.status, "light");
            ui.columns(3, |cols| {
                cols[0].group(|ui| {
                    ui.label(egui::RichText::new("Painting").strong());
                    ui.label(format!("PhotoCraft engine: {photo}"));
                    ui.label("Native raster layers live in painting/.");
                });
                cols[1].group(|ui| {
                    ui.label(egui::RichText::new("Vectors").strong());
                    ui.label(format!("VectorCraft engine: {vector}"));
                    ui.label("Id-stable objects live in vectors/.");
                });
                cols[2].group(|ui| {
                    ui.label(egui::RichText::new("Composite").strong());
                    ui.label(format!("LightCraft engine: {light}"));
                    ui.label("composite.png is a preview for companions, not the document.");
                });
            });
        }
        if !self.hf_result.is_null() {
            ui.separator();
            ui.label("Last handoff");
            self.show_gate(ui, &self.hf_result.clone());
        }
        ui.separator();
        ui.label("Scan root");
        ui.text_edit_singleline(&mut self.scan_root);
        if ui.button("List projects").clicked() {
            self.act_list();
        }
        if let Some(projects) = self
            .last_list
            .get("projects")
            .and_then(Value::as_array)
            .cloned()
        {
            for project in projects {
                let path = json_str(&project, "path").to_string();
                let name = json_str(&project, "project");
                let rev = project.get("revision").and_then(Value::as_i64).unwrap_or(0);
                if path.is_empty() {
                    continue;
                }
                if ui.button(format!("{name}  rev {rev}  {path}")).clicked() {
                    self.project_path.clone_from(&path);
                    self.act_open();
                }
            }
        }
    }

    fn show_gate(&self, ui: &mut egui::Ui, v: &Value) {
        if let Some(flag) = v.get("allowed").and_then(Value::as_bool) {
            ui.label(format!("allowed: {flag}"));
        }
        let reason = json_str(v, "reason");
        if !reason.is_empty() {
            ui.label(reason);
        }
        for (key, label) in [
            ("photo_to_vector", "photo -> vector"),
            ("vector_to_photo", "vector -> photo"),
        ] {
            if let Some(child) = v.get(key) {
                let flag = is_allowed(child);
                ui.label(format!("{label}: allowed={flag}"));
            }
        }
        ui.collapsing("Gate JSON", |ui| {
            let text = serde_json::to_string_pretty(v).unwrap_or_else(|_| v.to_string());
            ui.code(text);
        });
    }

    fn inspector(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            if self.show_handoff {
                ui.heading("Handoff");
                ui.label("Block instead of flattening.");
                ui.label("From");
                ui.text_edit_singleline(&mut self.hf_from);
                ui.label("To");
                ui.text_edit_singleline(&mut self.hf_to);
                ui.checkbox(&mut self.hf_unsaved, "unsaved");
                ui.checkbox(&mut self.hf_unsupported, "unsupported");
                if ui.button("Check handoff").clicked() {
                    let from = self.hf_from.clone();
                    let to = self.hf_to.clone();
                    self.act_handoff(&from, &to);
                }
                ui.separator();
                ui.label("Companion file");
                ui.text_edit_singleline(&mut self.comp_path);
                ui.label("Kind");
                egui::ComboBox::from_id_salt("kind")
                    .selected_text(&self.comp_kind)
                    .show_ui(ui, |ui| {
                        for kind in COMPANION_KINDS {
                            ui.selectable_value(&mut self.comp_kind, (*kind).to_string(), *kind);
                        }
                    });
                if ui.button("Attach companion").clicked() {
                    self.act_attach();
                }
                ui.separator();
            }
            if self.show_adapters {
                ui.heading("Adapters");
                ui.label("Missing engine stays down. Nothing is faked.");
                for key in ["photo", "vector", "light"] {
                    ui.label(format!("{key}: {}", engine_status(&self.status, key)));
                }
                if let Some(arr) = self.caps.get("editable").and_then(Value::as_array) {
                    for item in arr {
                        let label = json_str(item, "label");
                        let writes = json_str(item, "writes");
                        ui.label(format!("{label}: {writes}"));
                    }
                }
                if let Some(arr) = self.caps.get("preview").and_then(Value::as_array) {
                    for item in arr {
                        let label = json_str(item, "label");
                        ui.label(format!("{label}: preview-linked"));
                    }
                }
                if ui.button("Refresh engines").clicked() {
                    self.act_refresh();
                }
                ui.separator();
            }
            if self.show_chat {
                ui.heading("Chat");
                ui.label("Local-first. Keys stay in memory.");
                self.llm_form(ui);
                if ui.button("Send").clicked() {
                    self.act_chat();
                }
                if !self.llm_reply.is_empty() {
                    ui.separator();
                    ui.label("Reply");
                    ui.code(&self.llm_reply);
                }
            }
        });
    }

    fn llm_form(&mut self, ui: &mut egui::Ui) {
        ui.label("Provider");
        egui::ComboBox::from_id_salt("provider")
            .selected_text(&self.llm_provider)
            .show_ui(ui, |ui| {
                for provider in ["ollama", "lmstudio", "openai", "anthropic", "custom"] {
                    ui.selectable_value(&mut self.llm_provider, provider.to_string(), provider);
                }
            });
        ui.label("Base URL");
        ui.text_edit_singleline(&mut self.llm_base);
        ui.label("Model");
        ui.text_edit_singleline(&mut self.llm_model);
        if ui.button("Load models").clicked() {
            self.act_models();
        }
        if !self.llm_models.is_empty() {
            let mut pick = self.llm_model.clone();
            egui::ComboBox::from_id_salt("model")
                .selected_text(&self.llm_model)
                .show_ui(ui, |ui| {
                    for model in &self.llm_models {
                        ui.selectable_value(&mut pick, model.clone(), model);
                    }
                });
            self.llm_model = pick;
        }
        ui.label("API key (memory only)");
        ui.text_edit_singleline(&mut self.llm_key);
        ui.label("System");
        ui.text_edit_multiline(&mut self.llm_system);
        ui.label("Message");
        ui.text_edit_multiline(&mut self.llm_message);
    }

    fn logs_body(&mut self, ui: &mut egui::Ui) {
        if ui.button("Clear").clicked() {
            self.log.clear();
        }
        egui::ScrollArea::vertical().show(ui, |ui| {
            for line in &self.log {
                ui.label(line);
            }
        });
    }

    fn dialogs(&mut self, ctx: &egui::Context) {
        let Some(which) = self.dialog else {
            return;
        };
        let mut open = true;
        match which {
            Dialog::Palette => {
                egui::Window::new("Command Palette")
                    .open(&mut open)
                    .collapsible(false)
                    .default_width(480.0)
                    .show(ctx, |ui| self.palette(ui));
            }
            Dialog::Shortcuts => {
                egui::Window::new("Keyboard Shortcuts")
                    .open(&mut open)
                    .collapsible(false)
                    .default_width(460.0)
                    .show(ctx, |ui| self.shortcuts(ui));
            }
            Dialog::Docs => {
                egui::Window::new("Project Files & Docs")
                    .open(&mut open)
                    .collapsible(false)
                    .default_width(560.0)
                    .show(ctx, |ui| self.docs(ui));
            }
            Dialog::Licenses => {
                egui::Window::new("Licenses & Notices")
                    .open(&mut open)
                    .collapsible(false)
                    .default_width(460.0)
                    .show(ctx, |ui| self.licenses(ui));
            }
            Dialog::About => {
                egui::Window::new("About StudioCraft")
                    .open(&mut open)
                    .collapsible(false)
                    .default_width(420.0)
                    .show(ctx, |ui| self.about(ui));
            }
            Dialog::Preferences => {
                egui::Window::new("Preferences")
                    .open(&mut open)
                    .collapsible(false)
                    .default_width(480.0)
                    .show(ctx, |ui| {
                        ui.label(
                            "LLM settings stay in memory. They are not written to the project.",
                        );
                        self.llm_form(ui);
                    });
            }
        }
        if !open {
            self.dialog = None;
        }
    }

    fn palette(&mut self, ui: &mut egui::Ui) {
        ui.label("Search");
        ui.text_edit_singleline(&mut self.palette_query);
        ui.separator();
        let query = self.palette_query.to_lowercase();
        let ctx = ui.ctx().clone();
        egui::ScrollArea::vertical()
            .max_height(360.0)
            .show(ui, |ui| {
                for cmd in CMDS {
                    let hay = format!("{} {}", cmd.menu, cmd.label).to_lowercase();
                    if !query.is_empty() && !hay.contains(&query) {
                        continue;
                    }
                    if ui.button(format!("{}: {}", cmd.menu, cmd.label)).clicked() {
                        let id = cmd.id;
                        let keep = id.starts_with("help.") || id == "edit.prefs";
                        self.run_cmd(&ctx, id);
                        if !keep {
                            self.dialog = None;
                        }
                    }
                }
            });
    }

    fn shortcuts(&self, ui: &mut egui::Ui) {
        for line in [
            "Ctrl+N  New project",
            "Ctrl+O  Open project",
            "Ctrl+S  Reload manifest (natives save in PhotoCraft and VectorCraft)",
            "Ctrl+W  Close project",
            "Ctrl+Q  Quit",
            "Ctrl+E  Composite preview path (does not flatten)",
            "Ctrl+Z / Ctrl+Y  Undo and redo are not in v1",
            "Ctrl+X / Ctrl+C / Ctrl+V  Cut, copy, and paste the project path",
            "Ctrl+A  Copy path and revision",
            "Ctrl+,  Preferences",
            "Ctrl+= / Ctrl+-  Zoom",
            "Ctrl+0 / Ctrl+1  Reset zoom",
            "Ctrl+Wheel  Zoom",
            "Ctrl+K or Ctrl+Shift+P  Command palette",
            "F1  This list",
        ] {
            ui.label(line);
        }
    }

    fn docs(&self, ui: &mut egui::Ui) {
        ui.label("StudioCraft keeps one project folder and the handoff gate.");
        ui.label("PhotoCraft writes painting/. VectorCraft writes vectors/.");
        ui.separator();
        for line in [
            "docs/PRD.md  what the host is for",
            "docs/CONTRACT.md  folder layout and block rules",
            "docs/SKILLS.md  the six tasks",
            "docs/WORKFLOWS.md  round-trip, blocked, companion, chat",
            "docs/TOOLS.md  CLI and MCP names",
            "docs/UI_PARITY.md  menu, sidebar, and dispatch",
        ] {
            ui.label(line);
        }
        ui.separator();
        ui.label("Contract");
        egui::ScrollArea::vertical()
            .max_height(220.0)
            .show(ui, |ui| {
                ui.code(include_str!("../../../docs/CONTRACT.md"));
            });
    }

    fn licenses(&self, ui: &mut egui::Ui) {
        ui.label("StudioCraft is MIT OR Apache-2.0.");
        ui.label("The window is eframe and egui. Manifests are serde_json.");
        ui.label("Those crate notices ship with their crates.io copies.");
        ui.separator();
        ui.label("The host calls dispatch in-process and opens no port.");
        ui.label(
            "Companion attach writes a pointer JSON file. It does not copy the companion bytes.",
        );
    }

    fn about(&self, ui: &mut egui::Ui) {
        let version = json_str(&self.status, "version");
        let contract = json_str(&self.status, "contract");
        ui.heading("StudioCraft");
        ui.label(format!("version {version}"));
        ui.label(format!("contract {contract}"));
        ui.separator();
        ui.label("Shared project host for PhotoCraft and VectorCraft.");
        ui.label("This window checks handoffs, links companions, and lists tasks.");
        ui.label("It does not paint, draw, or flatten a project into one file.");
    }
}

impl eframe::App for StudioApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.handle_keys(&ctx);
        handle_zoom(&ctx);

        let mut titles: Vec<egui::Response> = Vec::new();
        egui::Panel::top("top_menubar").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                let file = ui.menu_button("File", |ui| self.file_menu(ui));
                titles.push(file.response);
                let edit = ui.menu_button("Edit", |ui| self.edit_menu(ui));
                titles.push(edit.response);
                let view = ui.menu_button("View", |ui| self.view_menu(ui));
                titles.push(view.response);
                let insert = ui.menu_button("Insert", |ui| self.insert_menu(ui));
                titles.push(insert.response);
                let modify = ui.menu_button("Modify", |ui| self.modify_menu(ui));
                titles.push(modify.response);
                let commands = ui.menu_button("Commands", |ui| self.commands_menu(ui));
                titles.push(commands.response);
                let window = ui.menu_button("Window", |ui| self.window_menu(ui));
                titles.push(window.response);
                let help = ui.menu_button("Help", |ui| self.help_menu(ui));
                titles.push(help.response);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Safe Rust");
                    ui.label(json_str(&self.status, "version"));
                });
            });
        });
        switch_on_hover(&ctx, &titles);

        egui::Panel::bottom("statusbar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status_line);
                ui.separator();
                ui.label(format!("{} log lines", self.log.len()));
            });
        });
        if self.show_logs {
            egui::Panel::bottom("logs")
                .resizable(true)
                .default_size(140.0)
                .show(ui, |ui| self.logs_body(ui));
        }
        if self.show_tasks {
            egui::Panel::left("tasks")
                .resizable(true)
                .default_size(300.0)
                .show(ui, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| self.tasks_panel(ui));
                });
        }
        if self.show_handoff || self.show_adapters || self.show_chat {
            egui::Panel::right("inspector")
                .resizable(true)
                .default_size(320.0)
                .show(ui, |ui| self.inspector(ui));
        }
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| self.stage(ui));
        });
        self.dialogs(&ctx);
    }
}

/// Headless parity + workstation shell checks (no window).
pub fn run_smoke_checks() -> Result<Vec<String>, String> {
    let mut lines: Vec<String> = Vec::new();
    shell_check()?;
    lines.push("menus: File Edit View Insert Modify Commands Window Help".into());
    lines.push("help: 5 dialogs".into());
    lines.push("tasks: sidebar matches skills".into());

    let status = dispatch("status", &json!({})).map_err(|e| e.to_string())?;
    if status
        .get("success")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        lines.push("status: ok".into());
    } else {
        return Err("status missing success".into());
    }
    let via_app = StudioApp::app_dispatch("status", &json!({}))?;
    if via_app.get("version") == status.get("version") {
        lines.push("parity: app_dispatch == core dispatch".into());
    } else {
        return Err("parity mismatch".into());
    }
    let caps = dispatch("adapter_caps", &json!({})).map_err(|e| e.to_string())?;
    let n = caps
        .get("editable")
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or(0);
    if n == 2 {
        lines.push("caps: photo+vector editable".into());
    } else {
        return Err("caps editable != 2".into());
    }
    let gate = dispatch(
        "handoff_check",
        &json!({"project": "D:/tmp/smoke", "from": "photo", "to": "vector"}),
    )
    .map_err(|e| e.to_string())?;
    if is_allowed(&gate) {
        lines.push("handoff: photo->vector allowed".into());
    } else {
        return Err("handoff photo->vector should allow".into());
    }
    let skills = dispatch("skills_list", &json!({})).map_err(|e| e.to_string())?;
    if skills.get("count").and_then(Value::as_i64).unwrap_or(0) >= 6 {
        lines.push("skills: 6 registered".into());
    } else {
        return Err("skills < 6".into());
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
    fn menus_follow_craft_standard() {
        assert_eq!(
            MENU_TITLES,
            ["File", "Edit", "View", "Insert", "Modify", "Commands", "Window", "Help"]
        );
        let err = shell_check().err().unwrap_or_default();
        assert!(err.is_empty(), "{err}");
    }

    #[test]
    fn task_sidebar_matches_skill_registry() {
        let err = shell_check().err().unwrap_or_default();
        assert!(err.is_empty(), "{err}");
    }

    #[test]
    fn bump_requires_both_directions() {
        let allow = json!({"allowed": true});
        let block = json!({"allowed": false, "reason": "blocked"});
        assert!(both_allowed(&allow, &allow));
        assert!(!both_allowed(&allow, &block));
        assert!(!both_allowed(&block, &allow));
    }

    #[test]
    fn glyph_allowlist_no_tofu() {
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
