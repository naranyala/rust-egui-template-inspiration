//! Pattern: vertical navigation rail + breadcrumbs.
//!
//! A compact icon rail (narrow, vertical) paired with breadcrumb navigation.
//! Common in IDEs, email clients, and settings-heavy apps where horizontal
//! space is at a premium.

use super::{Category, Demo};
use egui::{Layout, RichText, Ui};

#[derive(Default)]
pub struct NavRail {
    section: usize,
    crumb: usize,
}

impl Demo for NavRail {
    fn name(&self) -> &'static str {
        "Nav rail + breadcrumbs"
    }

    fn description(&self) -> &'static str {
        "Narrow vertical icon rail plus breadcrumb path with active-step highlighting."
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        // Breadcrumbs above the rail: "Section / Subsection / Item".
        let crumbs = ["Library", "Documents", "Q4 Report"];
        ui.horizontal(|ui| {
            for (index, crumb) in crumbs.iter().enumerate() {
                let active = index == self.crumb;
                let text = if active {
                    RichText::new(*crumb).strong()
                } else {
                    RichText::new(*crumb)
                };
                if ui.link(text).clicked() {
                    self.crumb = index;
                }
                if index + 1 < crumbs.len() {
                    ui.weak(" › ");
                }
            }
        });
        ui.add_space(4.0);

        egui::Panel::left("nav_rail")
            .exact_size(64.0)
            .resizable(false)
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    for (index, (icon, label)) in [
                        ("🏠", "Home"),
                        ("📁", "Files"),
                        ("⭐", "Starred"),
                        ("⚙️", "Settings"),
                    ]
                    .iter()
                    .enumerate()
                    {
                        let selected = self.section == index;
                        if ui
                            .add(
                                egui::Button::new(RichText::new(*icon).size(18.0))
                                    .fill(if selected {
                                        ui.visuals().selection.bg_fill
                                    } else {
                                        egui::Color32::TRANSPARENT
                                    })
                                    .min_size(egui::vec2(44.0, 44.0)),
                            )
                            .on_hover_text(*label)
                            .clicked()
                        {
                            self.section = index;
                        }
                    }
                });
            });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.with_layout(Layout::top_down(egui::Align::Min), |ui| {
                ui.heading(["Home", "Files", "Starred", "Settings"][self.section]);
                ui.separator();
                ui.label(
                    RichText::new(format!("Breadcrumb: {}", crumbs[..=self.crumb].join(" › ")))
                        .weak(),
                );
                ui.add_space(8.0);
                for i in 0..5 {
                    ui.label(format!("Item {} of {}", i + 1, 5));
                }
            });
        });
    }
}
