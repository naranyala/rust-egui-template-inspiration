//! Pattern: buttons and actions.
//!
//! Sizes, fills, disabled/busy states, menus, and the "action + shortcut hint"
//! pairing that makes a desktop app feel native.

use super::{Category, Demo};
use egui::{RichText, Ui};
use std::time::{Duration, Instant};

#[derive(Default)]
pub struct Buttons {
    /// Simulated background work: while `busy`, actions are disabled and a
    /// spinner is shown instead of the label.
    busy: bool,
    /// When the current fake job started, used to clear `busy`.
    started: Option<Instant>,
    /// Selected item of a mutually exclusive button group.
    selection: usize,
    /// Incremented by the primary action, so the demo shows a visible effect.
    clicks: u32,
    toast: Option<String>,
}

impl Demo for Buttons {
    fn name(&self) -> &'static str {
        "Buttons & actions"
    }

    fn description(&self) -> &'static str {
        "Button sizes and fills, disabled/busy states, menu buttons, and \
         selectable button groups."
    }

    fn category(&self) -> Category {
        Category::Widgets
    }

    fn ui(&mut self, ui: &mut Ui) {
        // --- Sizes and fills -------------------------------------------------
        ui.heading("Sizes & fills");
        ui.horizontal_wrapped(|ui| {
            // Every button returns a `Response`. Here each variant reports
            // into the toast, so the demo has a visible effect.
            let responses = [
                (ui.button("Default"), "Default"),
                (ui.small_button("Small"), "Small"),
                (
                    ui.add_sized([110.0, 24.0], egui::Button::new("Fixed size")),
                    "Fixed size",
                ),
                (
                    ui.add(egui::Button::new("Accent").fill(ui.visuals().selection.bg_fill)),
                    "Accent",
                ),
                (
                    ui.add(egui::Button::new("Danger").fill(ui.visuals().error_fg_color)),
                    "Danger",
                ),
                (
                    ui.add(egui::Button::new(RichText::new("⭐ Starred"))),
                    "Starred",
                ),
            ];
            for (response, label) in responses {
                if response.clicked() {
                    self.toast = Some(format!("{label} button clicked"));
                }
            }
        });

        ui.add_space(10.0);

        // --- Primary action with a busy state -------------------------------
        ui.heading("Busy state");

        // Finish the fake job once enough time has passed. Asking egui to
        // repaint after a delay is how you drive timers in an immediate-mode
        // UI — nothing repaints on its own unless you ask for it.
        if let Some(started) = self.started {
            if started.elapsed() >= Duration::from_millis(1500) {
                self.busy = false;
                self.started = None;
                self.toast = Some("Job finished.".to_owned());
            } else {
                ui.ctx()
                    .request_repaint_after(Duration::from_millis(100));
            }
        }

        ui.horizontal(|ui| {
            // Keyboard shortcut: pair the modifier check with a visible hint
            // in the button label so the shortcut is discoverable.
            let shortcut = ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::Enter));

            // `add_enabled_ui` greys out and blocks everything inside it.
            let run = ui.add_enabled_ui(!self.busy, |ui| {
                if self.busy {
                    ui.spinner();
                    ui.label("Working…")
                } else {
                    let button = ui.add(
                        egui::Button::new(RichText::new("Run job").strong())
                            .fill(ui.visuals().selection.bg_fill),
                    );
                    ui.weak("(⌘⏎)");
                    button
                }
            });

            if (run.inner.clicked() || shortcut) && !self.busy {
                self.busy = true;
                self.started = Some(Instant::now());
                self.clicks += 1;
                self.toast = Some("Job queued.".to_owned());
            }
        });
        ui.label(format!("Jobs run: {}", self.clicks));

        ui.add_space(10.0);

        // --- Selectable group -----------------------------------------------
        ui.heading("Selectable group");
        ui.horizontal(|ui| {
            for (index, label) in ["List", "Grid", "Tree"].into_iter().enumerate() {
                if ui
                    .selectable_label(self.selection == index, label)
                    .clicked()
                {
                    self.selection = index;
                }
            }
        });

        ui.add_space(10.0);

        // --- Menus -----------------------------------------------------------
        ui.heading("Menus");
        ui.horizontal(|ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New").clicked() {
                    self.toast = Some("File → New".to_owned());
                    ui.close();
                }
                if ui.button("Open…").clicked() {
                    self.toast = Some("File → Open".to_owned());
                    ui.close();
                }
                ui.separator();
                // A submenu: just nest another menu button.
                ui.menu_button("Recent", |ui| {
                    for file in ["notes.txt", "Cargo.toml"] {
                        if ui.button(file).clicked() {
                            self.toast = Some(format!("Opened {file}"));
                            ui.close();
                        }
                    }
                });
                ui.separator();
                if ui.button("Quit").clicked() {
                    self.toast = Some("File → Quit".to_owned());
                    ui.close();
                }
            });

            // Action buttons with a keyboard-shortcut hint next to them.
            ui.menu_button("Help", |ui| {
                ui.label(RichText::new("Ctrl+S").monospace().weak());
                ui.label("Save");
                ui.separator();
                ui.hyperlink_to("egui docs", "https://docs.rs/egui");
            });

            ui.hyperlink_to("Open a link", "https://www.rust-lang.org");
        });

        ui.add_space(10.0);

        // --- Toast / status line --------------------------------------------
        if let Some(toast) = self.toast.clone() {
            ui.horizontal(|ui| {
                ui.colored_label(ui.visuals().widgets.active.bg_fill, &toast);
                if ui.small_button("Dismiss").clicked() {
                    self.toast = None;
                }
            });
        } else {
            ui.weak("Actions report here.");
        }

        ui.add_space(10.0);

        // Long-press / hover feedback is free with `on_hover_text` and friends.
        ui.label(
            RichText::new("Hover me for a tooltip, and note the click counter above.")
                .weak()
                .italics(),
        )
        .on_hover_text("Tooltips are chained onto any widget response.");
    }
}
