//! Pattern: masonry / flow layout.
//!
//! Unlike a fixed grid or wrapped row, masonry packs variable-height cards into
//! columns so each column stays roughly even. egui has no masonry widget, so
//! this demo shows the classic trick: estimate each card's height, then place
//! it in the currently shortest column.

use super::{Category, Demo};
use egui::{RichText, Ui};

#[derive(Default)]
pub struct Masonry {
    columns: usize,
    dense: bool,
}

/// One card's content plus a rough height estimate (in points).
fn cards() -> [(&'static str, &'static str, f32); 8] {
    [
        (
            "Design tokens",
            "A shared vocabulary for color, spacing and type.",
            78.0,
        ),
        (
            "Accessibility",
            "Contrast, focus order, keyboard support, screen readers.",
            104.0,
        ),
        (
            "Performance",
            "Budgets for frame time and repaint cost.",
            62.0,
        ),
        (
            "Theming",
            "Runtime theme switching without restarting the app.",
            70.0,
        ),
        (
            "Responsive",
            "Breakpoints that adapt the layout to the window.",
            88.0,
        ),
        (
            "Animation",
            "Short, purposeful transitions that respect motion preferences.",
            110.0,
        ),
        (
            "Localization",
            "Text direction, formatting and plural rules.",
            74.0,
        ),
        (
            "Testing",
            "Rendering one frame per widget in a headless context.",
            96.0,
        ),
    ]
}

impl Demo for Masonry {
    fn name(&self) -> &'static str {
        "Masonry / flow"
    }

    fn description(&self) -> &'static str {
        "Variable-height cards packed into balanced columns, simulated without \
         a dedicated widget."
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add(egui::Slider::new(&mut self.columns, 2..=5).text("columns"));
            ui.checkbox(&mut self.dense, "Compact spacing");
        });
        ui.add_space(4.0);

        let gap = if self.dense { 4.0 } else { 10.0 };
        let columns = self.columns.max(1);

        // Estimate how tall each card is and keep a running total per column.
        let mut heights = vec![0.0f32; columns];
        let mut assignment = vec![0usize; cards().len()];
        for (i, card) in cards().iter().enumerate() {
            let shortest = heights
                .iter()
                .enumerate()
                .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|(idx, _)| idx)
                .unwrap();
            assignment[i] = shortest;
            heights[shortest] += card.2 + gap;
        }

        // Draw the columns; each card remembers which column it belongs to.
        ui.horizontal_top(|ui| {
            for column in 0..columns {
                ui.vertical(|ui| {
                    ui.set_min_width(ui.available_width() / columns as f32);
                    for (i, (title, body, height)) in cards().iter().enumerate() {
                        if assignment[i] != column {
                            continue;
                        }
                        egui::Frame::group(ui.style()).show(ui, |ui| {
                            ui.set_min_height(*height);
                            ui.strong(*title);
                            ui.add_space(4.0);
                            ui.label(RichText::new(*body).weak());
                        });
                        ui.add_space(gap);
                    }
                });
                ui.add_space(gap);
            }
        });

        ui.add_space(4.0);
        ui.weak("Tip: for true masonry use `egui::Grid` per column plus a scrollbar.");
    }
}
