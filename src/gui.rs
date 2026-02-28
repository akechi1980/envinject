use std::collections::BTreeMap;

use anyhow::Result;
use eframe::egui;

use crate::config::{ConfigManager, ConfigStore, ProjectConfig};

const HELP_README: &str = include_str!("../README.md");

pub fn run_gui() -> Result<()> {
    let manager = ConfigManager::new_default()?;
    let store = manager.load_store()?;

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "envinject Config Editor",
        native_options,
        Box::new(move |cc| {
            configure_fonts_for_cjk(&cc.egui_ctx);
            Ok(Box::new(EnvInjectApp::new(manager, store)))
        }),
    )
    .map_err(|e| anyhow::anyhow!("Failed to start GUI: {e}"))?;

    Ok(())
}

fn configure_fonts_for_cjk(ctx: &egui::Context) {
    let Some(font_bytes) = load_cjk_font_bytes() else {
        return;
    };

    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "cjk-fallback".to_string(),
        egui::FontData::from_owned(font_bytes).into(),
    );

    if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        family.insert(0, "cjk-fallback".to_string());
    }
    if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
        family.push("cjk-fallback".to_string());
    }

    ctx.set_fonts(fonts);
}

fn load_cjk_font_bytes() -> Option<Vec<u8>> {
    let mut candidates = vec![
        // Linux common fonts
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc".to_string(),
        "/usr/share/fonts/opentype/noto/NotoSansCJKSC-Regular.otf".to_string(),
        "/usr/share/fonts/opentype/noto/NotoSansSC-Regular.otf".to_string(),
        "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc".to_string(),
        // macOS common fonts
        "/System/Library/Fonts/PingFang.ttc".to_string(),
        "/System/Library/Fonts/Hiragino Sans GB.ttc".to_string(),
    ];

    if let Ok(windir) = std::env::var("WINDIR") {
        candidates.push(format!("{windir}\\Fonts\\msyh.ttc"));
        candidates.push(format!("{windir}\\Fonts\\simhei.ttf"));
        candidates.push(format!("{windir}\\Fonts\\simsun.ttc"));
    }

    for path in candidates {
        if let Ok(bytes) = std::fs::read(&path) {
            return Some(bytes);
        }
    }

    None
}

struct EnvInjectApp {
    manager: ConfigManager,
    store: ConfigStore,
    selected_project: Option<String>,
    new_project_name: String,
    new_key: String,
    new_value: String,
    status: String,
    show_help: bool,
}

impl EnvInjectApp {
    fn new(manager: ConfigManager, store: ConfigStore) -> Self {
        let selected_project = store.projects.keys().next().cloned();
        Self {
            manager,
            store,
            selected_project,
            new_project_name: String::new(),
            new_key: String::new(),
            new_value: String::new(),
            status: String::new(),
            show_help: false,
        }
    }

    fn add_project(&mut self) {
        let name = self.new_project_name.trim();
        if name.is_empty() {
            self.status = "Project name cannot be empty".to_string();
            return;
        }

        if self.store.projects.contains_key(name) {
            self.status = format!("Project already exists: {name}");
            return;
        }

        self.store.projects.insert(
            name.to_string(),
            ProjectConfig {
                env: BTreeMap::new(),
            },
        );
        self.selected_project = Some(name.to_string());
        self.new_project_name.clear();
        self.status = "Project created".to_string();
    }

    fn remove_selected_project(&mut self) {
        let Some(project) = self.selected_project.clone() else {
            self.status = "Please select a project first".to_string();
            return;
        };

        self.store.projects.remove(&project);
        self.selected_project = self.store.projects.keys().next().cloned();
        self.status = format!("Project deleted: {project}");
    }

    fn add_env_key(&mut self) {
        let Some(project) = self.selected_project.clone() else {
            self.status = "Please select a project first".to_string();
            return;
        };
        let key = self.new_key.trim();
        if key.is_empty() {
            self.status = "Key cannot be empty".to_string();
            return;
        }

        let value = self.new_value.clone();
        if let Some(cfg) = self.store.projects.get_mut(&project) {
            cfg.env.insert(key.to_string(), value);
            self.new_key.clear();
            self.new_value.clear();
            self.status = "Key/value added or updated".to_string();
        }
    }

    fn save(&mut self) {
        match self.manager.save_store(&self.store) {
            Ok(_) => self.status = format!("Saved: {}", self.manager.path.display()),
            Err(e) => self.status = format!("Save failed: {e}"),
        }
    }
}

impl eframe::App for EnvInjectApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("envinject Config Editor");
                if ui.button("Save Config").clicked() {
                    self.save();
                }
                if ui.button("Help").clicked() {
                    self.show_help = true;
                }
            });
            if !self.status.is_empty() {
                ui.label(&self.status);
            }
        });

        if self.show_help {
            egui::Window::new("Help / 使用说明")
                .open(&mut self.show_help)
                .resizable(true)
                .default_size([860.0, 620.0])
                .show(ctx, |ui| {
                    ui.label("Usage Guide（含中文）");
                    ui.separator();
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            ui.label(HELP_README);
                        });
                });
        }

        egui::SidePanel::left("projects")
            .min_width(220.0)
            .show(ctx, |ui| {
                ui.heading("Projects");

                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.new_project_name);
                    if ui.button("Create").clicked() {
                        self.add_project();
                    }
                });

                if ui.button("Delete Selected Project").clicked() {
                    self.remove_selected_project();
                }

                ui.separator();
                for project in self.store.projects.keys() {
                    let is_selected = self
                        .selected_project
                        .as_ref()
                        .map(|s| s == project)
                        .unwrap_or(false);
                    if ui.selectable_label(is_selected, project).clicked() {
                        self.selected_project = Some(project.clone());
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let Some(project) = self.selected_project.clone() else {
                ui.label("Create and select a project from the left panel first.");
                return;
            };

            ui.heading(format!("Project: {project}"));
            ui.separator();

            ui.label("Add or update key/value");
            ui.horizontal(|ui| {
                ui.label("Key");
                let total_width = ui.available_width();
                let key_width = total_width * 0.3;
                let value_width = total_width * 0.6;
                ui.add_sized(
                    [key_width.max(120.0), 0.0],
                    egui::TextEdit::singleline(&mut self.new_key),
                );
                ui.label("Value");
                ui.add_sized(
                    [value_width.max(180.0), 0.0],
                    egui::TextEdit::singleline(&mut self.new_value),
                );
                if ui.button("Add/Update").clicked() {
                    self.add_env_key();
                }
            });

            ui.separator();
            ui.label("Current key/value pairs");

            let mut pending_removes = Vec::new();
            if let Some(cfg) = self.store.projects.get_mut(&project) {
                for (key, value) in cfg.env.iter_mut() {
                    ui.horizontal(|ui| {
                        ui.monospace(key);
                        ui.text_edit_singleline(value);
                        if ui.button("Delete").clicked() {
                            pending_removes.push(key.clone());
                        }
                    });
                }

                for k in pending_removes {
                    cfg.env.remove(&k);
                }
            }
        });
    }
}
