//! Pattern: document tabs.
//!
//! A practical tab variant with selection, dirty indicators, and close
//! buttons. Useful for editors, inspectors, and multi-document tools.

use super::{Category, Demo};
use egui::{Align, Layout, Ui};

#[derive(Default)]
pub struct DocumentTabs {
    documents: Vec<&'static str>,
    selected: usize,
    dirty: bool,
}

impl DocumentTabs {
    fn ensure_documents(&mut self) {
        if self.documents.is_empty() {
            self.documents = vec!["welcome.md", "app.rs", "theme.rs"];
            self.selected = 0;
        }
    }
}

impl Demo for DocumentTabs {
    fn name(&self) -> &'static str {
        "Document tabs"
    }
    fn description(&self) -> &'static str {
        "Closable editor tabs with active state, dirty marker, and new-document action."
    }
    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        self.ensure_documents();
        ui.horizontal(|ui| {
            ui.strong("Editor");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui.button("+ New").clicked() {
                    self.documents.push("untitled.rs");
                    self.selected = self.documents.len() - 1;
                    self.dirty = true;
                }
            });
        });
        ui.separator();
        let mut close = None;
        ui.horizontal_wrapped(|ui| {
            for (index, document) in self.documents.iter().enumerate() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        if ui
                            .selectable_label(self.selected == index, *document)
                            .clicked()
                        {
                            self.selected = index;
                        }
                        if self.selected == index && self.dirty {
                            ui.weak("●");
                        }
                        if ui.small_button("×").clicked() {
                            close = Some(index);
                        }
                    });
                });
            }
        });
        if let Some(index) = close {
            self.documents.remove(index);
            self.selected = self.selected.min(self.documents.len().saturating_sub(1));
            self.dirty = false;
        }
        ui.separator();
        let document = self
            .documents
            .get(self.selected)
            .copied()
            .unwrap_or("No document");
        ui.heading(document);
        ui.label("Document content area");
        if ui.button("Mark dirty").clicked() {
            self.dirty = true;
        }
    }
}
