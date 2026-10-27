//! Shared UI primitives used across multiple demo patterns.
//!
//! This module centralizes small, repeated presentation helpers so the demo
//! files stay readable and consistent without duplicating the same panel/card
//! patterns in many places.

use egui::{Align, Color32, Frame, Layout, RichText, Ui};

pub fn metric_card(ui: &mut Ui, label: &str, value: &str, delta: &str, color: Color32, compact: bool) {
    Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_width(if compact { 130.0 } else { 160.0 });
        ui.weak(label);
        ui.heading(value);
        ui.colored_label(color, delta);
    });
}

pub fn status_row(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.weak(value);
        });
    });
}

pub fn empty_state(ui: &mut Ui, title: &str, message: &str, action_label: Option<&str>) {
    ui.heading(title);
    ui.label(message);
    if let Some(action) = action_label {
        let _ = ui.button(action);
    }
}

pub fn info_banner(ui: &mut Ui, title: &str, body: &str, tint: Color32) {
    ui.scope(|ui| {
        ui.visuals_mut().widgets.noninteractive.bg_fill = tint;
        Frame::group(ui.style()).show(ui, |ui| {
            ui.colored_label(tint, title);
            ui.add_space(4.0);
            ui.label(RichText::new(body).weak());
        });
    });
}

pub fn toast(ui: &mut Ui, message: &str) {
    egui::Area::new(egui::Id::new("shared_toast"))
        .anchor(egui::Align2::RIGHT_BOTTOM, [-16.0, -16.0])
        .show(ui.ctx(), |ui| {
            Frame::popup(ui.style()).show(ui, |ui| {
                ui.label(message);
            });
        });
}
