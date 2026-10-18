//! Pattern: unsaved-changes confirmation and discard flows.
//!
//! Track a dirty flag against a saved snapshot. Any navigation (switching
//! tabs, closing the document) is guarded: the user must explicitly choose to
//! save, discard, or cancel before the action proceeds.

use super::{Category, Demo};
use egui::Ui;

/// What the user tried to do that triggered the confirm dialog.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    SwitchDocument(usize),
}

#[derive(Default)]
pub struct UnsavedChanges {
    /// The live, potentially unsaved text.
    text: String,
    /// The last checkpoint the user "saved" to.
    saved: String,
    selected: usize,
    /// Navigation guarded by the confirm dialog.
    pending: Option<Action>,
    sessions: u32,
}

impl UnsavedChanges {
    /// The document is dirty whenever it differs from the saved snapshot.
    pub fn is_dirty(&self) -> bool {
        self.text != self.saved
    }

    pub fn checkpoint(&mut self) {
        self.saved = self.text.clone();
    }

    pub fn discard(&mut self) {
        self.text = self.saved.clone();
    }
}

impl Demo for UnsavedChanges {
    fn name(&self) -> &'static str {
        "Unsaved changes"
    }

    fn description(&self) -> &'static str {
        "Dirty tracking with guarded navigation: save, discard, or cancel before leaving."
    }

    fn category(&self) -> Category {
        Category::Input
    }

    fn ui(&mut self, ui: &mut Ui) {
        let documents = ["notes.md", "plan.md", "todo.md"];

        ui.horizontal(|ui| {
            ui.heading("Editor");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if self.is_dirty() {
                    ui.colored_label(ui.visuals().warn_fg_color, "● unsaved changes");
                } else {
                    ui.weak("saved");
                }
            });
        });
        ui.separator();

        // Tab strip: switching away from a dirty document is guarded.
        ui.horizontal(|ui| {
            for (index, document) in documents.iter().enumerate() {
                let suffix = if index == self.selected && self.is_dirty() {
                    " ●"
                } else {
                    ""
                };
                if ui
                    .selectable_label(self.selected == index, format!("{document}{suffix}"))
                    .clicked()
                    && index != self.selected
                {
                    self.pending = Some(Action::SwitchDocument(index));
                }
            }
        });

        ui.add(
            egui::TextEdit::multiline(&mut self.text)
                .hint_text("Type something, then try switching documents…")
                .desired_width(f32::INFINITY),
        );

        ui.horizontal(|ui| {
            if ui
                .add_enabled(self.is_dirty(), egui::Button::new("Save"))
                .clicked()
            {
                self.checkpoint();
                self.sessions += 1;
            }
            if ui
                .add_enabled(self.is_dirty(), egui::Button::new("Discard"))
                .clicked()
            {
                self.discard();
            }
            ui.weak(format!("Saves this session: {}", self.sessions));
        });

        // Confirm dialog, always shown last so it draws on top.
        if let Some(pending) = self.pending {
            let mut resolve: Option<bool> = None; // true = proceed, false = cancel
            egui::Modal::new(egui::Id::unique("unsaved_modal"))
                .backdrop_color(egui::Color32::from_black_alpha(120))
                .show(ui.ctx(), |ui| {
                    ui.set_min_width(320.0);
                    ui.heading("Unsaved changes");
                    ui.label("Your edits will be lost if you leave this document.");
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            resolve = Some(false);
                        }
                        if ui.button("Discard changes").clicked() {
                            resolve = Some(true);
                        }
                        if ui
                            .add(
                                egui::Button::new("Save & continue")
                                    .fill(ui.visuals().selection.bg_fill),
                            )
                            .clicked()
                        {
                            resolve = Some(true);
                        }
                    });
                });

            if let Some(proceed) = resolve {
                if proceed {
                    let Action::SwitchDocument(index) = pending;
                    // Auto-save before switching when the user accepted.
                    self.checkpoint();
                    self.selected = index;
                    self.sessions += 1;
                }
                self.pending = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dirty_tracking_follows_the_snapshot() {
        let mut state = UnsavedChanges {
            text: "hello".into(),
            ..Default::default()
        };
        assert!(state.is_dirty());

        state.checkpoint();
        assert!(!state.is_dirty());

        state.text = "hello world".into();
        assert!(state.is_dirty());

        state.discard();
        assert_eq!(state.text, "hello");
        assert!(!state.is_dirty());
    }
}
