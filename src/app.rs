//! The launcher shell: top bar, searchable sidebar, and the demo host.
//!
//! Layout, outermost to innermost:
//!
//! ```text
//! Panel::top        -> title bar + theme picker
//! Panel::left       -> search box + category-grouped demo list
//! Panel::bottom     -> status bar
//! CentralPanel      -> the selected demo
//! ```
//!
//! In egui 0.37 panels are containers that carve space out of a `Ui`. eframe
//! hands your app the root `Ui` each frame, so panels are added directly to it.
//! Order matters: the first panel added is the outermost, and `CentralPanel`
//! must always come last.

use crate::demos::{Category, Demo};
use crate::theme::ThemeState;
use egui::{RichText, Ui};

pub struct LauncherApp {
    demos: Vec<Box<dyn Demo>>,
    /// Index into `demos` of the currently shown pattern.
    selected: usize,
    /// Sidebar search box contents.
    filter: String,
    /// Set when the `/` shortcut wants to move focus into the search box.
    focus_search: bool,
    theme: ThemeState,
}

impl LauncherApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, theme: ThemeState) -> Self {
        Self {
            demos: crate::demos::all_demos(),
            selected: 0,
            filter: String::new(),
            focus_search: false,
            theme,
        }
    }

    fn top_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("egui pattern launcher");
            ui.weak("(rust-egui-template-inspiration)");
            // Lay the theme picker out against the right edge of the bar.
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if self.theme.ui(ui) {
                    self.theme.apply(ui.ctx());
                }
            });
        });
    }

    fn sidebar(&mut self, ui: &mut Ui) {
        // `/` focuses the search box, unless the user is already typing.
        if ui.input(|i| i.key_pressed(egui::Key::Slash))
            && !ui.ctx().egui_wants_keyboard_input()
        {
            self.focus_search = true;
        }

        let search = ui.add(
            egui::TextEdit::singleline(&mut self.filter)
                .hint_text("Search patterns…  (press /)")
                .desired_width(f32::INFINITY),
        );
        if self.focus_search {
            search.request_focus();
            self.focus_search = false;
        }
        if !self.filter.is_empty() && ui.small_button("Clear").clicked() {
            self.filter.clear();
        }

        ui.separator();

        // ScrollArea both lets long lists scroll and makes the panel resizable
        // (a resizable panel needs its ui to consume the available space).
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let needle = self.filter.to_lowercase();

                for category in Category::ALL {
                    let matches: Vec<usize> = self
                        .demos
                        .iter()
                        .enumerate()
                        .filter(|(_, demo)| {
                            let text_matches =
                                demo.name().to_lowercase().contains(&needle) || needle.is_empty();
                            text_matches && demo.category() == category
                        })
                        .map(|(i, _)| i)
                        .collect();

                    if matches.is_empty() {
                        continue;
                    }

                    ui.add_space(8.0);
                    ui.strong(category.label());
                    ui.separator();

                    for index in matches {
                        let selected = index == self.selected;
                        if ui
                            .selectable_label(selected, self.demos[index].name())
                            .on_hover_text(self.demos[index].description())
                            .clicked()
                        {
                            self.selected = index;
                        }
                    }
                }
            });
    }

    fn demo_view(&mut self, ui: &mut Ui) {
        let demo = &mut self.demos[self.selected];

        ui.heading(demo.name());
        ui.label(RichText::new(demo.description()).weak().italics());
        ui.separator();

        demo.ui(ui);
    }

    fn status_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.weak(format!(
                "{} patterns",
                self.demos.len()
            ));
            ui.separator();
            ui.weak(format!("Selected: {}", self.demos[self.selected].name()));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.weak("Press / to search");
            });
        });
    }
}

impl eframe::App for LauncherApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_bar").exact_size(36.0).show(ui, |ui| {
            self.top_bar(ui);
        });

        egui::Panel::left("sidebar")
            .default_size(240.0)
            .min_size(200.0)
            .show(ui, |ui| {
                self.sidebar(ui);
            });

        egui::Panel::bottom("status_bar")
            .exact_size(24.0)
            .show(ui, |ui| {
                self.status_bar(ui);
            });

        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.demo_view(ui);
            });
        });
    }
}
