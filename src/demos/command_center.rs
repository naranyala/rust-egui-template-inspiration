//! Pattern: command center.
//!
//! A dense operational screen with navigation, alerts, metrics, and events.

use super::{Category, Demo};
use egui::{Align, Layout, RichText, Ui};

#[derive(Default)]
pub struct CommandCenter {
    selected: usize,
    acknowledged: bool,
    refreshes: u32,
}

impl Demo for CommandCenter {
    fn name(&self) -> &'static str {
        "Command center"
    }
    fn description(&self) -> &'static str {
        "Dense operations screen with navigation, alerts, metrics, and live events."
    }
    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        egui::Panel::left("command_nav")
            .default_size(150.0)
            .min_size(120.0)
            .show(ui, |ui| {
                ui.strong("Control room");
                ui.separator();
                for (index, item) in ["Overview", "Services", "Deployments", "Incidents"]
                    .iter()
                    .enumerate()
                {
                    if ui.selectable_label(self.selected == index, *item).clicked() {
                        self.selected = index;
                    }
                }
            });
        egui::Panel::right("command_events")
            .resizable(true)
            .default_size(210.0)
            .min_size(160.0)
            .show(ui, |ui| {
                ui.strong("Live events");
                ui.separator();
                for event in [
                    "API latency normal",
                    "Deploy started",
                    "Worker recovered",
                    "Report queued",
                ] {
                    ui.label(RichText::new(event).weak());
                    ui.add_space(4.0);
                }
            });
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(["Overview", "Services", "Deployments", "Incidents"][self.selected]);
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Refresh").clicked() {
                        self.refreshes += 1;
                        self.acknowledged = false;
                    }
                });
            });
            ui.separator();
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.colored_label(ui.visuals().error_fg_color, "● 2 active alerts");
                    ui.separator();
                    ui.weak(format!("Refreshed {} times", self.refreshes));
                    if !self.acknowledged && ui.button("Acknowledge").clicked() {
                        self.acknowledged = true;
                    }
                });
            });
            ui.add_space(10.0);
            ui.columns(3, |columns| {
                for (column, (label, value)) in columns.iter_mut().zip([
                    ("Uptime", "99.98%"),
                    ("Requests/min", "18.4k"),
                    ("Deployments", "24"),
                ]) {
                    egui::Frame::group(column.style()).show(column, |ui| {
                        ui.weak(label);
                        ui.heading(value);
                    });
                }
            });
        });
    }
}
