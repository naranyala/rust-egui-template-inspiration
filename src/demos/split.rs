//! Pattern: split panes.
//!
//! Resizable split views built from nested panels. egui has no dedicated
//! "splitter" widget, but a resizable `Panel` gives the same behaviour.

use super::{Category, Demo};
use egui::Ui;

#[derive(Default)]
pub struct Splits {}

impl Demo for Splits {
    fn name(&self) -> &'static str {
        "Split panes"
    }
    fn description(&self) -> &'static str {
        "Resizable horizontal/vertical splits built from nested panels."
    }
    fn category(&self) -> Category {
        Category::Layout
    }
    fn ui(&mut self, ui: &mut Ui) {
        ui.label("Drag the panel edges to resize either side.");
        ui.add_space(4.0);

        // Horizontal split: left + right panels, central fills the gap.
        egui::Panel::left("split_left")
            .resizable(true)
            .default_size(200.0)
            .min_size(80.0)
            .show(ui, |ui| {
                ui.strong("Left pane");
                ui.label("Sidebar / navigator content.");
            });
        egui::CentralPanel::default().show(ui, |ui| {
            ui.strong("Right pane");
            ui.label("Main content area.");
            ui.add_space(8.0);
            // Vertical split inside the right pane.
            egui::Panel::top("split_top")
                .resizable(true)
                .default_size(120.0)
                .min_size(40.0)
                .show(ui, |ui| {
                    ui.strong("Top");
                    ui.label("Preview / toolbar region.");
                });
            egui::CentralPanel::default().show(ui, |ui| {
                ui.strong("Bottom");
                ui.label("Detail / editor region.");
            });
        });
    }
}
