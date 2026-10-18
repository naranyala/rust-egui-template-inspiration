//! Pattern: wizard / stepper flow.
//!
//! A multi-step workflow with a persistent step indicator and guarded
//! navigation. Replace the sample fields with the state for an onboarding,
//! import, or configuration flow.

use super::{Category, Demo};
use egui::{Color32, RichText, Ui};

#[derive(Default)]
pub struct Wizard {
    step: usize,
    name: String,
    destination: String,
    confirmed: bool,
}

impl Demo for Wizard {
    fn name(&self) -> &'static str {
        "Wizard / stepper"
    }

    fn description(&self) -> &'static str {
        "A guided multi-step flow with progress, validation, and back/next actions."
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        let steps = ["Basics", "Destination", "Review"];
        ui.horizontal(|ui| {
            for (index, label) in steps.iter().enumerate() {
                let active = self.step == index;
                let complete = index < self.step;
                let text = if complete {
                    format!("✓ {label}")
                } else {
                    (*label).to_owned()
                };
                ui.colored_label(
                    if active {
                        ui.visuals().selection.bg_fill
                    } else {
                        Color32::PLACEHOLDER
                    },
                    RichText::new(text).strong(),
                );
                if index + 1 < steps.len() {
                    ui.weak("→");
                }
            }
        });
        ui.separator();

        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.set_min_height(170.0);
            match self.step {
                0 => {
                    ui.heading("Name your workspace");
                    ui.label("This name will appear in the sidebar and window title.");
                    ui.add_space(12.0);
                    ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Workspace name"));
                }
                1 => {
                    ui.heading("Choose a destination");
                    ui.label("Select where the imported data should be stored.");
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        ui.label("Folder");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.destination)
                                .hint_text("~/Projects/example"),
                        );
                    });
                }
                _ => {
                    ui.heading("Review and finish");
                    ui.label(format!(
                        "Workspace: {}",
                        if self.name.is_empty() {
                            "(unnamed)"
                        } else {
                            &self.name
                        }
                    ));
                    ui.label(format!(
                        "Destination: {}",
                        if self.destination.is_empty() {
                            "(not selected)"
                        } else {
                            &self.destination
                        }
                    ));
                    ui.add_space(8.0);
                    ui.checkbox(&mut self.confirmed, "I have reviewed these settings");
                }
            }
        });

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            if ui
                .add_enabled(self.step > 0, egui::Button::new("Back"))
                .clicked()
            {
                self.step -= 1;
            }
            let can_continue = self.can_continue();
            if self.step < steps.len() - 1 {
                if ui
                    .add_enabled(can_continue, egui::Button::new("Next"))
                    .clicked()
                {
                    self.step += 1;
                }
            } else if ui
                .add_enabled(can_continue, egui::Button::new("Finish"))
                .clicked()
            {
                self.step = 0;
                self.confirmed = false;
            }
            ui.weak(format!("Step {} of {}", self.step + 1, steps.len()));
        });
    }
}

impl Wizard {
    fn can_continue(&self) -> bool {
        match self.step {
            0 => !self.name.trim().is_empty(),
            1 => !self.destination.trim().is_empty(),
            _ => self.confirmed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wizard_requires_each_step_before_advancing() {
        let mut wizard = Wizard::default();
        assert!(!wizard.can_continue());
        wizard.name = "Workspace".into();
        assert!(wizard.can_continue());
        wizard.step = 1;
        assert!(!wizard.can_continue());
        wizard.destination = "~/Projects/workspace".into();
        assert!(wizard.can_continue());
        wizard.step = 2;
        assert!(!wizard.can_continue());
        wizard.confirmed = true;
        assert!(wizard.can_continue());
    }
}
