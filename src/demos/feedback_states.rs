//! Pattern: feedback and async states.
//!
//! A compact reference for loading, empty, error, retry, success, and toast
//! states that every production desktop application eventually needs.

use super::{Category, Demo};
use egui::{Color32, RichText, Ui};
use std::time::{Duration, Instant};

#[derive(Default)]
pub struct FeedbackStates {
    state: usize,
    toast_until: Option<Instant>,
}

impl Demo for FeedbackStates {
    fn name(&self) -> &'static str {
        "Feedback states"
    }
    fn description(&self) -> &'static str {
        "Loading, empty, error, retry, success, and temporary toast states."
    }
    fn category(&self) -> Category {
        Category::Feedback
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            for (index, label) in ["Loading", "Empty", "Error", "Success"].iter().enumerate() {
                if ui.selectable_label(self.state == index, *label).clicked() {
                    self.state = index;
                }
            }
            if ui.button("Show toast").clicked() {
                self.toast_until = Some(Instant::now() + Duration::from_secs(3));
            }
        });
        ui.separator();
        egui::Frame::group(ui.style()).show(ui, |ui| match self.state {
            0 => {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Loading records…");
                });
            }
            1 => {
                ui.heading("Nothing here yet");
                ui.label("Create a record or adjust your filters to get started.");
                let _ = ui.button("Create record");
            }
            2 => {
                ui.colored_label(ui.visuals().error_fg_color, "Could not load records");
                ui.label("The service did not respond. Check your connection and retry.");
                if ui.button("Retry").clicked() {
                    self.state = 0;
                }
            }
            _ => {
                ui.colored_label(
                    Color32::from_rgb(80, 180, 110),
                    "✓ All records are up to date",
                );
                ui.label(RichText::new("Last synchronized just now").weak());
            }
        });
        if let Some(until) = self.toast_until {
            if until > Instant::now() {
                ui.ctx().request_repaint_after(Duration::from_millis(100));
                egui::Area::new(egui::Id::unique("feedback_toast"))
                    .anchor(egui::Align2::RIGHT_BOTTOM, [-16.0, -16.0])
                    .show(ui.ctx(), |ui| {
                        egui::Frame::popup(ui.style())
                            .show(ui, |ui| ui.label("Saved successfully"));
                    });
            } else {
                self.toast_until = None;
            }
        }
    }
}
