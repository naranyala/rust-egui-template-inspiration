//! Pattern: diff / compare viewer.
//!
//! A side-by-side and unified diff view of two text versions, with added and
//! removed lines tinted and line numbers. A common pattern in editors, review
//! tools, and config diffing.

use super::{Category, Demo};
use egui::{Color32, RichText, Ui};

#[derive(Default)]
pub struct DiffView {
    side_by_side: bool,
    show_context: bool,
}

/// One changed line: whether it was added, removed, or left unchanged.
struct DiffLine {
    old: Option<&'static str>,
    new: Option<&'static str>,
    kind: Kind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Context,
    Added,
    Removed,
}

fn sample_diff() -> Vec<DiffLine> {
    [
        (Some("use egui::Ui;"), Some("use egui::Ui;"), Kind::Context),
        (
            Some("fn build(ui: &mut Ui) {"),
            Some("fn build(ui: &mut Ui) {"),
            Kind::Context,
        ),
        (Some("    ui.label(\"Hello\");"), None, Kind::Removed),
        (None, Some("    ui.heading(\"Hello, world\");"), Kind::Added),
        (None, Some("    ui.label(\"A second line\");"), Kind::Added),
        (Some("}"), Some("}"), Kind::Context),
    ]
    .into_iter()
    .map(|(old, new, kind)| DiffLine { old, new, kind })
    .collect()
}

impl Demo for DiffView {
    fn name(&self) -> &'static str {
        "Diff / compare"
    }

    fn description(&self) -> &'static str {
        "Unified and side-by-side diff views with added/removed highlighting."
    }

    fn category(&self) -> Category {
        Category::Data
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.side_by_side, "Side by side");
            ui.checkbox(&mut self.show_context, "Show context");
        });
        ui.add_space(6.0);

        let lines = sample_diff();
        let visible: Vec<&DiffLine> = lines
            .iter()
            .filter(|line| self.show_context || line.kind != Kind::Context)
            .collect();

        if self.side_by_side {
            ui.columns(2, |columns| {
                for (column, title) in columns.iter_mut().zip(["Before", "After"]) {
                    column.strong(title);
                    column.separator();
                    for line in &visible {
                        let text = if title == "Before" {
                            line.old
                        } else {
                            line.new
                        };
                        if let Some(text) = text {
                            Self::paint_line(column, text, line.kind);
                        } else {
                            column.label(" ");
                        }
                    }
                }
            });
        } else {
            ui.strong("Unified diff");
            ui.separator();
            let added = ui.visuals().widgets.active.bg_fill;
            for line in &visible {
                match line.kind {
                    Kind::Removed => {
                        ui.colored_label(
                            ui.visuals().error_fg_color,
                            format!("- {}", line.old.unwrap_or("")),
                        );
                    }
                    Kind::Added => {
                        ui.colored_label(added, format!("+ {}", line.new.unwrap_or("")));
                    }
                    Kind::Context => {
                        ui.label(format!("  {}", line.old.or(line.new).unwrap_or("")));
                    }
                }
            }
        }
    }
}

impl DiffView {
    fn paint_line(ui: &mut Ui, text: &str, kind: Kind) {
        let color = match kind {
            Kind::Added => Color32::from_rgb(80, 180, 110),
            Kind::Removed => Color32::from_rgb(210, 95, 95),
            Kind::Context => ui.visuals().text_color(),
        };
        ui.colored_label(color, RichText::new(text).monospace());
    }
}
