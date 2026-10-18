//! Pattern: responsive layout.
//!
//! A layout that changes composition based on available width: cards wrap in
//! a wide view and stack into a compact list in a narrow view.

use super::{Category, Demo};
use egui::{Align, Layout, Ui};

#[derive(Default)]
pub struct Responsive;

impl Demo for Responsive {
    fn name(&self) -> &'static str {
        "Responsive layout"
    }
    fn description(&self) -> &'static str {
        "Wide card grid that becomes a compact stacked layout as space narrows."
    }
    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        let wide = ui.available_width() >= 560.0;
        ui.horizontal(|ui| {
            ui.heading("Responsive workspace");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.weak(if wide { "Wide mode" } else { "Compact mode" });
            });
        });
        ui.separator();
        if wide {
            ui.columns(3, |columns| {
                for (column, (title, value)) in columns.iter_mut().zip([
                    ("Inbox", "24 items"),
                    ("Due today", "8 items"),
                    ("Completed", "126 items"),
                ]) {
                    egui::Frame::group(column.style()).show(column, |ui| {
                        ui.strong(title);
                        ui.heading(value);
                        ui.weak("Open section →");
                    });
                }
            });
        } else {
            for (title, value) in [
                ("Inbox", "24 items"),
                ("Due today", "8 items"),
                ("Completed", "126 items"),
            ] {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.strong(title);
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| ui.weak(value));
                    });
                });
                ui.add_space(6.0);
            }
        }
        ui.add_space(12.0);
        ui.label("Resize the window or launcher panel to see the composition change.");
    }
}
