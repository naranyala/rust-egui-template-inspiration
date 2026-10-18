//! Theme + accent-color handling.
//!
//! egui ships with built-in dark and light visuals. `Context::set_theme`
//! switches between them; `Context::style_mut` layers per-app tweaks (here, an
//! accent color) on top of whichever theme is active.

use egui::{Color32, Context, Theme};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ThemeState {
    pub theme: Theme,
    pub accent: Color32,
}

impl Default for ThemeState {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            accent: Color32::from_rgb(0x4C, 0x8D, 0xFF),
        }
    }
}

impl ThemeState {
    /// Push this state into the egui context. Cheap enough to call whenever it
    /// changes (not every frame).
    pub fn apply(&self, ctx: &Context) {
        ctx.set_theme(self.theme);

        // Style is stored per-theme, so tweaks must target the active one.
        let accent = self.accent;
        ctx.style_mut_of(self.theme, move |style| {
            style.visuals.hyperlink_color = accent;
            style.visuals.selection.bg_fill = accent;
            style.visuals.selection.stroke = egui::Stroke::new(1.0, Color32::WHITE);
            style.visuals.widgets.hovered.bg_stroke.color = accent;
            style.visuals.widgets.active.bg_stroke.color = accent;
        });
    }

    /// A small theme picker: dark/light toggle plus an accent color swatch.
    /// Returns `true` if anything changed (so the caller can re-apply).
    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.horizontal(|ui| {
            ui.strong("Theme");

            let dark = self.theme == Theme::Dark;
            if ui.selectable_label(dark, "Dark").clicked() && !dark {
                self.theme = Theme::Dark;
                changed = true;
            }
            if ui.selectable_label(!dark, "Light").clicked() && dark {
                self.theme = Theme::Light;
                changed = true;
            }

            ui.separator();
            ui.label("Accent");
            if ui.color_edit_button_srgba(&mut self.accent).changed() {
                changed = true;
            }
        });

        changed
    }
}
