//! Pattern: master-detail workspace.
//!
//! A common desktop application structure: a searchable list on the left and
//! the selected record's details on the right.

use super::{Category, Demo};
use egui::{Align, Layout, RichText, Ui};

#[derive(Default)]
pub struct MasterDetail {
    selected: usize,
    filter: String,
    show_metadata: bool,
    next_steps: [bool; 3],
}

impl Demo for MasterDetail {
    fn name(&self) -> &'static str {
        "Master–detail"
    }

    fn description(&self) -> &'static str {
        "Searchable master list with a selected item's detail and metadata panes."
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_metadata, "Show metadata");
            ui.weak("Select a record to update the detail view");
        });
        ui.add_space(4.0);

        egui::Panel::left("master_detail_list")
            .resizable(true)
            .default_size(220.0)
            .min_size(150.0)
            .show(ui, |ui| {
                ui.strong("Projects");
                ui.add(egui::TextEdit::singleline(&mut self.filter).hint_text("Filter projects…"));
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let needle = self.filter.to_lowercase();
                    for (index, (name, status, owner)) in [
                        ("Website redesign", "In progress", "Mira"),
                        ("Mobile release", "Planning", "Jon"),
                        ("Analytics refresh", "Review", "Ari"),
                        ("Support migration", "Complete", "Sam"),
                        ("Design system", "In progress", "Mira"),
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        if !needle.is_empty() && !name.to_lowercase().contains(&needle) {
                            continue;
                        }
                        if ui
                            .selectable_label(
                                self.selected == index,
                                format!("{name}\n{status} · {owner}"),
                            )
                            .clicked()
                        {
                            self.selected = index;
                        }
                    }
                });
            });

        egui::CentralPanel::default().show(ui, |ui| {
            let projects = [
                (
                    "Website redesign",
                    "In progress",
                    "Mira",
                    "Refresh the public website and improve the onboarding flow.",
                ),
                (
                    "Mobile release",
                    "Planning",
                    "Jon",
                    "Coordinate the next mobile release across product and engineering.",
                ),
                (
                    "Analytics refresh",
                    "Review",
                    "Ari",
                    "Consolidate dashboard metrics into a shared reporting model.",
                ),
                (
                    "Support migration",
                    "Complete",
                    "Sam",
                    "Move support workflows into the new service desk.",
                ),
                (
                    "Design system",
                    "In progress",
                    "Mira",
                    "Document reusable components and interaction patterns.",
                ),
            ];
            let (name, status, owner, summary) = projects[self.selected];

            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                ui.heading(name);
                ui.horizontal(|ui| {
                    ui.label(RichText::new(status).strong());
                    ui.separator();
                    ui.weak(format!("Owner: {owner}"));
                });
                ui.separator();
                ui.label(summary);
                ui.add_space(12.0);
                ui.strong("Next steps");
                for (index, step) in [
                    "Confirm scope with stakeholders",
                    "Review open tasks",
                    "Schedule the next checkpoint",
                ]
                .into_iter()
                .enumerate()
                {
                    ui.checkbox(&mut self.next_steps[index], step);
                }

                if self.show_metadata {
                    ui.add_space(12.0);
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.strong("Metadata");
                        ui.label("Created: 12 days ago");
                        ui.label("Last updated: 2 hours ago");
                        ui.label("Members: 6");
                    });
                }
            });
        });
    }
}
