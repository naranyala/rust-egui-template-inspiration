//! Pattern: tabs.
//!
//! egui has no built-in tab widget; a row of `selectable_label`s plus a
//! match on the selected index is the idiomatic approach.

use super::{Category, Demo};
use egui::Ui;

#[derive(Default)]
pub struct Tabs {
    selected: usize,
}

impl Demo for Tabs {
    fn name(&self) -> &'static str {
        "Tabs"
    }
    fn description(&self) -> &'static str {
        "Tabbed views built from selectable labels + conditional content."
    }
    fn category(&self) -> Category {
        Category::Layout
    }
    fn ui(&mut self, ui: &mut Ui) {
        let tabs = ["Overview", "Details", "History", "Settings"];
        ui.horizontal(|ui| {
            for (i, label) in tabs.iter().enumerate() {
                if ui.selectable_label(self.selected == i, *label).clicked() {
                    self.selected = i;
                }
            }
        });
        ui.separator();
        match self.selected {
            0 => {
                ui.heading("Overview");
                ui.label("Summary of the selected item.");
            }
            1 => {
                ui.heading("Details");
                ui.label("Properties and metadata.");
            }
            2 => {
                ui.heading("History");
                ui.label("Recent activity log.");
            }
            _ => {
                ui.heading("Settings");
                ui.label("Configuration options.");
            }
        }
    }
}
