//! Pattern: charts drawn directly with the painter.
//!
//! `egui_plot` is the quick answer for charts, but it is currently built
//! against egui 0.36 and so cannot be mixed with eframe 0.37. This demo shows
//! the fallback — and arguably the more instructive — approach: take a
//! rectangle, map data coordinates onto it yourself, and paint shapes.
//!
//! The same skeleton covers sparklines, timelines, audio waveforms and graph
//! editors.

use super::{Category, Demo};
use egui::{Color32, Pos2, Rect, Ui, pos2, vec2};
use std::time::Duration;

/// Vertical range of the data, in data units.
const Y_MIN: f64 = -1.4;
const Y_MAX: f64 = 1.4;

pub struct Plots {
    samples: usize,
    phase: f32,
    animate: bool,
    show_bars: bool,
}

impl Default for Plots {
    fn default() -> Self {
        Self {
            samples: 200,
            phase: 0.0,
            animate: true,
            show_bars: true,
        }
    }
}

impl Plots {
    fn wave(x: f64, phase: f32, frequency: f64) -> f64 {
        (x * frequency + phase as f64).sin()
    }

    fn bar(x: f64) -> f64 {
        (x * 0.9).sin() * 0.5 + 0.8
    }
}

impl Demo for Plots {
    fn name(&self) -> &'static str {
        "Charts"
    }

    fn description(&self) -> &'static str {
        "A line + bar chart mapped onto a painter: grid, axis labels, an \
         animated series and a hover crosshair with a value readout."
    }

    fn category(&self) -> Category {
        Category::Data
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.animate, "Animate");
            ui.checkbox(&mut self.show_bars, "Bar chart");
            ui.separator();
            ui.label("Samples");
            ui.add(egui::Slider::new(&mut self.samples, 20..=400));
        });
        ui.add_space(6.0);

        if self.animate {
            // Drive the animation by asking for a repaint — nothing repaints on
            // its own in an immediate-mode UI.
            ui.ctx().request_repaint_after(Duration::from_millis(16));
            self.phase += 0.03;
        }

        // 1. Claim a rectangle, and a `Response` to interpret input against.
        let size = vec2(ui.available_width().min(820.0), 280.0);
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());
        let painter = ui.painter_at(rect);

        let grid_color = ui.visuals().widgets.noninteractive.bg_stroke.color;
        let text_color = ui.visuals().text_color();
        let accent = ui.visuals().selection.bg_fill;

        // 2. Background.
        painter.rect_filled(rect, 6.0, ui.visuals().extreme_bg_color);
        painter.rect_stroke(
            rect,
            6.0,
            egui::Stroke::new(1.0, grid_color),
            egui::StrokeKind::Inside,
        );

        // 3. The plot area, inset so axis labels have room.
        let plot = Rect::from_min_max(rect.min + vec2(46.0, 12.0), rect.max - vec2(12.0, 28.0));
        let x_max = self.samples as f64 * 0.05;

        // 4. Data space -> screen space.
        let to_screen = |x: f64, y: f64| {
            pos2(
                plot.left() + (x / x_max) as f32 * plot.width(),
                plot.top() + ((Y_MAX - y) / (Y_MAX - Y_MIN)) as f32 * plot.height(),
            )
        };

        // 5. Horizontal grid lines + Y labels.
        let font = egui::FontId::monospace(11.0);
        for i in 0..=8 {
            let y = Y_MIN + (Y_MAX - Y_MIN) * i as f64 / 8.0;
            let left = to_screen(0.0, y);
            let right = to_screen(x_max, y);
            painter.line_segment(
                [left, right],
                egui::Stroke::new(1.0, grid_color.gamma_multiply(0.5)),
            );
            painter.text(
                pos2(plot.left() - 8.0, left.y),
                egui::Align2::RIGHT_CENTER,
                format!("{y:.1}"),
                font.clone(),
                text_color,
            );
        }

        // Zero line is emphasised.
        painter.line_segment(
            [to_screen(0.0, 0.0), to_screen(x_max, 0.0)],
            egui::Stroke::new(1.0, grid_color.gamma_multiply(1.8)),
        );

        // 6. X labels.
        for i in 0..=4 {
            let x = x_max * i as f64 / 4.0;
            let bottom = to_screen(x, Y_MIN);
            painter.text(
                pos2(bottom.x, plot.bottom() + 8.0),
                egui::Align2::CENTER_TOP,
                format!("{x:.1}"),
                font.clone(),
                text_color,
            );
        }

        // 7. Bars, drawn from the zero line to each value.
        if self.show_bars {
            let bars = 24;
            let bar_color = Color32::from_gray(140).gamma_multiply(0.45);
            for i in 0..bars {
                let x = (i as f64 + 0.5) * x_max / bars as f64;
                let base = to_screen(x, 0.0);
                let top = to_screen(x, Self::bar(x));
                let width = (plot.width() / bars as f32) * 0.7;
                let bar = Rect::from_center_size(
                    pos2(base.x, (base.y + top.y) * 0.5),
                    vec2(width, (base.y - top.y).abs()),
                );
                painter.rect_filled(bar, 2.0, bar_color);
            }
        }

        // 8. The line series, as one polyline.
        let points: Vec<Pos2> = (0..=self.samples)
            .map(|i| {
                let x = i as f64 * 0.05;
                to_screen(x, Self::wave(x, self.phase, 1.0))
            })
            .collect();
        painter.add(egui::Shape::line(points, egui::Stroke::new(2.0, accent)));

        // 9. Hover: crosshair plus a readout bubble.
        if let Some(pointer) = response.hover_pos()
            && plot.contains(pointer)
        {
            let data_x = ((pointer.x - plot.left()) / plot.width()) as f64 * x_max;
            let data_y = Self::wave(data_x, self.phase, 1.0);
            let snapped = to_screen(data_x, data_y);

            let crosshair = egui::Stroke::new(1.0, accent.gamma_multiply(0.5));
            painter.line_segment(
                [pos2(plot.left(), snapped.y), pos2(plot.right(), snapped.y)],
                crosshair,
            );
            painter.line_segment(
                [pos2(snapped.x, plot.top()), pos2(snapped.x, plot.bottom())],
                crosshair,
            );
            painter.circle_filled(snapped, 3.0, accent);

            // Layout the label first so we know how big to make its backdrop.
            // `fonts_mut` is needed because laying out text caches glyphs.
            let label = format!("x = {data_x:.2}   y = {data_y:+.3}");
            let galley = ui.ctx().fonts_mut(|fonts| {
                fonts.layout_no_wrap(label, egui::FontId::monospace(12.0), text_color)
            });
            let pos = pos2(
                (snapped.x + 10.0).min(plot.right() - galley.size().x - 4.0),
                (snapped.y - galley.size().y - 8.0).max(plot.top() + 4.0),
            );
            let backdrop =
                Rect::from_min_size(pos - vec2(4.0, 2.0), galley.size() + vec2(8.0, 4.0));
            painter.rect_filled(backdrop, 3.0, ui.visuals().panel_fill);
            painter.galley(pos, galley, text_color);
        }
    }
}
