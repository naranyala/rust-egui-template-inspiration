//! Pattern: command palette / quick-open.
//!
//! A keyboard-friendly command surface that filters actions and exposes
//! shortcuts without adding another permanent panel to the application.

use super::{Category, Demo};
use egui::{Align, Layout, Ui};

#[derive(Default)]
pub struct CommandPalette {
    query: String,
    open: bool,
    last_command: Option<&'static str>,
}

impl Demo for CommandPalette {
    fn name(&self) -> &'static str {
        "Command palette"
    }
    fn description(&self) -> &'static str {
        "Searchable keyboard-first actions in a transient command surface."
    }
    fn category(&self) -> Category {
        Category::Widgets
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Command palette");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui.button("Open palette  ⌘K").clicked() {
                    self.open = true;
                }
            });
        });
        ui.separator();
        ui.label("Use a transient command surface when an application has many actions but limited toolbar space.");
        if let Some(command) = self.last_command {
            ui.add_space(8.0);
            ui.weak(format!("Last command: {command}"));
        }

        if ui.input(|input| input.key_pressed(egui::Key::K) && input.modifiers.command) {
            self.open = true;
        }

        if self.open {
            let commands = [
                ("Open file", "⌘O"),
                ("Toggle sidebar", "⌘B"),
                ("Switch theme", "⌘T"),
                ("Export screenshot", "⌘⇧S"),
            ];
            let mut close = false;
            egui::Window::new("Command palette")
                .id(egui::Id::unique("command_palette_window"))
                .collapsible(false)
                .resizable(false)
                .default_width(420.0)
                .show(ui.ctx(), |ui| {
                    let search = ui.add(
                        egui::TextEdit::singleline(&mut self.query)
                            .hint_text("Type a command…")
                            .desired_width(f32::INFINITY),
                    );
                    search.request_focus();
                    ui.separator();
                    let needle = self.query.to_lowercase();
                    for (label, shortcut) in commands {
                        if !needle.is_empty() && !label.to_lowercase().contains(&needle) {
                            continue;
                        }
                        ui.horizontal(|ui| {
                            if ui.selectable_label(false, label).clicked() {
                                self.last_command = Some(label);
                                close = true;
                            }
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.weak(shortcut);
                            });
                        });
                    }
                    if ui.input(|input| input.key_pressed(egui::Key::Escape)) {
                        close = true;
                    }
                });
            if close {
                self.open = false;
                self.query.clear();
            }
        }
    }
}
