//! Pattern: a data table.
//!
//! `egui_extras::TableBuilder` handles the hard parts: column widths,
//! striped rows, a sticky header, and — importantly — only laying out the rows
//! that are actually visible, so it stays fast with thousands of entries.

use super::{Category, Demo};
use egui::{RichText, Ui};
use egui_extras::{Column, TableBuilder};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Folder,
    Code,
    Doc,
    Binary,
}

impl Kind {
    fn label(self) -> &'static str {
        match self {
            Kind::Folder => "Folder",
            Kind::Code => "Code",
            Kind::Doc => "Doc",
            Kind::Binary => "Binary",
        }
    }

    fn icon(self) -> &'static str {
        match self {
            Kind::Folder => "📁",
            Kind::Code => "🦀",
            Kind::Doc => "📄",
            Kind::Binary => "⚙️",
        }
    }
}

struct Row {
    name: String,
    kind: Kind,
    size_kb: f64,
    /// Days since last modification — a cheap stand-in for a timestamp.
    age_days: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SortKey {
    Name,
    Kind,
    Size,
}

pub struct Tables {
    rows: Vec<Row>,
    sort: SortKey,
    ascending: bool,
    selected: Option<usize>,
    striped: bool,
    resizable: bool,
    page: usize,
    page_size: usize,
}

impl Default for Tables {
    fn default() -> Self {
        Self {
            rows: sample_rows(),
            sort: SortKey::Name,
            ascending: true,
            selected: None,
            striped: true,
            resizable: true,
            page: 0,
            page_size: 5,
        }
    }
}

impl Tables {
    /// Row indices in the order the current sort produces. Computing this once
    /// per frame (instead of sorting inside the closure) keeps the borrow
    /// checker happy and the code obvious.
    fn order(&self) -> Vec<usize> {
        let mut order: Vec<usize> = (0..self.rows.len()).collect();
        order.sort_by(|&a, &b| {
            let (left, right) = (&self.rows[a], &self.rows[b]);
            let cmp = match self.sort {
                SortKey::Name => left.name.cmp(&right.name),
                SortKey::Kind => left.kind.label().cmp(right.kind.label()),
                SortKey::Size => left
                    .size_kb
                    .partial_cmp(&right.size_kb)
                    .unwrap_or(std::cmp::Ordering::Equal),
            };
            if self.ascending { cmp } else { cmp.reverse() }
        });
        order
    }

    /// Header cell that flips the sort when clicked.
    fn header_cell(
        ui: &mut Ui,
        title: &str,
        key: SortKey,
        sort: SortKey,
        ascending: bool,
        on_click: &mut Option<SortKey>,
    ) {
        let arrow = if sort == key {
            if ascending { " ▲" } else { " ▼" }
        } else {
            ""
        };
        let label = format!("{title}{arrow}");
        if ui
            .selectable_label(sort == key, RichText::new(label).strong())
            .clicked()
        {
            *on_click = Some(key);
        }
    }
}

impl Demo for Tables {
    fn name(&self) -> &'static str {
        "Data table"
    }

    fn description(&self) -> &'static str {
        "Sortable, striped, resizable columns with row selection — virtualised \
         so it scales to thousands of rows."
    }

    fn category(&self) -> Category {
        Category::Data
    }

    fn ui(&mut self, ui: &mut Ui) {
        let page_count = self.rows.len().div_ceil(self.page_size);
        self.page = self.page.min(page_count.saturating_sub(1));

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.striped, "Striped");
            ui.checkbox(&mut self.resizable, "Resizable columns");
            ui.separator();
            ui.weak(format!("{} rows", self.rows.len()));
            ui.separator();
            ui.label(format!("Page {}/{}", self.page + 1, page_count));
            if ui.button("Previous").clicked() {
                self.page = self.page.saturating_sub(1);
            }
            if ui.button("Next").clicked() {
                self.page = (self.page + 1).min(page_count.saturating_sub(1));
            }
        });
        ui.add_space(6.0);

        // The sort state is written by the header closure and applied after
        // the table is done, avoiding a mutable borrow of `self` inside it.
        let mut new_sort: Option<SortKey> = None;
        let mut new_selected: Option<usize> = None;
        let order = self
            .order()
            .into_iter()
            .skip(self.page * self.page_size)
            .take(self.page_size)
            .collect::<Vec<_>>();

        TableBuilder::new(ui)
            .striped(self.striped)
            .resizable(self.resizable)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto().at_least(160.0))
            .column(Column::auto().at_least(90.0))
            .column(Column::initial(110.0).at_least(70.0).resizable(true))
            .column(Column::remainder().at_least(90.0))
            .header(24.0, |mut header| {
                let (sort, ascending) = (self.sort, self.ascending);
                header.col(|ui| {
                    Self::header_cell(ui, "Name", SortKey::Name, sort, ascending, &mut new_sort);
                });
                header.col(|ui| {
                    Self::header_cell(ui, "Kind", SortKey::Kind, sort, ascending, &mut new_sort);
                });
                header.col(|ui| {
                    Self::header_cell(ui, "Size", SortKey::Size, sort, ascending, &mut new_sort);
                });
                header.col(|ui| {
                    ui.strong("Modified");
                });
            })
            .body(|body| {
                // `rows` only lays out what fits in the viewport.
                body.rows(22.0, order.len(), |mut row| {
                    let index = order[row.index()];
                    let item = &self.rows[index];

                    row.set_selected(self.selected == Some(index));

                    row.col(|ui| {
                        ui.label(format!("{}  {}", item.kind.icon(), item.name));
                    });
                    row.col(|ui| {
                        ui.label(item.kind.label());
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:.1} KB", item.size_kb));
                    });
                    row.col(|ui| {
                        ui.weak(format!("{}d ago", item.age_days));
                    });

                    if row.response().clicked() {
                        new_selected = Some(index);
                    }
                });
            });

        // Apply deferred state changes.
        if let Some(key) = new_sort {
            if self.sort == key {
                self.ascending = !self.ascending;
            } else {
                self.sort = key;
                self.ascending = true;
            }
        }
        self.selected = new_selected;

        if let Some(index) = self.selected {
            let item = &self.rows[index];
            ui.add_space(8.0);
            ui.label(
                RichText::new(format!(
                    "Selected: {} ({}, {:.1} KB)",
                    item.name,
                    item.kind.label(),
                    item.size_kb
                ))
                .weak(),
            );
        }
    }
}

fn sample_rows() -> Vec<Row> {
    let entries: [(&str, Kind, f64, u32); 14] = [
        ("src", Kind::Folder, 4.0, 0),
        ("app.rs", Kind::Code, 12.4, 0),
        ("main.rs", Kind::Code, 1.2, 0),
        ("demos", Kind::Folder, 4.0, 1),
        ("panels.rs", Kind::Code, 6.8, 1),
        ("tables.rs", Kind::Code, 7.1, 1),
        ("Cargo.toml", Kind::Doc, 0.6, 2),
        ("Cargo.lock", Kind::Binary, 412.0, 2),
        ("README.md", Kind::Doc, 3.3, 3),
        ("assets", Kind::Folder, 4.0, 6),
        ("icon.png", Kind::Binary, 24.8, 6),
        ("logo.svg", Kind::Doc, 8.9, 6),
        ("LICENSE-MIT", Kind::Doc, 1.1, 30),
        ("target", Kind::Folder, 4.0, 0),
    ];

    entries
        .into_iter()
        .map(|(name, kind, size_kb, age_days)| Row {
            name: name.to_owned(),
            kind,
            size_kb,
            age_days,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_order_respects_ascending_and_descending_sort() {
        let mut table = Tables {
            sort: SortKey::Size,
            ascending: true,
            ..Default::default()
        };
        let ascending = table.order();
        assert_eq!(table.rows[ascending[0]].name, "Cargo.toml");

        table.ascending = false;
        let descending = table.order();
        assert_eq!(table.rows[descending[0]].name, "Cargo.lock");
    }
}
