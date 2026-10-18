//! Pattern: popups, context menus and modals.
//!
//! Four levels of "floating UI", from least to most intrusive:
//!
//! * tooltips (`on_hover_ui`) — passive, no state,
//! * context menus (`Response::context_menu`) — right-click, auto-dismissing,
//! * menus (`Ui::menu_button`) — the app-menubar pattern,
//! * modals (`egui::Modal`) — blocks the rest of the UI until dismissed.

use super::{Category, Demo};
use egui::{Align2, FontId, RichText, Ui};

#[derive(Default)]
pub struct Popups {
    show_modal: bool,
    deleted: u32,
    log: Vec<String>,
    /// Which menu item was last used, shown as a status line.
    last_action: Option<String>,
}

impl Demo for Popups {
    fn name(&self) -> &'static str {
        "Popups & modals"
    }

    fn description(&self) -> &'static str {
        "Tooltips, right-click context menus, menu buttons, and a blocking \
         modal dialog with a dimmed backdrop."
    }

    fn category(&self) -> Category {
        Category::Widgets
    }

    fn ui(&mut self, ui: &mut Ui) {
        // --- Tooltips --------------------------------------------------------
        ui.heading("Tooltips");
        ui.label("Hover me").on_hover_ui(|ui| {
            ui.label(RichText::new("Rich tooltip").strong());
            ui.weak("Tooltips can contain any widgets.");
        });
        ui.separator();

        // --- Context menu ----------------------------------------------------
        ui.heading("Context menu");
        ui.label("Right-click inside the box below.");

        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(ui.available_width().min(600.0), 110.0),
            egui::Sense::click(),
        );
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 6.0, ui.visuals().faint_bg_color);
        painter.rect_stroke(
            rect,
            6.0,
            egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color),
            egui::StrokeKind::Inside,
        );
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            "Right-click me",
            FontId::proportional(15.0),
            ui.visuals().text_color(),
        );

        // The closure only runs while the menu is open.
        response.context_menu(|ui| {
            if ui.button("Rename…").clicked() {
                self.last_action = Some("Rename".to_owned());
                ui.close();
            }
            if ui.button("Duplicate").clicked() {
                self.last_action = Some("Duplicate".to_owned());
                ui.close();
            }
            ui.separator();
            if ui.button("Delete").clicked() {
                self.show_modal = true;
                ui.close();
            }
        });
        ui.add_space(8.0);

        // --- Menu bar --------------------------------------------------------
        ui.heading("Menus");
        ui.horizontal(|ui| {
            ui.menu_button("File", |ui| {
                for label in ["New", "Open…", "Save"] {
                    if ui.button(label).clicked() {
                        self.last_action = Some(format!("File → {label}"));
                        ui.close();
                    }
                }
            });
            ui.menu_button("Edit", |ui| {
                ui.add_enabled_ui(false, |ui| {
                    // Disabled buttons: shown, but not clickable.
                    let _ = ui.button("Undo");
                    let _ = ui.button("Redo");
                });
                ui.separator();
                if ui.button("Preferences…").clicked() {
                    self.last_action = Some("Edit → Preferences".to_owned());
                    ui.close();
                }
            });
        });
        ui.add_space(8.0);

        // --- Modal -----------------------------------------------------------
        ui.heading("Modal");
        if ui.button("Open modal dialog…").clicked() {
            self.show_modal = true;
        }
        if !self.log.is_empty() {
            ui.add_space(6.0);
            ui.weak("Activity:");
            egui::ScrollArea::vertical()
                .id_salt("popup_log")
                .max_height(90.0)
                .show(ui, |ui| {
                    for line in &self.log {
                        ui.monospace(line);
                    }
                });
        }

        // --- Status ----------------------------------------------------------
        if let Some(action) = &self.last_action {
            ui.add_space(6.0);
            ui.weak(format!("Last action: {action}"));
        }

        // A modal is shown last, on top of everything else in the frame.
        if self.show_modal {
            let mut close = false;

            egui::Modal::new(egui::Id::unique("demo_modal"))
                .backdrop_color(egui::Color32::from_black_alpha(120))
                .show(ui.ctx(), |ui| {
                    ui.set_min_width(300.0);
                    ui.heading("Delete this item?");
                    ui.label("The item will be moved to the trash.");
                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            close = true;
                        }
                        if ui
                            .add(egui::Button::new("Delete").fill(ui.visuals().error_fg_color))
                            .clicked()
                        {
                            self.deleted += 1;
                            self.log.push(format!("Deleted item #{}", self.deleted));
                            close = true;
                        }
                    });
                });

            if close {
                self.show_modal = false;
            }
        }
    }
}
