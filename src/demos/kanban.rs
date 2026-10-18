//! Pattern: kanban board.
//!
//! Three work columns with real egui drag-and-drop payloads. Cards can be
//! moved between columns, making this a practical starting point for task
//! boards and workflow tools.

use super::{Category, Demo};
use egui::{Align, Id, Layout, RichText, Ui};

struct Card {
    id: usize,
    title: &'static str,
    owner: &'static str,
}

pub struct Kanban {
    show_done: bool,
    selected: Option<usize>,
    next_id: usize,
    columns: Vec<(&'static str, Vec<Card>)>,
}

impl Default for Kanban {
    fn default() -> Self {
        Self {
            show_done: false,
            selected: None,
            next_id: 8,
            columns: vec![
                (
                    "Backlog",
                    vec![
                        Card {
                            id: 0,
                            title: "Research onboarding",
                            owner: "Mira",
                        },
                        Card {
                            id: 1,
                            title: "Audit permissions",
                            owner: "Sam",
                        },
                    ],
                ),
                (
                    "In progress",
                    vec![
                        Card {
                            id: 2,
                            title: "New dashboard",
                            owner: "Ari",
                        },
                        Card {
                            id: 3,
                            title: "Import flow",
                            owner: "Jon",
                        },
                        Card {
                            id: 4,
                            title: "Keyboard shortcuts",
                            owner: "Mira",
                        },
                    ],
                ),
                (
                    "Review",
                    vec![
                        Card {
                            id: 5,
                            title: "Empty states",
                            owner: "Sam",
                        },
                        Card {
                            id: 6,
                            title: "Export report",
                            owner: "Ari",
                        },
                    ],
                ),
                (
                    "Done",
                    vec![Card {
                        id: 7,
                        title: "Release notes",
                        owner: "Jon",
                    }],
                ),
            ],
        }
    }
}

impl Demo for Kanban {
    fn name(&self) -> &'static str {
        "Kanban board"
    }
    fn description(&self) -> &'static str {
        "Drag cards between horizontally scrolling workflow columns."
    }
    fn category(&self) -> Category {
        Category::Layout
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Product roadmap");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.checkbox(&mut self.show_done, "Show completed");
            });
        });
        ui.separator();

        let visible_columns = if self.show_done {
            self.columns.len()
        } else {
            self.columns.len() - 1
        };
        let mut move_to: Option<(usize, usize)> = None;
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.horizontal_top(|ui| {
                for column_index in 0..visible_columns {
                    let title = self.columns[column_index].0;
                    let (inner, dropped) =
                        ui.dnd_drop_zone::<usize, _>(egui::Frame::group(ui.style()), |ui| {
                            ui.set_min_width(190.0);
                            ui.strong(title);
                            ui.separator();
                            for card in &self.columns[column_index].1 {
                                let id = Id::unique(("kanban_card", card.id));
                                let response = ui.dnd_drag_source(id, card.id, |ui| {
                                    Self::render_card(ui, card, self.selected == Some(card.id));
                                });
                                if response.response.clicked() {
                                    self.selected = Some(card.id);
                                }
                                ui.add_space(6.0);
                            }
                            if ui.small_button("+ Add card").clicked() {
                                move_to = Some((self.next_id, column_index));
                            }
                        });
                    if let Some(card_id) = dropped {
                        move_to = Some((*card_id, column_index));
                    }
                    let _ = inner;
                    ui.add_space(8.0);
                }
            });
        });

        if let Some((card_id, target_column)) = move_to {
            if card_id >= self.next_id {
                let id = self.next_id;
                self.next_id += 1;
                self.columns[target_column].1.push(Card {
                    id,
                    title: "New card",
                    owner: "You",
                });
            } else {
                self.move_card(card_id, target_column);
            }
        }

        if let Some(selected) = self.selected
            && let Some(card) = self.card(selected)
        {
            ui.add_space(8.0);
            ui.label(RichText::new(format!("Selected card: {}", card.title)).weak());
        }
    }
}

impl Kanban {
    fn render_card(ui: &mut Ui, card: &Card, selected: bool) {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            ui.set_min_width(170.0);
            let _ = ui.selectable_label(selected, card.title);
            ui.horizontal(|ui| {
                ui.weak(format!("👤 {}", card.owner));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| ui.weak("2d"));
            });
        });
    }

    fn card(&self, id: usize) -> Option<&Card> {
        self.columns
            .iter()
            .flat_map(|(_, cards)| cards)
            .find(|card| card.id == id)
    }

    fn move_card(&mut self, id: usize, target_column: usize) {
        let Some(source_column) = self
            .columns
            .iter()
            .position(|(_, cards)| cards.iter().any(|card| card.id == id))
        else {
            return;
        };
        if source_column == target_column {
            return;
        }
        let position = self.columns[source_column]
            .1
            .iter()
            .position(|card| card.id == id)
            .expect("card was found in source column");
        let card = self.columns[source_column].1.remove(position);
        self.columns[target_column].1.push(card);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moving_a_card_changes_columns() {
        let mut board = Kanban::default();
        assert_eq!(board.columns[0].1.len(), 2);
        board.move_card(0, 1);
        assert_eq!(board.columns[0].1.len(), 1);
        assert!(board.columns[1].1.iter().any(|card| card.id == 0));
    }
}
