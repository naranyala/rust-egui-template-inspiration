//! Pattern: progress and long-running work.
//!
//! `ProgressBar` (determinate and animated) plus `Spinner`, and the repaint
//! dance needed to drive a simulated background task from an immediate-mode UI.

use super::{Category, Demo};
use egui::{RichText, Ui};
use std::time::{Duration, Instant};

pub struct Progress {
    /// 0.0 … 1.0 progress of the running task.
    value: f32,
    running: bool,
    /// When the current task started, and when it last advanced.
    started: Option<Instant>,
    last_tick: Option<Instant>,
    /// Fake work log shown under the progress bars.
    log: Vec<String>,
    /// Indeterminate (barber-pole) mode.
    indeterminate: bool,
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            value: 0.0,
            running: false,
            started: None,
            last_tick: None,
            log: Vec::new(),
            indeterminate: false,
        }
    }
}

impl Demo for Progress {
    fn name(&self) -> &'static str {
        "Progress & status"
    }

    fn description(&self) -> &'static str {
        "Determinate and indeterminate progress bars, spinners, and a status \
         log driven by request_repaint_after."
    }

    fn category(&self) -> Category {
        Category::Feedback
    }

    fn ui(&mut self, ui: &mut Ui) {
        // --- Advance the simulated task -------------------------------------
        if self.running {
            let now = Instant::now();
            let delta = self
                .last_tick
                .map(|last| now.duration_since(last).as_secs_f32())
                .unwrap_or(0.0);
            self.last_tick = Some(now);

            // ~4 seconds per "job".
            self.value = (self.value + delta / 4.0).min(1.0);

            if self.value >= 1.0 {
                self.running = false;
                self.started = None;
                self.last_tick = None;
                self.log.push(format!(
                    "finished job #{} in {:.1}s",
                    self.log.len() + 1,
                    now.duration_since(self.started.unwrap_or(now)).as_secs_f32()
                ));
            } else {
                // Nothing repaints on its own: ask egui to come back shortly.
                ui.ctx()
                    .request_repaint_after(Duration::from_millis(50));
            }
        }

        // --- Controls --------------------------------------------------------
        ui.horizontal(|ui| {
            let label = if self.running { "Cancel" } else { "Start job" };
            if ui.button(label).clicked() {
                if self.running {
                    self.running = false;
                    self.started = None;
                    self.last_tick = None;
                    self.log.push("cancelled".to_owned());
                } else {
                    self.running = true;
                    self.value = 0.0;
                    self.started = Some(Instant::now());
                    self.last_tick = Some(Instant::now());
                }
            }
            ui.checkbox(&mut self.indeterminate, "Indeterminate");
            if ui.small_button("Clear log").clicked() {
                self.log.clear();
            }
        });

        ui.add_space(10.0);

        // --- Determinate progress -------------------------------------------
        ui.strong("Overall");
        let text = if self.indeterminate {
            "working…".to_owned()
        } else {
            format!("{:.0}%", self.value * 100.0)
        };
        ui.add(
            egui::ProgressBar::new(self.value)
                .animate(true)
                .desired_width(ui.available_width())
                .text(text),
        );

        ui.add_space(8.0);

        // --- A few sub-tasks side by side ------------------------------------
        ui.strong("Steps");
        let steps = [("Downloading", 0.9), ("Compiling", 0.55), ("Linking", 0.1)];
        for (name, fraction) in steps {
            ui.horizontal(|ui| {
                ui.add_sized([110.0, 16.0], egui::ProgressBar::new(fraction).animate(true));
                ui.weak(format!("{} {:.0}%", name, fraction * 100.0));
            });
        }

        ui.add_space(8.0);

        // --- Spinner ---------------------------------------------------------
        ui.horizontal(|ui| {
            ui.spinner();
            ui.weak("Spinners are for work of unknown length.");
        });

        ui.add_space(10.0);

        // --- Log -------------------------------------------------------------
        if self.log.is_empty() {
            ui.weak("No jobs yet.");
        } else {
            ui.strong("Log");
            egui::Frame::group(ui.style()).show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("progress_log")
                    .max_height(140.0)
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        for line in &self.log {
                            ui.monospace(line);
                        }
                    });
            });
        }

        if let Some(started) = self.started {
            ui.add_space(6.0);
            ui.label(
                RichText::new(format!("Elapsed: {:.1}s", started.elapsed().as_secs_f32()))
                    .weak(),
            );
        }
    }
}
