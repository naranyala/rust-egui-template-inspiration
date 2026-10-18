//! Pattern: accordion sections.
//!
//! Collapsible groups that share a panel: opening one closes the others.
//! Built from `CollapsingHeader`, the idiomatic egui primitive for this.

use super::{Category, Demo};
use egui::{RichText, Ui};

#[derive(Default)]
pub struct Accordion {
    open: usize,
    show_icons: bool,
}

impl Demo for Accordion {
    fn name(&self) -> &'static str {
        "Accordion sections"
    }

    fn description(&self) -> &'static str {
        "Collapsible groups where opening one closes the rest, plus optional icons."
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_icons, "Show icons");
            ui.weak("Only one section stays open at a time");
        });
        ui.add_space(4.0);

        let sections = [
            (
                "Getting started",
                "Install the template and run the launcher.",
            ),
            ("Layout basics", "Panels, grids, splits and tabs."),
            ("Data patterns", "Tables, trees and master-detail views."),
            ("Feedback", "Dialogs, toasts, progress and status."),
        ];
        let show_icons = self.show_icons;

        for (index, (title, body)) in sections.iter().enumerate() {
            let response = egui::CollapsingHeader::new(*title)
                .id_salt(index)
                .open(Some(self.open == index))
                .icon(move |ui, openness, response| {
                    if show_icons {
                        let arrow = if openness > 0.5 { "▼" } else { "▶" };
                        response.widget_info(|| {
                            egui::WidgetInfo::labeled(egui::Role::Button, true, arrow)
                        });
                        ui.label(arrow);
                    }
                })
                .show(ui, |ui| {
                    ui.label(RichText::new(*body).weak());
                    if ui.button("Open docs").clicked() {
                        self.open = index;
                    }
                });
            if response.header_response.clicked() {
                self.open = if self.open == index {
                    sections.len()
                } else {
                    index
                };
            }
        }

        if self.open < sections.len() {
            ui.add_space(4.0);
            ui.weak(format!("Last opened: {}", sections[self.open].0));
        }
    }
}
