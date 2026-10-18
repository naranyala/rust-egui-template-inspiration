//! Pattern: timeline / activity feed.
//!
//! A vertical event stream with date grouping and status markers.

use super::{Category, Demo};
use egui::{Color32, RichText, Ui};

#[derive(Default)]
pub struct Timeline {
    show_system: bool,
}

impl Demo for Timeline {
    fn name(&self) -> &'static str {
        "Timeline / activity"
    }
    fn description(&self) -> &'static str {
        "Vertical activity feed with date grouping, status markers, and details."
    }
    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Activity");
            ui.checkbox(&mut self.show_system, "Show system events");
        });
        ui.separator();
        egui::ScrollArea::vertical()
            .max_height(360.0)
            .show(ui, |ui| {
                let mut current_day = "";
                for (day, time, title, detail, color, system) in [
                    (
                        "Today",
                        "14:32",
                        "Deployment completed",
                        "Version 2.4.0 is live in production.",
                        Color32::from_rgb(80, 180, 110),
                        false,
                    ),
                    (
                        "Today",
                        "12:18",
                        "Mira commented",
                        "The updated empty state looks good.",
                        Color32::from_rgb(90, 145, 220),
                        false,
                    ),
                    (
                        "Today",
                        "09:04",
                        "Backup finished",
                        "Automated backup completed successfully.",
                        Color32::from_rgb(165, 165, 165),
                        true,
                    ),
                    (
                        "Yesterday",
                        "17:40",
                        "New milestone created",
                        "The Q4 launch milestone was added.",
                        Color32::from_rgb(190, 145, 65),
                        false,
                    ),
                    (
                        "Yesterday",
                        "11:20",
                        "Access policy changed",
                        "Two members were added to the project.",
                        Color32::from_rgb(210, 95, 95),
                        true,
                    ),
                ] {
                    if system && !self.show_system {
                        continue;
                    }
                    if day != current_day {
                        ui.add_space(8.0);
                        ui.label(RichText::new(day).strong());
                        current_day = day;
                    }
                    ui.horizontal(|ui| {
                        ui.add_sized([52.0, 20.0], egui::Label::new(RichText::new(time).weak()));
                        ui.colored_label(color, "●");
                        egui::Frame::group(ui.style()).show(ui, |ui| {
                            ui.strong(title);
                            ui.label(detail);
                        });
                    });
                }
            });
    }
}
