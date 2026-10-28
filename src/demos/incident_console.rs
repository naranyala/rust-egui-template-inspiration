use super::{Category, Demo};
use egui::{Align, Color32, Layout, Ui};

#[derive(Default)]
pub struct AccessControl {
    selected_role: usize,
    selected_user: usize,
}

impl Demo for AccessControl {
    fn name(&self) -> &'static str {
        "Access control matrix"
    }

    fn description(&self) -> &'static str {
        "Role-based permission review with user, resource, and policy coverage for enterprise access controls."
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Access control");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.button("Review policy");
            });
        });
        ui.separator();

        let roles = ["System admin", "Project lead", "Analyst", "Viewer"];
        let users = ["Nia Jones", "Liam Chen", "Ava Singh", "Omar Ali"];

        ui.columns(3, |columns| {
            egui::Frame::group(columns[0].style()).show(&mut columns[0], |ui| {
                ui.strong("Roles");
                ui.separator();
                for (index, role) in roles.iter().enumerate() {
                    if ui
                        .selectable_label(self.selected_role == index, *role)
                        .clicked()
                    {
                        self.selected_role = index;
                    }
                }
            });

            egui::Frame::group(columns[1].style()).show(&mut columns[1], |ui| {
                ui.strong("Users");
                ui.separator();
                for (index, user) in users.iter().enumerate() {
                    if ui
                        .selectable_label(self.selected_user == index, *user)
                        .clicked()
                    {
                        self.selected_user = index;
                    }
                }
            });

            egui::Frame::group(columns[2].style()).show(&mut columns[2], |ui| {
                ui.strong("Permissions");
                ui.separator();

                let matrix = [
                    ("Projects", true, true, false, false),
                    ("Billing", false, true, true, false),
                    ("Users", true, true, false, false),
                    ("Reports", true, false, true, true),
                ];

                egui::Grid::new("access_grid")
                    .spacing([8.0, 6.0])
                    .show(ui, |ui| {
                        ui.weak("Resource");
                        ui.weak("Read");
                        ui.weak("Write");
                        ui.weak("Admin");
                        ui.weak("Access");
                        ui.end_row();

                        for (resource, read, write, admin, access) in matrix {
                            ui.label(resource);
                            ui.colored_label(if read { Color32::GREEN } else { Color32::from_rgb(120, 120, 120) }, if read { "✓" } else { "—" });
                            ui.colored_label(if write { Color32::from_rgb(90, 150, 230) } else { Color32::from_rgb(120, 120, 120) }, if write { "✓" } else { "—" });
                            ui.colored_label(if admin { Color32::from_rgb(220, 110, 90) } else { Color32::from_rgb(120, 120, 120) }, if admin { "✓" } else { "—" });
                            ui.colored_label(if access { Color32::from_rgb(80, 170, 120) } else { Color32::from_rgb(120, 120, 120) }, if access { "Granted" } else { "Blocked" });
                            ui.end_row();
                        }
                    });
            });
        });
    }
}
