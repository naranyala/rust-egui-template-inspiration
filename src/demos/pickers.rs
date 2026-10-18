//! Pattern: date/time and file/folder pickers.
//!
//! egui ships no native file or date dialogs. This demo shows the in-app
//! equivalents: a segmented date/time picker built from `ComboBox` +
//! `DragValue`, and a folder browser that walks the filesystem with
//! `std::fs::read_dir` (a stand-in for `rfd`, which is not in the dep tree).

use super::{Category, Demo};
use egui::{ComboBox, RichText, Ui};

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

#[derive(Default)]
pub struct Pickers {
    day: usize,
    month: usize,
    hour: u32,
    minute: u32,
    /// Path of the directory currently shown by the browser.
    folder: String,
    selected_entry: Option<String>,
}

fn current_dir() -> String {
    std::env::var("PWD")
        .or_else(|_| std::env::current_dir().map(|p| p.to_string_lossy().into_owned()))
        .unwrap_or_else(|_| ".".into())
}

fn dirs_of(path: &str) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(path) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            path.is_dir()
                .then(|| path.file_name().map(|n| n.to_string_lossy().into_owned()))
                .flatten()
        })
        .collect();
    names.sort();
    names
}

impl Demo for Pickers {
    fn name(&self) -> &'static str {
        "Date, time & folder pickers"
    }

    fn description(&self) -> &'static str {
        "Segmented date/time pickers plus a real filesystem folder browser."
    }

    fn category(&self) -> Category {
        Category::Input
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.heading("Date & time");
        let folder_path = if self.folder.is_empty() {
            current_dir()
        } else {
            self.folder.clone()
        };

        ComboBox::from_label("Day")
            .selected_text(format!("{:02}", self.day + 1))
            .show_ui(ui, |ui| {
                for index in 0..31 {
                    ui.selectable_value(&mut self.day, index, format!("{:02}", index + 1));
                }
            });
        ComboBox::from_label("Month")
            .selected_text(MONTHS[self.month])
            .show_ui(ui, |ui| {
                for (index, month) in MONTHS.iter().enumerate() {
                    ui.selectable_value(&mut self.month, index, *month);
                }
            });
        ui.add(
            egui::DragValue::new(&mut self.hour)
                .range(0..=23)
                .suffix(" h"),
        );
        ui.add(
            egui::DragValue::new(&mut self.minute)
                .range(0..=59)
                .suffix(" min"),
        );
        ui.weak(format!(
            "→ {:02} {} at {:02}:{:02}",
            self.day + 1,
            MONTHS[self.month],
            self.hour,
            self.minute
        ));

        ui.add_space(12.0);
        ui.heading("Folder picker");
        ui.label(RichText::new(folder_path.clone()).monospace());
        ui.add_space(4.0);

        if ComboBox::from_label("Directory")
            .selected_text(
                self.selected_entry
                    .as_deref()
                    .unwrap_or("choose a directory…"),
            )
            .show_ui(ui, |ui| {
                if let Some(parent) = parent_of(&folder_path)
                    && ui.button("⬆ Up").clicked()
                {
                    self.folder = parent;
                    self.selected_entry = None;
                    ui.close();
                }
                for name in dirs_of(&folder_path) {
                    if ui
                        .selectable_label(
                            self.selected_entry.as_deref() == Some(name.as_str()),
                            format!("📁 {name}"),
                        )
                        .clicked()
                    {
                        self.selected_entry = Some(name.clone());
                    }
                }
            })
            .response
            .changed()
            && let Some(name) = &self.selected_entry
        {
            self.folder = join(&folder_path, name);
        }

        if !self.folder.is_empty() && ui.button("Reset browser").clicked() {
            self.folder.clear();
            self.selected_entry = None;
        }
    }
}

fn parent_of(path: &str) -> Option<String> {
    std::path::Path::new(path)
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map(|p| p.to_string_lossy().into_owned())
}

fn join(dir: &str, name: &str) -> String {
    format!("{}{}{name}", dir, if dir.ends_with('/') { "" } else { "/" })
}
