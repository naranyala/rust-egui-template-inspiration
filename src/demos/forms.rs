//! Pattern: a validated form.
//!
//! Covers the everyday combination of `Grid` label/control rows, `TextEdit`
//! with hints, a `ComboBox`, validation errors rendered inline, and
//! submit/reset actions.

use super::{Category, Demo};
use egui::{ComboBox, Grid, RichText, Ui};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Role {
    Viewer,
    Editor,
    Admin,
}

impl Role {
    const ALL: [Role; 3] = [Role::Viewer, Role::Editor, Role::Admin];

    fn label(self) -> &'static str {
        match self {
            Role::Viewer => "Viewer",
            Role::Editor => "Editor",
            Role::Admin => "Admin",
        }
    }
}

#[derive(Default)]
pub struct Forms {
    name: String,
    email: String,
    password: String,
    show_password: bool,
    /// `None` = no role picked yet.
    role: Option<Role>,
    subscribe: bool,
    /// Last submit result, shown as a banner.
    status: Option<Result<(), String>>,
}

impl Forms {
    /// Validate and either record an error or a success banner.
    fn submit(&mut self) {
        let mut errors = Vec::new();

        if self.name.trim().is_empty() {
            errors.push("Name is required.");
        }
        if !self.email.contains('@') || !self.email.contains('.') {
            errors.push("Email must look like an address.");
        }
        if self.password.len() < 8 {
            errors.push("Password must be at least 8 characters.");
        }
        if self.role.is_none() {
            errors.push("Pick a role.");
        }

        self.status = if errors.is_empty() {
            Some(Ok(()))
        } else {
            Some(Err(errors.join("\n")))
        };
    }

    /// A label in the left column, an optional error in the right one.
    fn row(ui: &mut Ui, label: &str, error: Option<&str>, add: impl FnOnce(&mut Ui)) {
        ui.label(label);
        ui.vertical(|ui| {
            add(ui);
            if let Some(error) = error {
                ui.colored_label(ui.visuals().error_fg_color, RichText::new(error).small());
            }
        });
        ui.end_row();
    }
}

impl Demo for Forms {
    fn name(&self) -> &'static str {
        "Form & validation"
    }

    fn description(&self) -> &'static str {
        "Grid-aligned label/control rows with inline validation errors and \
         submit/reset actions."
    }

    fn category(&self) -> Category {
        Category::Input
    }

    fn ui(&mut self, ui: &mut Ui) {
        let error_fg = ui.visuals().error_fg_color;

        // Only show errors for fields the user has already touched.
        let name_error = (!self.name.is_empty() && self.name.trim().is_empty())
            .then_some("Name cannot be blank.");
        let email_error =
            (!self.email.is_empty() && !self.email.contains('@')).then_some("Missing '@'.");
        let password_error =
            (self.password.len() < 8 && !self.password.is_empty()).then_some("Too short.");

        Grid::new("form_grid")
            .num_columns(2)
            .spacing([16.0, 10.0])
            .show(ui, |ui| {
                Self::row(ui, "Name", name_error, |ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.name)
                            .hint_text("Ada Lovelace")
                            .desired_width(220.0),
                    );
                });

                Self::row(ui, "Email", email_error, |ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.email)
                            .hint_text("ada@example.com")
                            .desired_width(220.0),
                    );
                });

                Self::row(ui, "Password", password_error, |ui| {
                    ui.horizontal(|ui| {
                        let edit = egui::TextEdit::singleline(&mut self.password)
                            .password(!self.show_password)
                            .desired_width(180.0);
                        ui.add(edit);
                        ui.checkbox(&mut self.show_password, "Show");
                    });
                });

                Self::row(ui, "Role", None, |ui| {
                    ComboBox::from_id_salt("form_role")
                        .selected_text(self.role.map(Role::label).unwrap_or("Choose…"))
                        .show_ui(ui, |ui| {
                            for role in Role::ALL {
                                ui.selectable_value(&mut self.role, Some(role), role.label());
                            }
                        });
                });

                Self::row(ui, "Newsletter", None, |ui| {
                    ui.checkbox(&mut self.subscribe, "Send me release notes");
                });
            });

        ui.add_space(12.0);

        ui.horizontal(|ui| {
            if ui
                .add(egui::Button::new("Submit").fill(ui.visuals().selection.bg_fill))
                .clicked()
            {
                self.submit();
            }
            if ui.button("Reset").clicked() {
                *self = Self::default();
            }
        });

        if let Some(status) = &self.status {
            ui.add_space(8.0);
            match status {
                Ok(()) => {
                    ui.colored_label(ui.visuals().widgets.active.bg_fill, "✓ Account created.");
                }
                Err(message) => {
                    // A framed, error-tinted box listing everything at once.
                    egui::Frame::popup(ui.style())
                        .stroke(egui::Stroke::new(1.0, error_fg))
                        .show(ui, |ui| {
                            ui.colored_label(error_fg, "Please fix the following:");
                            for line in message.lines() {
                                ui.colored_label(error_fg, format!("• {line}"));
                            }
                        });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_form_reports_all_required_fields() {
        let mut form = Forms::default();
        form.submit();

        let Some(Err(message)) = form.status else {
            panic!("empty form should be rejected");
        };
        assert!(message.contains("Name is required."));
        assert!(message.contains("Email must look like an address."));
        assert!(message.contains("Password must be at least 8 characters."));
        assert!(message.contains("Pick a role."));
    }

    #[test]
    fn complete_form_is_accepted() {
        let mut form = Forms {
            name: "Ada Lovelace".into(),
            email: "ada@example.com".into(),
            password: "correct horse".into(),
            role: Some(Role::Admin),
            ..Default::default()
        };
        form.submit();
        assert_eq!(form.status, Some(Ok(())));
    }
}
