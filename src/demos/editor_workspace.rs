//! Pattern: explorer / editor / terminal workspace.
//!
//! A complete developer-tool composition with a file navigator, editor area,
//! and collapsible terminal output.

use super::{Category, Demo};
use egui::{Align, Layout, RichText, Ui};

#[derive(Default)]
pub struct EditorWorkspace {
    selected_file: usize,
    terminal_open: bool,
    terminal_command: String,
    command_count: u32,
}

impl Demo for EditorWorkspace {
    fn name(&self) -> &'static str {
        "Editor workspace"
    }
    fn description(&self) -> &'static str {
        "File explorer, editor surface, and collapsible terminal in one desktop shell."
    }
    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.strong("rust-egui-template-inspiration");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui.button("Terminal").clicked() {
                    self.terminal_open = !self.terminal_open;
                }
            });
        });
        ui.separator();
        egui::Panel::left("editor_explorer")
            .resizable(true)
            .default_size(190.0)
            .min_size(140.0)
            .show(ui, |ui| {
                ui.strong("Explorer");
                ui.separator();
                for (index, file) in [
                    "src/main.rs",
                    "src/app.rs",
                    "src/theme.rs",
                    "Cargo.toml",
                    "README.md",
                ]
                .iter()
                .enumerate()
                {
                    if ui
                        .selectable_label(self.selected_file == index, *file)
                        .clicked()
                    {
                        self.selected_file = index;
                    }
                }
            });
        if self.terminal_open {
            egui::Panel::bottom("editor_terminal")
                .resizable(true)
                .default_size(130.0)
                .min_size(80.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.monospace("> ");
                        let response = ui.add(
                            egui::TextEdit::singleline(&mut self.terminal_command)
                                .desired_width(f32::INFINITY),
                        );
                        if response.lost_focus()
                            && ui.input(|input| input.key_pressed(egui::Key::Enter))
                        {
                            self.command_count += 1;
                            self.terminal_command.clear();
                        }
                    });
                    ui.label(
                        RichText::new(format!("{} command(s) executed", self.command_count)).weak(),
                    );
                });
        }
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading(
                ["main.rs", "app.rs", "theme.rs", "Cargo.toml", "README.md"][self.selected_file],
            );
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.monospace("use egui::Ui;");
                ui.monospace("");
                ui.monospace("pub fn render(ui: &mut Ui) {");
                ui.monospace("    ui.heading(\"Editor surface\");");
                ui.monospace("}");
            });
        });
    }
}
