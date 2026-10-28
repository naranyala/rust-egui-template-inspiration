use super::{Category, Demo};
use egui::{Align, Color32, Layout, RichText, Ui};

#[derive(Default)]
pub struct IncidentConsole {
    selected: usize,
}

impl Demo for IncidentConsole {
    fn name(&self) -> &'static str {
        "Incident console"
    }

    fn description(&self) -> &'static str {
        "Operational incident triage with severity, ownership, and timeline views."
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Incident console");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.button("Refresh");
            });
        });
        ui.separator();

        let incidents = [
            (
                "API latency spike",
                "P1",
                Color32::from_rgb(220, 110, 90),
                "Auth gateway • N. Bell",
                "Customers seeing 2.4s latency; rollback plan active.",
            ),
            (
                "Database failover",
                "P2",
                Color32::from_rgb(210, 160, 70),
                "Payments • D. Wu",
                "Replica promotion in progress; write traffic protected.",
            ),
            (
                "Search index backlog",
                "P3",
                Color32::from_rgb(100, 160, 220),
                "Discovery • R. Singh",
                "Recent indexing lag is under threshold and monitored.",
            ),
        ];

        self.selected = self.selected.min(incidents.len().saturating_sub(1));

        egui::Panel::left("incident_list")
            .default_size(240.0)
            .show(ui, |ui| {
                ui.strong("Active incidents");
                ui.separator();
                for (index, (name, severity, color, owner, _)) in incidents.iter().enumerate() {
                    if ui
                        .selectable_label(self.selected == index, *name)
                        .clicked()
                    {
                        self.selected = index;
                    }
                    ui.horizontal(|ui| {
                        ui.colored_label(*color, *severity);
                        ui.weak(owner);
                    });
                    ui.add_space(6.0);
                }
            });

        egui::Panel::right("incident_timeline")
            .default_size(240.0)
            .show(ui, |ui| {
                ui.strong("Timeline");
                ui.separator();
                for line in [
                    "09:12 alert fired",
                    "09:18 owner assigned",
                    "09:29 rollback initiated",
                    "09:42 mitigation validated",
                ] {
                    ui.horizontal(|ui| {
                        ui.small("•");
                        ui.label(RichText::new(line).weak());
                    });
                }
            });

        egui::CentralPanel::default().show(ui, |ui| {
            let (name, severity, color, owner, detail) = incidents[self.selected];

            ui.horizontal(|ui| {
                ui.heading(name);
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.colored_label(color, severity);
                });
            });
            ui.separator();

            ui.horizontal(|ui| {
                ui.small("Owner");
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.weak(owner);
                });
            });

            ui.add_space(8.0);
            ui.label("Summary");
            ui.label(RichText::new(detail).weak());
            ui.add_space(10.0);

            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.strong("Impact");
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Services affected");
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.weak("auth, api, web");
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("Customers impacted");
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.weak("17% of traffic");
                    });
                });
            });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Acknowledge").clicked() {
                    // no-op state for demo illustration
                }
                if ui.button("Escalate").clicked() {
                    // no-op state for demo illustration
                }
                if ui.button("Resolve").clicked() {
                    // no-op state for demo illustration
                }
            });
        });
    }
}
