//! Pattern: grouped lists and bulk selection.
//!
//! Checkboxes with per-group and global select controls, plus a bulk-action
//! bar that appears once something is selected. A common data-surface pattern
//! for mail, file managers, and admin tables.

use super::{Category, Demo};
use egui::Ui;

struct Item {
    name: &'static str,
    selected: bool,
}

#[derive(Default)]
pub struct BulkSelection {
    inbox: Vec<Item>,
    archive: Vec<Item>,
    last_action: Option<&'static str>,
}

impl BulkSelection {
    fn seed() -> (Vec<Item>, Vec<Item>) {
        let inbox = [
            "Welcome email",
            "Invoice #1042",
            "Team update",
            "Release notes",
        ]
        .into_iter()
        .map(|name| Item {
            name,
            selected: false,
        })
        .collect();
        let archive = ["Old receipt", "Archived thread"]
            .into_iter()
            .map(|name| Item {
                name,
                selected: false,
            })
            .collect();
        (inbox, archive)
    }

    fn count(items: &[Item]) -> usize {
        items.iter().filter(|item| item.selected).count()
    }
}

impl Demo for BulkSelection {
    fn name(&self) -> &'static str {
        "Grouped bulk selection"
    }

    fn description(&self) -> &'static str {
        "Grouped checkbox lists with select-all, per-group, and bulk action bar."
    }

    fn category(&self) -> Category {
        Category::Widgets
    }

    fn ui(&mut self, ui: &mut Ui) {
        if self.inbox.is_empty() && self.archive.is_empty() {
            let (inbox, archive) = Self::seed();
            self.inbox = inbox;
            self.archive = archive;
        }

        let total_selected = Self::count(&self.inbox) + Self::count(&self.archive);

        ui.horizontal(|ui| {
            ui.heading("Messages");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.weak(format!("{} selected", total_selected));
            });
        });
        ui.separator();

        // Bulk action bar: only meaningful when something is selected.
        if total_selected > 0 {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.strong(format!("{total_selected} selected"));
                    if ui.button("Archive").clicked() {
                        self.inbox.retain(|item| !item.selected);
                        self.archive.retain(|item| !item.selected);
                        self.last_action = Some("archive");
                    }
                    if ui.button("Delete").clicked() {
                        self.inbox.retain(|item| !item.selected);
                        self.archive.retain(|item| !item.selected);
                        self.last_action = Some("delete");
                    }
                    if ui.small_button("Clear").clicked() {
                        for item in self.inbox.iter_mut().chain(self.archive.iter_mut()) {
                            item.selected = false;
                        }
                    }
                });
            });
            ui.add_space(8.0);
        }

        let groups = [("Inbox", &mut self.inbox), ("Archive", &mut self.archive)];
        for (title, items) in groups {
            let mut all = !items.is_empty() && items.iter().all(|item| item.selected);
            ui.horizontal(|ui| {
                ui.strong(title);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.checkbox(&mut all, "All").changed() {
                        for item in items.iter_mut() {
                            item.selected = all;
                        }
                    }
                });
            });
            for item in items.iter_mut() {
                ui.checkbox(&mut item.selected, item.name);
            }
            ui.add_space(6.0);
        }

        if let Some(action) = self.last_action {
            ui.weak(format!("Last bulk action: {action}"));
        }
    }
}
