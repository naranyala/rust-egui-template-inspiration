//! Pattern: dockable / floating windows.
//!
//! A different layout paradigm from the panel-based demos: movable, closable
//! `egui::Window`s that overlay a canvas. This is how IDEs, editors and tool
//! palettes present optional panes that the user arranges freely.

use super::{Category, Demo};
use egui::{RichText, Ui};

#[derive(Default)]
pub struct Floating {
    show_output: bool,
    show_search: bool,
    show_tools: bool,
    search: String,
    selected_tool: usize,
}

impl Demo for Floating {
    fn name(&self) -> &'static str {
        "Floating windows"
    }

    fn description(&self) -> &'static str {
        "Movable, closable overlay windows (IDE-style) on top of a canvas."
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_output, "Output");
            ui.checkbox(&mut self.show_search, "Search");
            ui.checkbox(&mut self.show_tools, "Tools");
        });
        ui.add_space(4.0);

        // The canvas is the "workspace" the windows float above.
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            ui.set_min_height(260.0);
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("Canvas / editor area").weak().italics());
            });
        });

        if self.show_output {
            egui::Window::new("Output")
                .id(egui::Id::unique("floating_output"))
                .collapsible(true)
                .resizable(true)
                .anchor(egui::Align2::LEFT_BOTTOM, [16.0, -16.0])
                .default_size([260.0, 120.0])
                .show(ui.ctx(), |ui| {
                    ui.label("build finished in 1.2s");
                    ui.label("0 warnings, 0 errors");
                });
        }

        if self.show_search {
            egui::Window::new("Search")
                .id(egui::Id::unique("floating_search"))
                .collapsible(true)
                .resizable(true)
                .anchor(egui::Align2::RIGHT_TOP, [-16.0, 16.0])
                .default_size([240.0, 140.0])
                .show(ui.ctx(), |ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.search).hint_text("Type to search…"),
                    );
                    ui.separator();
                    for hit in ["app.rs", "main.rs", "theme.rs"] {
                        if self.search.is_empty() || hit.contains(&self.search) {
                            let _ = ui.selectable_label(false, hit);
                        }
                    }
                });
        }

        if self.show_tools {
            egui::Window::new("Tools")
                .id(egui::Id::unique("floating_tools"))
                .collapsible(true)
                .resizable(true)
                .anchor(egui::Align2::LEFT_TOP, [16.0, 16.0])
                .default_size([160.0, 180.0])
                .show(ui.ctx(), |ui| {
                    ui.vertical(|ui| {
                        for (index, tool) in ["Move", "Scale", "Rotate", "Erase"].iter().enumerate()
                        {
                            if ui
                                .selectable_label(self.selected_tool == index, *tool)
                                .clicked()
                            {
                                self.selected_tool = index;
                            }
                        }
                    });
                });
        }

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.weak("Tip:");
            ui.label("windows remember their position and can be dragged by their title bar.");
        });
    }
}
