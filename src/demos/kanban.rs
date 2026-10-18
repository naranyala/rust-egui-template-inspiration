//! Pattern: kanban board.
//!
//! Three equal-width work columns with compact cards.

use super::{Category, Demo};
use egui::{Align, Layout, RichText, Ui};

#[derive(Default)]
pub struct Kanban {
    show_done: bool,
    selected: Option<&'static str>,
}

impl Demo for Kanban {
    fn name(&self) -> &'static str {
        "Kanban board"
    }
    fn description(&self) -> &'static str {
        "Horizontally scrolling work columns made from reusable card surfaces."
    }
    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Product roadmap");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.checkbox(&mut self.show_done, "Show completed");
            });
        });
        ui.separator();
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.horizontal_top(|ui| {
                for (column, cards) in [
                    (
                        "Backlog",
                        vec![
                            ("Research onboarding", "Mira"),
                            ("Audit permissions", "Sam"),
                        ],
                    ),
                    (
                        "In progress",
                        vec![
                            ("New dashboard", "Ari"),
                            ("Import flow", "Jon"),
                            ("Keyboard shortcuts", "Mira"),
                        ],
                    ),
                    (
                        "Review",
                        vec![("Empty states", "Sam"), ("Export report", "Ari")],
                    ),
                ] {
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.set_min_width(190.0);
                        ui.strong(column);
                        ui.separator();
                        for (title, owner) in cards {
                            if Self::card(ui, title, owner, self.selected == Some(title)).clicked()
                            {
                                self.selected = Some(title);
                            }
                            ui.add_space(6.0);
                        }
                        let _ = ui.small_button("+ Add card");
                    });
                }
                if self.show_done {
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.set_min_width(190.0);
                        ui.strong("Done");
                        ui.separator();
                        Self::card(ui, "Release notes", "Jon", false);
                        Self::card(ui, "Update help center", "Sam", false);
                    });
                }
            });
        });
        if let Some(selected) = self.selected {
            ui.add_space(8.0);
            ui.label(RichText::new(format!("Selected card: {selected}")).weak());
        }
    }
}

impl Kanban {
    fn card(
        ui: &mut Ui,
        title: &'static str,
        owner: &'static str,
        selected: bool,
    ) -> egui::Response {
        egui::Frame::canvas(ui.style())
            .show(ui, |ui| {
                ui.set_min_width(170.0);
                let _ = ui.selectable_label(selected, title);
                ui.horizontal(|ui| {
                    ui.weak(format!("👤 {owner}"));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| ui.weak("2d"));
                });
            })
            .response
    }
}
