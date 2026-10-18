//! Pattern: dashboard layout.
//!
//! A responsive collection of cards: a compact toolbar, KPI tiles, and a
//! two-column content area. This is a useful starting point for monitoring,
//! analytics, and admin screens.

use super::{Category, Demo};
use egui::{Align, Color32, Layout, RichText, Ui};

#[derive(Default)]
pub struct Dashboard {
    compact: bool,
    show_activity: bool,
}

impl Demo for Dashboard {
    fn name(&self) -> &'static str {
        "Dashboard"
    }

    fn description(&self) -> &'static str {
        "Responsive KPI cards, summary panels, toolbar controls, and activity content."
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Operations dashboard");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.checkbox(&mut self.compact, "Compact");
                ui.checkbox(&mut self.show_activity, "Activity");
            });
        });
        ui.separator();

        let gap = if self.compact { 6.0 } else { 12.0 };
        ui.spacing_mut().item_spacing.x = gap;
        ui.horizontal_wrapped(|ui| {
            for (label, value, change, color) in [
                (
                    "Revenue",
                    "$84.2k",
                    "+12.4%",
                    Color32::from_rgb(70, 180, 120),
                ),
                (
                    "Active users",
                    "12,480",
                    "+8.1%",
                    Color32::from_rgb(80, 150, 235),
                ),
                (
                    "Conversion",
                    "6.8%",
                    "+1.2%",
                    Color32::from_rgb(190, 145, 65),
                ),
                ("Incidents", "3", "-42%", Color32::from_rgb(210, 90, 90)),
            ] {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.set_min_width(if self.compact { 130.0 } else { 160.0 });
                    ui.weak(label);
                    ui.heading(value);
                    ui.colored_label(color, change);
                });
            }
        });

        ui.add_space(gap);
        ui.columns(2, |columns| {
            egui::Frame::group(columns[0].style()).show(&mut columns[0], |ui| {
                ui.strong("Traffic overview");
                ui.separator();
                for (label, amount) in [
                    ("Mon", 0.42),
                    ("Tue", 0.68),
                    ("Wed", 0.54),
                    ("Thu", 0.82),
                    ("Fri", 0.73),
                ] {
                    ui.horizontal(|ui| {
                        ui.add_sized([34.0, 16.0], egui::Label::new(label));
                        ui.add(egui::ProgressBar::new(amount).desired_width(ui.available_width()));
                    });
                }
            });
            egui::Frame::group(columns[1].style()).show(&mut columns[1], |ui| {
                ui.strong("System health");
                ui.separator();
                for (service, state) in [
                    ("API", "Operational"),
                    ("Database", "Operational"),
                    ("Search", "Degraded"),
                    ("Billing", "Operational"),
                ] {
                    ui.horizontal(|ui| {
                        ui.label(service);
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.weak(state);
                        });
                    });
                }
            });
        });

        if self.show_activity {
            ui.add_space(gap);
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.strong("Recent activity");
                ui.separator();
                for line in [
                    "Deployment completed · 2 minutes ago",
                    "New team member invited · 18 minutes ago",
                    "Weekly report generated · 1 hour ago",
                ] {
                    ui.label(RichText::new(line).weak());
                }
            });
        }
    }
}
