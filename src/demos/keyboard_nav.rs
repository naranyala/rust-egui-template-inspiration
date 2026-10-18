//! Pattern: keyboard focus order and shortcuts.
//!
//! Demonstrates two related ideas:
//!
//! * moving *focus* between widgets with the arrow keys (so keyboard users can
//!   operate a list without a mouse), and
//! * a discoverable shortcut reference table.
//!
//! `Ui::push_id` gives each row a stable [`egui::Id`], and
//! `Memory::request_focus` moves focus programmatically.

use super::{Category, Demo};
use egui::{RichText, Ui};

#[derive(Default)]
pub struct KeyboardNav {
    focused: usize,
    selected: Option<usize>,
    activations: u32,
}

const ITEMS: [&str; 5] = ["Alpha", "Bravo", "Charlie", "Delta", "Echo"];

impl Demo for KeyboardNav {
    fn name(&self) -> &'static str {
        "Keyboard focus & shortcuts"
    }

    fn description(&self) -> &'static str {
        "Arrow-key focus traversal, Enter to activate, and a shortcut reference table."
    }

    fn category(&self) -> Category {
        Category::Input
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.heading("Keyboard focus");
        ui.label("Use ↑/↓ to move focus and Enter to activate.");
        ui.separator();

        let parent = ui.scope_id();

        // Handle traversal before drawing, so focus lands on this frame's widget.
        let down = ui.input(|i| i.key_pressed(egui::Key::ArrowDown));
        let up = ui.input(|i| i.key_pressed(egui::Key::ArrowUp));
        let enter = ui.input(|i| i.key_pressed(egui::Key::Enter));

        if down {
            self.focused = (self.focused + 1) % ITEMS.len();
        }
        if up {
            self.focused = (self.focused + ITEMS.len() - 1) % ITEMS.len();
        }

        for (index, item) in ITEMS.iter().enumerate() {
            let child_id = parent.with(index);
            let is_focused = self.focused == index;

            ui.push_id(index, |ui| {
                let response = ui.selectable_label(self.selected == Some(index), *item);
                if response.clicked() {
                    self.selected = Some(index);
                    self.activations += 1;
                }
                // Reflect the programmatic focus state back onto the widget.
                if is_focused {
                    response.request_focus();
                }
            });

            // Ask egui to move focus; this also updates the memory used above.
            if is_focused {
                ui.ctx().memory_mut(|memory| memory.request_focus(child_id));
            }
        }

        if enter {
            self.selected = Some(self.focused);
            self.activations += 1;
        }

        ui.add_space(8.0);
        ui.weak(format!(
            "Focused: {} · Selected: {} · Activations: {}",
            ITEMS[self.focused],
            self.selected.map(|i| ITEMS[i]).unwrap_or("none"),
            self.activations
        ));

        ui.add_space(12.0);
        ui.heading("Shortcuts");
        egui::Grid::new("shortcut_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                for (keys, action) in [
                    ("↑ / ↓", "Move focus between items"),
                    ("Enter", "Activate the focused item"),
                    (
                        "Tab / Shift+Tab",
                        "Move to the next/previous focusable widget",
                    ),
                    ("Escape", "Dismiss the current dialog or menu"),
                ] {
                    ui.label(RichText::new(keys).monospace().strong());
                    ui.label(action);
                    ui.end_row();
                }
            });
    }
}
