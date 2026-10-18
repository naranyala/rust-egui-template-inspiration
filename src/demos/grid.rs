//! Pattern: grid layout.
//!
//! `egui::Grid` is the right tool when widgets need to line up in rows and
//! columns (form fields, key/value pairs, property tables). It handles column
//! widths, alignment and spacing for you.

use super::{Category, Demo};
use egui::{Grid, RichText, Ui};

#[derive(Default)]
pub struct Grids {
    show_borders: bool,
    spacing: f32,
}

impl Demo for Grids {
    fn name(&self) -> &'static str {
        "Grid layout"
    }

    fn description(&self) -> &'static str {
        "Aligned rows/columns with egui::Grid: forms, key/value tables, and \
         nested grids with per-column styling."
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_borders, "Borders");
            ui.add(egui::Slider::new(&mut self.spacing, 0.0..=24.0).text("spacing"));
        });
        ui.add_space(4.0);

        // Basic key/value grid.
        ui.strong("Key / value");
        Grid::new("kv_grid")
            .num_columns(2)
            .spacing([self.spacing, 4.0])
            .striped(true)
            .show(ui, |ui| {
                for (k, v) in [
                    ("Name", "egui"),
                    ("Version", "0.37"),
                    ("License", "MIT OR Apache-2.0"),
                    ("Renderer", "wgpu / glow"),
                ] {
                    ui.label(RichText::new(k).strong());
                    ui.label(v);
                    ui.end_row();
                }
            });
        ui.add_space(8.0);

        // Form grid with mixed widget types.
        ui.strong("Form");
        Grid::new("form_grid")
            .num_columns(2)
            .spacing([self.spacing, 6.0])
            .show(ui, |ui| {
                ui.label("Username");
                ui.text_edit_singleline(&mut String::new());
                ui.end_row();

                ui.label("Password");
                ui.add(egui::TextEdit::singleline(&mut String::new()).password(true));
                ui.end_row();

                ui.label("Role");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut 0i32, 0, "Admin");
                    ui.selectable_value(&mut 0i32, 1, "User");
                });
                ui.end_row();

                ui.label("Bio");
                ui.text_edit_multiline(&mut String::new());
                ui.end_row();
            });
        ui.add_space(8.0);

        // Nested grid: a grid inside a grid cell.
        ui.strong("Nested grids");
        Grid::new("outer")
            .num_columns(2)
            .spacing([self.spacing, 6.0])
            .show(ui, |ui| {
                ui.label("Left");
                Grid::new("inner")
                    .num_columns(2)
                    .spacing([4.0, 2.0])
                    .show(ui, |ui| {
                        for i in 0..3 {
                            ui.label(format!("r{i}c0"));
                            ui.label(format!("r{i}c1"));
                            ui.end_row();
                        }
                    });
                ui.end_row();

                ui.label("Right");
                ui.vertical(|ui| {
                    ui.label("line 1");
                    ui.label("line 2");
                });
                ui.end_row();
            });

        if self.show_borders {
            ui.weak("Borders toggle is cosmetic here; enable via Grid::show");
        }
    }
}
