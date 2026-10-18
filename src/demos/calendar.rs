//! Pattern: calendar / scheduler.
//!
//! A compact month-like scheduler with selected-day state and a day/week
//! toggle, useful as a starting point for bookings and planning screens.

use super::{Category, Demo};
use egui::{Align, Layout, RichText, Ui};

#[derive(Default)]
pub struct Calendar {
    selected_day: usize,
    week_view: bool,
}

impl Demo for Calendar {
    fn name(&self) -> &'static str {
        "Calendar / scheduler"
    }
    fn description(&self) -> &'static str {
        "Month and week planning surface with selected days and scheduled events."
    }
    fn category(&self) -> Category {
        Category::Data
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("October 2026");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.selectable_value(&mut self.week_view, true, "Week");
                ui.selectable_value(&mut self.week_view, false, "Month");
            });
        });
        ui.separator();
        if self.week_view {
            for (day, events) in [
                ("Mon 19", ["Design review", "Stand-up"]),
                ("Tue 20", ["Planning", "Launch check"]),
                ("Wed 21", ["Focus time", "Demo"]),
                ("Thu 22", ["Interview", "Retro"]),
                ("Fri 23", ["Release", ""]),
            ] {
                ui.horizontal(|ui| {
                    ui.add_sized([80.0, 20.0], egui::Label::new(RichText::new(day).strong()));
                    ui.label(events[0]);
                    if !events[1].is_empty() {
                        ui.separator();
                        ui.label(events[1]);
                    }
                });
            }
        } else {
            egui::Grid::new("calendar_grid")
                .num_columns(7)
                .spacing([8.0, 8.0])
                .show(ui, |ui| {
                    for day in ["M", "T", "W", "T", "F", "S", "S"] {
                        ui.strong(day);
                    }
                    ui.end_row();
                    for day in 1..=31 {
                        if ui
                            .selectable_label(self.selected_day == day, day.to_string())
                            .clicked()
                        {
                            self.selected_day = day;
                        }
                        if day % 7 == 0 {
                            ui.end_row();
                        }
                    }
                });
            ui.add_space(10.0);
            ui.label(format!(
                "Selected day: {}",
                if self.selected_day == 0 {
                    "none".to_owned()
                } else {
                    self.selected_day.to_string()
                }
            ));
        }
    }
}
