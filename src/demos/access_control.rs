use super::{Category, Demo};
use egui::{Align, Color32, Layout, RichText, Ui};

#[derive(Default)]
pub struct ApprovalCenter {
    selected: usize,
    priority_only: bool,
    decision: Option<&'static str>,
}

impl Demo for ApprovalCenter {
    fn name(&self) -> &'static str {
        "Approval center"
    }

    fn description(&self) -> &'static str {
        "Queue-based request review with approval decisions, details, and risk metadata."
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Approval center");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.checkbox(&mut self.priority_only, "Priority only");
                ui.button("New request");
            });
        });
        ui.separator();

        let approvals = [
            (
                "Vendor access request",
                "Security · 2h left · High priority",
                true,
                "Manager sign-off required",
                "API access for finance contractor",
                "Open",
                Color32::from_rgb(220, 110, 90),
            ),
            (
                "Budget increase",
                "Finance · 1d left · Medium",
                false,
                "Reviewed by department lead",
                "Additional spend for analytics tooling",
                "Pending",
                Color32::from_rgb(210, 160, 70),
            ),
            (
                "Data export approval",
                "Compliance · 6h left · High priority",
                true,
                "Policy review in progress",
                "Customer usage export for audit pack",
                "Escalated",
                Color32::from_rgb(155, 100, 220),
            ),
            (
                "Contract renewal",
                "Operations · 3d left · Low",
                false,
                "Ready for legal review",
                "Cloud support agreement renewal",
                "Waiting",
                Color32::from_rgb(90, 170, 120),
            ),
        ];

        let filtered: Vec<_> = approvals
            .iter()
            .enumerate()
            .filter(|(_, item)| !self.priority_only || item.2)
            .collect();

        if filtered.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.heading("No urgent approvals");
                ui.label("Disable the priority filter to view the full queue.");
            });
            return;
        }

        self.selected = self.selected.min(filtered.len().saturating_sub(1));

        egui::Panel::left("approval_queue")
            .default_size(220.0)
            .min_size(180.0)
            .show(ui, |ui| {
                ui.strong("Queue");
                ui.separator();

                for (index, (_, item)) in filtered.iter().enumerate() {
                    let selected = self.selected == index;
                    if ui
                        .selectable_label(selected, item.0)
                        .on_hover_text(item.1)
                        .clicked()
                    {
                        self.selected = index;
                    }
                    ui.weak(item.1);
                    ui.add_space(6.0);
                }
            });

        egui::Panel::right("approval_meta")
            .default_size(220.0)
            .min_size(180.0)
            .show(ui, |ui| {
                let (_, item) = filtered[self.selected];
                ui.strong("Metadata");
                ui.separator();
                ui.label("Request type");
                ui.weak(item.3);
                ui.add_space(8.0);
                ui.label("Risk level");
                ui.colored_label(item.6, match self.selected {
                    0 | 2 => "High",
                    1 => "Medium",
                    _ => "Low",
                });
                ui.add_space(8.0);
                ui.label("Dependencies");
                for label in ["Manager review", "Security check", "Policy sign-off"] {
                    ui.horizontal(|ui| {
                        ui.small("•");
                        ui.weak(label);
                    });
                }
            });

        egui::CentralPanel::default().show(ui, |ui| {
            let (_, item) = filtered[self.selected];

            ui.horizontal(|ui| {
                ui.heading(item.0);
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.colored_label(item.6, item.5);
                });
            });
            ui.separator();

            ui.horizontal(|ui| {
                ui.small("Owner");
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.weak("A. Patel");
                });
            });

            ui.horizontal(|ui| {
                ui.small("Timing");
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.weak(item.1);
                });
            });

            ui.add_space(8.0);
            ui.label("Reason");
            ui.label(RichText::new(item.4).weak());
            ui.add_space(10.0);

            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.label("Decision notes");
                ui.label(RichText::new("Requested by Finance with supporting justification attached. The approval path includes manager review and policy verification.").weak());
            });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Approve").clicked() {
                    self.decision = Some("Approved");
                }
                if ui.button("Reject").clicked() {
                    self.decision = Some("Rejected");
                }
                if ui.button("Request info").clicked() {
                    self.decision = Some("Info requested");
                }
            });

            if let Some(decision) = self.decision {
                ui.add_space(8.0);
                let color = match decision {
                    "Approved" => Color32::from_rgb(80, 170, 120),
                    "Rejected" => Color32::from_rgb(220, 110, 90),
                    _ => Color32::from_rgb(180, 150, 90),
                };
                ui.colored_label(color, format!("Current decision: {decision}"));
            }
        });
    }
}
