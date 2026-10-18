//! Pattern: a custom-painted canvas.
//!
//! `Ui::allocate_asked_size`/`allocate_exact_size` hands you a `Rect` plus a
//! `Response` to interpret yourself; `ui.painter_at` then lets you draw into
//! it. This is the basis for freehand drawing, graph editors, timelines, etc.

use egui::{Color32, Pos2, Ui};

use super::{Category, Demo};

/// One freehand stroke: the points the pointer travelled, plus the style it
/// was drawn with (so undoing or re-theming doesn't restyle old strokes).
struct Freehand {
    points: Vec<Pos2>,
    color: Color32,
    width: f32,
}

pub struct Canvas {
    strokes: Vec<Freehand>,
    brush: f32,
    color: Color32,
    /// Cached so the "points" counter doesn't recompute every frame.
    point_count: usize,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            strokes: Vec::new(),
            brush: 3.0,
            color: Color32::from_rgb(0x4C, 0x8D, 0xFF),
            point_count: 0,
        }
    }
}

impl Demo for Canvas {
    fn name(&self) -> &'static str {
        "Canvas / drawing"
    }

    fn description(&self) -> &'static str {
        "Freehand drawing on an allocated painter: drag to draw, click to dot, \
         Ctrl+Z to undo."
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Brush");
            ui.add(egui::Slider::new(&mut self.brush, 1.0..=24.0).logarithmic(true));
            ui.label("Color");
            ui.color_edit_button_srgba(&mut self.color);
            if ui.button("Clear").clicked() {
                self.strokes.clear();
                self.point_count = 0;
            }
            ui.separator();
            ui.weak(format!(
                "{} strokes, {} points",
                self.strokes.len(),
                self.point_count
            ));
        });
        ui.add_space(6.0);

        // Undo, before we allocate (so it doesn't fight with the canvas).
        let undo = ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::Z));
        if undo && self.strokes.pop().is_some() {
            self.point_count = self.strokes.iter().map(|s| s.points.len()).sum();
        }

        // Take a chunk of the remaining space for the canvas.
        let size = egui::vec2(ui.available_width().min(800.0), 320.0);
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());

        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 6.0, ui.visuals().extreme_bg_color);
        painter.rect_stroke(
            rect,
            6.0,
            egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color),
            egui::StrokeKind::Inside,
        );

        // Start a new stroke when a drag begins.
        if response.drag_started() {
            self.strokes.push(Freehand {
                points: Vec::new(),
                color: self.color,
                width: self.brush,
            });
        }

        // Append the current pointer position while dragging.
        if response.dragged()
            && let Some(pointer) = response.hover_pos()
            && let Some(stroke) = self.strokes.last_mut()
        {
            stroke.points.push(pointer);
            self.point_count += 1;
        }

        // A plain click (no drag) places a single dot.
        if response.clicked()
            && let Some(pointer) = response.interact_pointer_pos()
        {
            self.strokes.push(Freehand {
                points: vec![pointer],
                color: self.color,
                width: self.brush,
            });
            self.point_count += 1;
        }

        // Draw everything. In a real app you'd spatially partition this.
        for stroke in &self.strokes {
            if stroke.points.is_empty() {
                continue;
            }
            if stroke.points.len() == 1 {
                painter.circle_filled(stroke.points[0], stroke.width * 0.5, stroke.color);
            } else {
                painter.add(egui::Shape::line(
                    stroke.points.clone(),
                    egui::Stroke::new(stroke.width, stroke.color),
                ));
            }
        }

        // Hint text, only while the canvas is empty.
        if self.strokes.is_empty() {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Drag here to draw",
                egui::FontId::proportional(15.0),
                ui.visuals().weak_text_color(),
            );
        }

        // Hovering changes the cursor to signal interactivity.
        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
        }
    }
}
