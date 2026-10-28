//! Pattern: tabs.
//!
//! egui has no built-in tab widget; a row of `selectable_label`s plus a
//! match on the selected index is the idiomatic approach.

use super::{Category, Demo};
use egui::Ui;

#[derive(Default)]
pub struct Tabs {
    selected: usize,
    inner_selected: usize,
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
        let tabs = ["Overview", "Workspace", "Details", "Settings"];

        ui.horizontal_wrapped(|ui| {
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
                ui.label("A simple top-level tab layout with supporting summary content.");
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    ui.label("Status:");
                    ui.colored_label(ui.visuals().warn_fg_color, "Ready");
                });
            }
            1 => {
                ui.heading("Workspace");
                ui.label("This tab contains an inner set of vertical tabs on the left.");
                ui.add_space(12.0);

                ui.columns(2, |cols| {
                    let left = &mut cols[0];
                    left.vertical(|ui| {
                        ui.heading("Navigation");
                        for (i, label) in ["Dashboard", "Files", "Layout", "Activity"].iter().enumerate() {
                            if ui.selectable_label(self.inner_selected == i, *label).clicked() {
                                self.inner_selected = i;
                            }
                        }
                    });

                    let right = &mut cols[1];
                    right.vertical(|ui| match self.inner_selected {
                        0 => {
                            ui.heading("Dashboard");
                            ui.label("Quick metrics and high-level status for the active workspace.");
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                ui.label("Completion:");
                                ui.add(egui::ProgressBar::new(0.72).text("72%"));
                            });
                        }
                        1 => {
                            ui.heading("Files");
                            ui.label("Recent documents, shared assets, and project folders.");
                            ui.add_space(8.0);
                            ui.columns(2, |cols| {
                                cols[0].label("src/main.rs");
                                cols[1].label("updated 2m ago");
                                cols[0].label("design/mockup.png");
                                cols[1].label("shared");
                            });
                        }
                        2 => {
                            ui.heading("Layout");
                            ui.label("Workspace arrangement, pane sizing, and editor preferences.");
                            ui.checkbox(&mut false, "Compact mode");
                            ui.checkbox(&mut true, "Show side panel");
                            ui.checkbox(&mut true, "Snap panels");
                        }
                        _ => {
                            ui.heading("Activity");
                            ui.label("Recent updates, changes, and collaboration stream.");
                            ui.label("• Anna pushed a design refactor");
                            ui.label("• Build succeeded in 3m 12s");
                            ui.label("• Preview URL refreshed");
                        }
                    });
                });
            }
            2 => {
                ui.heading("Details");
                ui.label("Properties, metadata, and extended information.");
                ui.add_space(8.0);
                ui.group(|ui| {
                    ui.label("Project name: Example Dashboard");
                    ui.label("Owner: Platform Team");
                    ui.label("Last updated: 2 hours ago");
                });
            }
            _ => {
                ui.heading("Settings");
                ui.label("Configuration options for the active workspace.");
                ui.add_space(8.0);
                ui.checkbox(&mut true, "Enable autosave");
                ui.checkbox(&mut true, "Notifications");
                ui.checkbox(&mut false, "Experimental preview");
            }
        }
    }
}
