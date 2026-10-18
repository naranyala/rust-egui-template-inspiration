//! Pattern: three-pane workspace.
//!
//! A classic IDE/file-manager composition: navigator, main editor, and an
//! inspector. Each side pane is independently resizable.

use super::{Category, Demo};
use egui::{Align, Layout, Ui};

#[derive(Default)]
pub struct ThreePane {
    selected_file: usize,
    show_inspector: bool,
}

impl Demo for ThreePane {
    fn name(&self) -> &'static str {
        "Three-pane workspace"
    }
    fn description(&self) -> &'static str {
        "IDE-style navigator, editor, and inspector with independently resizable panes."
    }
    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.strong("Workspace");
            ui.checkbox(&mut self.show_inspector, "Inspector");
        });
        ui.separator();

        egui::Panel::left("three_pane_explorer")
            .resizable(true)
            .default_size(180.0)
            .min_size(130.0)
            .show(ui, |ui| {
                ui.strong("Explorer");
                ui.separator();
                for (index, file) in ["main.rs", "app.rs", "theme.rs", "Cargo.toml"]
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

        if self.show_inspector {
            egui::Panel::right("three_pane_inspector")
                .resizable(true)
                .default_size(190.0)
                .min_size(140.0)
                .show(ui, |ui| {
                    ui.strong("Inspector");
                    ui.separator();
                    ui.label("Type: source file");
                    ui.label("Encoding: UTF-8");
                    ui.label("Modified: today");
                });
        }

        egui::CentralPanel::default().show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                let file = ["main.rs", "app.rs", "theme.rs", "Cargo.toml"][self.selected_file];
                ui.heading(file);
                ui.separator();
                ui.monospace("// editor content lives here");
                ui.monospace("fn main() { /* ... */ }");
            });
        });
    }
}
