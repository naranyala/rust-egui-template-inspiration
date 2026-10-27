use super::{Category, Demo};
use egui::{Grid, RichText, Ui};

#[derive(Default)]
pub struct AccessibilityFocus {
    name: String,
    email: String,
    alerts: bool,
    digest: bool,
    priority: usize,
}

impl Demo for AccessibilityFocus {
    fn name(&self) -> &'static str {
        "Accessibility & focus"
    }

    fn description(&self) -> &'static str {
        "Visible focus rings, clear labels, and keyboard-first choices for real desktop apps."
    }

    fn category(&self) -> Category {
        Category::Widgets
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.heading("Accessibility checklist");
        ui.label(
            "Good desktop apps keep labels obvious, actions predictable, and focus visibly connected to the active control.",
        );
        ui.add_space(8.0);

        Grid::new("a11y_grid")
            .num_columns(2)
            .spacing([18.0, 10.0])
            .show(ui, |ui| {
                ui.label("Display name");
                ui.add(
                    egui::TextEdit::singleline(&mut self.name)
                        .hint_text("Ada Lovelace")
                        .desired_width(220.0),
                );
                ui.end_row();

                ui.label("Email");
                ui.add(
                    egui::TextEdit::singleline(&mut self.email)
                        .hint_text("ada@example.com")
                        .desired_width(220.0),
                );
                ui.end_row();

                ui.label("Notifications");
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.alerts, "Alerts");
                    ui.checkbox(&mut self.digest, "Digest");
                });
                ui.end_row();

                ui.label("Priority");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.priority, 0, "Low");
                    ui.selectable_value(&mut self.priority, 1, "Normal");
                    ui.selectable_value(&mut self.priority, 2, "High");
                });
                ui.end_row();
            });

        ui.add_space(12.0);
        ui.label(RichText::new("Focus guidance").strong());
        ui.horizontal(|ui| {
            ui.button("Save changes");
            ui.button("Cancel");
        });

        ui.add_space(8.0);
        ui.label(
            RichText::new(
                "Strong contrast, labels next to controls, and consistent focus cues make the UI easier for keyboard and assistive-tech users.",
            )
            .small()
            .weak(),
        );
    }
}
