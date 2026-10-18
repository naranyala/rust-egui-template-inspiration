//! Pattern: settings page.
//!
//! A navigation rail paired with grouped settings content.

use super::{Category, Demo};
use egui::{Align, Layout, RichText, Ui};

#[derive(Default)]
pub struct Settings {
    section: usize,
    search: String,
    workspace_name: String,
    language: usize,
    theme: usize,
    notifications: bool,
    telemetry: bool,
    compact_mode: bool,
}

impl Demo for Settings {
    fn name(&self) -> &'static str {
        "Settings page"
    }
    fn description(&self) -> &'static str {
        "Navigation rail with searchable, grouped preference controls."
    }
    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        let sections = ["General", "Appearance", "Notifications", "Privacy"];
        ui.horizontal(|ui| {
            ui.heading("Settings");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.weak("Changes are saved automatically");
            });
        });
        ui.separator();

        egui::Panel::left("settings_nav")
            .resizable(true)
            .default_size(170.0)
            .min_size(130.0)
            .show(ui, |ui| {
                ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("Search settings…"));
                ui.add_space(8.0);
                let needle = self.search.to_lowercase();
                for (index, label) in sections.iter().enumerate() {
                    if (needle.is_empty() || label.to_lowercase().contains(&needle))
                        && ui.selectable_label(self.section == index, *label).clicked()
                    {
                        self.section = index;
                    }
                }
            });

        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_max_width(620.0);
                ui.heading(sections[self.section]);
                ui.add_space(8.0);
                match self.section {
                    0 => {
                        Self::group(ui, "Workspace", |ui| {
                            ui.label("Workspace name");
                            ui.text_edit_singleline(&mut self.workspace_name);
                            ui.label("Language");
                            egui::ComboBox::from_id_salt("settings_language")
                                .selected_text(["English", "Japanese", "German"][self.language])
                                .show_ui(ui, |ui| {
                                    for (index, label) in
                                        ["English", "Japanese", "German"].iter().enumerate()
                                    {
                                        ui.selectable_value(&mut self.language, index, *label);
                                    }
                                });
                        });
                    }
                    1 => {
                        Self::group(ui, "Density", |ui| {
                            ui.checkbox(&mut self.compact_mode, "Use compact spacing");
                            ui.weak("Compact mode fits more controls into a window.");
                        });
                        Self::group(ui, "Theme", |ui| {
                            ui.horizontal(|ui| {
                                for (index, label) in ["System", "Light", "Dark"].iter().enumerate()
                                {
                                    if ui.selectable_label(self.theme == index, *label).clicked() {
                                        self.theme = index;
                                    }
                                }
                            });
                        });
                    }
                    2 => Self::group(ui, "Delivery", |ui| {
                        ui.checkbox(&mut self.notifications, "Enable desktop notifications");
                        ui.checkbox(&mut self.telemetry, "Notify me about weekly summaries");
                    }),
                    _ => Self::group(ui, "Data sharing", |ui| {
                        ui.checkbox(&mut self.telemetry, "Share anonymous usage data");
                        ui.label(RichText::new("No personal content is collected.").weak());
                    }),
                }
            });
        });
    }
}

impl Settings {
    fn group(ui: &mut Ui, title: &str, add_contents: impl FnOnce(&mut Ui)) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.strong(title);
            ui.separator();
            add_contents(ui);
        });
        ui.add_space(10.0);
    }
}
