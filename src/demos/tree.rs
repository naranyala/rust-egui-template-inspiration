//! Pattern: a tree / hierarchy view.
//!
//! `CollapsingHeader` gives you the disclosure triangle and the indentation for
//! free; a recursive function turns your own nested data into a tree. Adding a
//! filter box shows the usual "prune branches that don't match" trick.

use super::{Category, Demo};
use egui::{RichText, Ui};

enum Kind {
    Folder,
    File(&'static str),
}

struct Node {
    name: &'static str,
    kind: Kind,
    children: Vec<Node>,
}

impl Node {
    fn folder(name: &'static str, children: Vec<Node>) -> Self {
        Self {
            name,
            kind: Kind::Folder,
            children,
        }
    }

    fn file(name: &'static str) -> Self {
        Self {
            name,
            kind: Kind::File(icon(name)),
            children: Vec::new(),
        }
    }

    /// A node matches if its own name matches, or any descendant does — so a
    /// folder stays visible when one of its children is a hit.
    fn matches(&self, needle: &str) -> bool {
        if needle.is_empty() || self.name.to_lowercase().contains(needle) {
            return true;
        }
        self.children.iter().any(|child| child.matches(needle))
    }
}

fn icon(name: &str) -> &'static str {
    match name.split('.').next_back().unwrap_or("") {
        "rs" => "🦀",
        "toml" | "lock" => "⚙️",
        "md" | "txt" => "📄",
        "png" | "jpg" | "svg" => "🖼️",
        _ => "•",
    }
}

fn sample_tree() -> Vec<Node> {
    vec![
        Node::folder(
            "src",
            vec![
                Node::file("main.rs"),
                Node::file("app.rs"),
                Node::folder(
                    "demos",
                    vec![
                        Node::file("panels.rs"),
                        Node::file("tables.rs"),
                        Node::file("tree.rs"),
                    ],
                ),
            ],
        ),
        Node::folder(
            "assets",
            vec![Node::file("icon.png"), Node::file("logo.svg")],
        ),
        Node::folder(
            "docs",
            vec![Node::file("architecture.md"), Node::file("changelog.txt")],
        ),
        Node::file("Cargo.toml"),
        Node::file("README.md"),
    ]
}

pub struct Tree {
    nodes: Vec<Node>,
    filter: String,
    /// Path of names from the root to the selected node.
    selected: Option<Vec<String>>,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            nodes: sample_tree(),
            filter: String::new(),
            selected: None,
        }
    }
}

impl Tree {
    fn node_ui(ui: &mut Ui, node: &Node, needle: &str, selected: &mut Option<Vec<String>>) {
        if !node.matches(needle) {
            return;
        }

        match &node.kind {
            Kind::Folder => {
                // `CollapsingHeader` handles the arrow, the indentation and the
                // open/closed state (persisted across frames automatically).
                let response =
                    ui.collapsing(RichText::new(format!("📁 {}", node.name)).strong(), |ui| {
                        for child in &node.children {
                            Self::node_ui(ui, child, needle, selected);
                        }
                    });
                let _ = response;
            }
            Kind::File(icon) => {
                let path = node.name.to_owned();
                let is_selected = selected
                    .as_ref()
                    .is_some_and(|p| p.len() == 1 && p[0] == path);
                if ui
                    .selectable_label(is_selected, format!("{icon} {}", node.name))
                    .clicked()
                {
                    *selected = Some(vec![path]);
                }
            }
        }
    }
}

impl Demo for Tree {
    fn name(&self) -> &'static str {
        "Tree / hierarchy"
    }

    fn description(&self) -> &'static str {
        "A recursive CollapsingHeader tree with a live filter that prunes \
         branches with no matches."
    }

    fn category(&self) -> Category {
        Category::Data
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Filter");
            ui.add(
                egui::TextEdit::singleline(&mut self.filter)
                    .hint_text("e.g. demo")
                    .desired_width(200.0),
            );
            if !self.filter.is_empty() && ui.small_button("Clear").clicked() {
                self.filter.clear();
            }
        });
        ui.add_space(6.0);

        let needle = self.filter.to_lowercase();
        let mut selected = self.selected.take();

        // A frame makes the tree read as a distinct surface.
        egui::Frame::group(ui.style()).show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("tree_scroll")
                .max_height(280.0)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    for node in &self.nodes {
                        Self::node_ui(ui, node, &needle, &mut selected);
                    }
                });
        });

        self.selected = selected;

        ui.add_space(8.0);
        match &self.selected {
            Some(path) => {
                ui.label(RichText::new(format!("Selected: {}", path.join("/"))).weak());
            }
            None => {
                ui.weak("Nothing selected — click a file in the tree.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_filter_keeps_matching_ancestors() {
        let nodes = sample_tree();
        assert!(nodes[0].matches("tables"));
        assert!(!nodes[1].matches("tables"));
        assert!(nodes[3].matches("cargo"));
    }

    #[test]
    fn file_icons_follow_known_extensions() {
        assert_eq!(icon("main.rs"), "🦀");
        assert_eq!(icon("README.md"), "📄");
        assert_eq!(icon("image.png"), "🖼️");
        assert_eq!(icon("unknown.bin"), "•");
    }
}
