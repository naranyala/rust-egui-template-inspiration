//! Pattern: input widgets at a glance.
//!
//! Sliders, drag-values, toggles, radio groups, combo boxes and color pickers
//! — the widgets you reach for when the user edits a value in place.

use super::{Category, Demo};
use egui::{Color32, Grid, Ui};

#[derive(Default)]
pub struct Inputs {
    gain: f32,
    retries: u32,
    temperature: f64,
    verbose: bool,
    mode: Mode,
    priority: Priority,
    tint: Color32,
    rgb: [f32; 3],
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
enum Mode {
    #[default]
    Fast,
    Balanced,
    Precise,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
enum Priority {
    #[default]
    Low,
    Normal,
    High,
}

impl Demo for Inputs {
    fn name(&self) -> &'static str {
        "Input widgets"
    }

    fn description(&self) -> &'static str {
        "Sliders (linear, stepped, logarithmic), drag values, toggles, radio \
         groups, combo boxes and color pickers."
    }

    fn category(&self) -> Category {
        Category::Input
    }

    fn ui(&mut self, ui: &mut Ui) {
        Grid::new("inputs_grid")
            .num_columns(2)
            .spacing([24.0, 10.0])
            .show(ui, |ui| {
                ui.label("Gain");
                ui.add(
                    egui::Slider::new(&mut self.gain, 0.0..=1.0)
                        .fixed_decimals(2)
                        .show_value(true),
                );
                ui.end_row();

                ui.label("Retries (drag the number)");
                ui.add(egui::DragValue::new(&mut self.retries).range(0..=10));
                ui.end_row();

                // Logarithmic sliders are great for wide ranges.
                ui.label("Temperature");
                ui.add(
                    egui::Slider::new(&mut self.temperature, 0.1..=10_000.0)
                        .logarithmic(true)
                        .suffix(" K")
                        .fixed_decimals(1),
                );
                ui.end_row();

                ui.label("Toggle");
                ui.checkbox(&mut self.verbose, "Verbose logging");
                ui.end_row();

                ui.label("Mode");
                ui.horizontal(|ui| {
                    for (mode, label) in [
                        (Mode::Fast, "Fast"),
                        (Mode::Balanced, "Balanced"),
                        (Mode::Precise, "Precise"),
                    ] {
                        ui.selectable_value(&mut self.mode, mode, label);
                    }
                });
                ui.end_row();

                ui.label("Priority (radio)");
                ui.horizontal(|ui| {
                    for (priority, label) in [
                        (Priority::Low, "Low"),
                        (Priority::Normal, "Normal"),
                        (Priority::High, "High"),
                    ] {
                        ui.radio_value(&mut self.priority, priority, label);
                    }
                });
                ui.end_row();

                ui.label("Color");
                ui.horizontal(|ui| {
                    ui.color_edit_button_srgba(&mut self.tint);
                    ui.color_edit_button_rgb(&mut self.rgb);
                    ui.label(format!("{:?}", self.rgb));
                });
                ui.end_row();
            });

        ui.add_space(12.0);

        // A read-out of the current values: cheap "state at a glance" panel.
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.monospace(format!("gain        = {:.2}", self.gain));
            ui.monospace(format!("retries     = {}", self.retries));
            ui.monospace(format!("temperature = {:.1} K", self.temperature));
            ui.monospace(format!("mode        = {:?}", self.mode));
            ui.monospace(format!("priority    = {:?}", self.priority));
            ui.monospace(format!("tint        = {:?}", self.tint));
        });

        ui.add_space(8.0);

        // Reading input state is just as common as writing it: `Ui::input`
        // gives read access to the current frame's input.
        ui.horizontal(|ui| {
            ui.label("Currently held keys:");
            let keys = ui.input(|i| {
                [egui::Key::A, egui::Key::S, egui::Key::D, egui::Key::F]
                    .iter()
                    .filter(|key| i.key_down(**key))
                    .map(|key| format!("{key:?}"))
                    .collect::<Vec<_>>()
            });
            if keys.is_empty() {
                ui.weak("none — try holding A/S/D/F");
            } else {
                ui.strong(keys.join(", "));
            }
        });
    }
}

impl Inputs {
    /// Unused, but a handy reference for the "enum + combo box" pattern.
    #[allow(dead_code)]
    fn combo(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_label("Mode")
            .selected_text(format!("{:?}", self.mode))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.mode, Mode::Fast, "Fast");
                ui.selectable_value(&mut self.mode, Mode::Balanced, "Balanced");
                ui.selectable_value(&mut self.mode, Mode::Precise, "Precise");
            });
    }
}
