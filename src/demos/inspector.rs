//! Pattern: property inspector.
//!
//! A selectable object list paired with editable properties, a common layout
//! in editors, dashboards, design tools, and admin applications.

use super::{Category, Demo};
use egui::{Color32, Layout, Ui};

#[derive(Default)]
pub struct Inspector {
    selected: usize,
    read_only: bool,
    visible: bool,
    locked: bool,
    opacity: f32,
    color: Color32,
}

impl Demo for Inspector {
    fn name(&self) -> &'static str {
        "Property inspector"
    }
    fn description(&self) -> &'static str {
        "Selectable objects with editable or read-only properties in a side inspector."
    }
    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.strong("Scene editor");
            ui.checkbox(&mut self.read_only, "Read only");
        });
        ui.separator();
        egui::Panel::left("inspector_objects")
            .resizable(true)
            .default_size(180.0)
            .min_size(130.0)
            .show(ui, |ui| {
                ui.strong("Objects");
                ui.separator();
                for (index, object) in ["Header", "Hero image", "CTA button", "Footer"]
                    .iter()
                    .enumerate()
                {
                    if ui
                        .selectable_label(self.selected == index, *object)
                        .clicked()
                    {
                        self.selected = index;
                    }
                }
            });
        egui::Panel::right("inspector_properties")
            .resizable(true)
            .default_size(230.0)
            .min_size(180.0)
            .show(ui, |ui| {
                ui.strong("Properties");
                ui.separator();
                ui.add_enabled_ui(!self.read_only, |ui| {
                    ui.checkbox(&mut self.visible, "Visible");
                    ui.checkbox(&mut self.locked, "Locked");
                    ui.add(egui::Slider::new(&mut self.opacity, 0.0..=1.0).text("Opacity"));
                    ui.color_edit_button_srgba(&mut self.color);
                });
                if self.read_only {
                    ui.weak("Read-only mode disables editing controls.");
                }
            });
        egui::CentralPanel::default().show(ui, |ui| {
            ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(30.0);
                ui.heading(["Header", "Hero image", "CTA button", "Footer"][self.selected]);
                ui.label("Selected object preview");
            });
        });
    }
}
