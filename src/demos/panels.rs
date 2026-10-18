//! Pattern: nested panels.
//!
//! The workhorse layout for desktop apps: fixed chrome (toolbars, sidebars,
//! status bars) around a central area that always fills whatever is left.
//!
//! In egui 0.37 the old `SidePanel`/`TopBottomPanel` types were unified into
//! [`egui::Panel`], which can be shown inside any `Ui` — not just the root.
//! Rules of thumb:
//!
//! * add panels outermost-first; the first one added is the outermost,
//! * always add `CentralPanel` **last**,
//! * a resizable panel needs its ui to consume space (a `ScrollArea`, a
//!   wrapping `Label`, a `TextEdit`, …).

use super::{Category, Demo};
use egui::{Align, Layout, RichText, Ui};

#[derive(Default)]
pub struct Panels {
    /// Drives the animated left panel.
    left_open: bool,
    show_separator_line: bool,
    /// Last toolbar action, shown in the bottom panel.
    status: Option<&'static str>,
}

impl Demo for Panels {
    fn name(&self) -> &'static str {
        "Panel layout"
    }

    fn description(&self) -> &'static str {
        "Resizable top/left/right/bottom panels around a central area, plus a \
         collapsible panel that slides in and out."
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        // The demo's own controls live in the parent Ui, before the sub-panels.
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.left_open, "Collapsible left panel");
            ui.checkbox(&mut self.show_separator_line, "Separator line");
        });
        ui.add_space(4.0);

        // Top toolbar inside this demo. `exact_size` = height for top/bottom.
        egui::Panel::top("demo_top")
            .exact_size(28.0)
            .show_separator_line(self.show_separator_line)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.strong("Toolbar");
                    let actions = [
                        (ui.button("New"), "New"),
                        (ui.button("Open"), "Open"),
                        (ui.button("Save"), "Save"),
                    ];
                    for (response, label) in actions {
                        if response.clicked() {
                            self.status = Some(label);
                        }
                    }
                });
            });

        // A collapsible panel slides open/closed with an animation, and can be
        // dragged back open from its thin grab handle.
        egui::Panel::left("demo_left_collapsible")
            .resizable(true)
            .default_size(140.0)
            .min_size(90.0)
            .show_collapsible(ui, &mut self.left_open, |ui| {
                ui.strong("Explorer");
                ui.separator();
                for item in ["src/", "  app.rs", "  demos/", "Cargo.toml", "README.md"] {
                    ui.label(item);
                }
            });

        egui::Panel::left("demo_left")
            .resizable(true)
            .default_size(160.0)
            .min_size(100.0)
            .show(ui, |ui| {
                ui.strong("Tools");
                ui.separator();
                // Fill the panel so it can be resized by dragging.
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for i in 0..40 {
                        ui.label(format!("Tool {i}"));
                    }
                });
            });

        egui::Panel::right("demo_right")
            .resizable(true)
            .default_size(150.0)
            .min_size(90.0)
            .show(ui, |ui| {
                ui.strong("Inspector");
                ui.separator();
                ui.label("Properties of the selected item appear here.");
            });

        egui::Panel::bottom("demo_bottom")
            .resizable(true)
            .default_size(40.0)
            .min_size(24.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.strong("Output");
                    match self.status {
                        Some(action) => ui.weak(format!("Last action: {action}")),
                        None => ui.weak("3 warnings, 0 errors"),
                    };
                });
            });

        // Must come last: it fills whatever space the panels left over.
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Central area");
            ui.label("Drag any panel edge to resize it.");
            ui.label("Everything else stays pinned to the window edges.");
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Tip:").strong());
                ui.label("panels can also be nested inside other panels.");
            });
            ui.add_space(4.0);

            // Two other ways to lay widgets out horizontally:
            ui.separator();
            ui.strong("Columns / wrapping");
            ui.columns(3, |columns| {
                for (i, column) in columns.iter_mut().enumerate() {
                    column.label(format!("Column {i}"));
                }
            });
            ui.with_layout(
                Layout::left_to_right(Align::Min).with_main_wrap(true),
                |ui| {
                    for i in 0..12 {
                        ui.add(egui::Button::new(format!("Wrapped {i}")));
                    }
                },
            );
        });
    }
}
