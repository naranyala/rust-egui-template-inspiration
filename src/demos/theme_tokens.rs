use super::{Category, Demo};
use egui::{Color32, Frame, RichText, Ui};

#[derive(Default)]
pub struct ThemeTokens {
    accent: usize,
    draft: String,
}

impl Demo for ThemeTokens {
    fn name(&self) -> &'static str {
        "Theme tokens & focus"
    }

    fn description(&self) -> &'static str {
        "Color tokens, accent variants, and high-contrast focus states for polished desktop UIs."
    }

    fn category(&self) -> Category {
        Category::Widgets
    }

    fn ui(&mut self, ui: &mut Ui) {
        let accent = match self.accent {
            0 => Color32::from_rgb(76, 130, 255),
            1 => Color32::from_rgb(255, 132, 98),
            2 => Color32::from_rgb(76, 180, 120),
            _ => Color32::from_rgb(76, 130, 255),
        };

        ui.heading("Theme tokens & focus states");
        ui.label(
            "Consistent colors, clear hierarchy, and visible focus outlines improve both brand polish and keyboard accessibility.",
        );
        ui.add_space(8.0);

        ui.horizontal_wrapped(|ui| {
            for (name, color) in [
                ("Primary", Color32::from_rgb(76, 130, 255)),
                ("Success", Color32::from_rgb(76, 180, 120)),
                ("Warning", Color32::from_rgb(220, 170, 86)),
                ("Danger", Color32::from_rgb(212, 92, 92)),
                ("Neutral", Color32::from_rgb(105, 112, 132)),
            ] {
                ui.group(|ui| {
                    ui.set_min_size(egui::vec2(110.0, 70.0));
                    ui.painter().rect_filled(ui.max_rect(), 12.0, color);
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), 0.0),
                        egui::Layout::bottom_up(egui::Align::LEFT),
                        |ui| {
                            ui.label(name);
                        },
                    );
                });
            }
        });

        ui.add_space(12.0);
        ui.horizontal(|ui| {
            ui.label("Accent");
            ui.selectable_value(&mut self.accent, 0, "Ocean");
            ui.selectable_value(&mut self.accent, 1, "Sunset");
            ui.selectable_value(&mut self.accent, 2, "Forest");
        });

        ui.add_space(12.0);
        Frame::group(ui.style()).show(ui, |ui| {
            ui.horizontal(|ui| {
                let primary = egui::Button::new("Primary action").fill(accent);
                ui.add(primary);
                ui.add(egui::Button::new("Secondary"));
                ui.add(egui::Button::new("Danger").fill(Color32::from_rgb(212, 92, 92)));
            });

            ui.add_space(10.0);
            ui.label(RichText::new("Keyboard focus example").strong());
            ui.horizontal(|ui| {
                ui.label("Project name");
                ui.add(
                    egui::TextEdit::singleline(&mut self.draft)
                        .hint_text("Focus ring should remain obvious here")
                        .desired_width(260.0),
                );
                ui.add(egui::Button::new("Save").fill(accent));
            });
            ui.add_space(4.0);
            ui.label(
                RichText::new(
                    "Use a visible stroke, clear contrast, and consistent spacing so keyboard users are never unsure where focus landed.",
                )
                .small()
                .weak(),
            );
        });
    }
}
