//! The pattern catalogue.
//!
//! Every demo is a self-contained struct implementing [`Demo`]. The launcher
//! (`app.rs`) only knows about the trait, so adding a new pattern is just:
//!
//! 1. create `src/demos/my_pattern.rs`,
//! 2. add `pub mod my_pattern;` below,
//! 3. push `Box::new(my_pattern::MyPattern::default())` in [`all_demos`].

pub mod accordion;
pub mod bulk_selection;
pub mod buttons;
pub mod calendar;
pub mod canvas;
pub mod command_center;
pub mod command_palette;
pub mod dashboard;
pub mod diff_view;
pub mod document_tabs;
pub mod editor_workspace;
pub mod feedback_states;
pub mod floating;
pub mod forms;
pub mod grid;
pub mod inputs;
pub mod inspector;
pub mod kanban;
pub mod keyboard_nav;
pub mod masonry;
pub mod master_detail;
pub mod nav_rail;
pub mod panels;
pub mod plots;
pub mod popups;
pub mod progress;
pub mod responsive;
pub mod settings;
pub mod split;
pub mod tables;
pub mod tabs;
pub mod three_pane;
pub mod timeline;
pub mod tree;
pub mod wizard;

/// Grouping used to organise the launcher's sidebar.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Category {
    Layout,
    Widgets,
    Input,
    Data,
    Feedback,
}

impl Category {
    /// Order in which categories appear in the sidebar.
    pub const ALL: [Category; 5] = [
        Category::Layout,
        Category::Widgets,
        Category::Input,
        Category::Data,
        Category::Feedback,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Category::Layout => "Layout",
            Category::Widgets => "Widgets",
            Category::Input => "Input",
            Category::Data => "Data",
            Category::Feedback => "Feedback",
        }
    }
}

/// A single UI pattern shown by the launcher.
pub trait Demo {
    /// Short title shown in the sidebar.
    fn name(&self) -> &'static str;

    /// One-or-two-line blurb shown above the demo.
    fn description(&self) -> &'static str;

    fn category(&self) -> Category;

    /// Draw the pattern. `ui` is the central area of the launcher; demos that
    /// need bounded space (panels, painters) get it from that `ui`.
    fn ui(&mut self, ui: &mut egui::Ui);
}

/// Every demo the launcher knows about, in sidebar order within a category.
pub fn all_demos() -> Vec<Box<dyn Demo>> {
    vec![
        Box::<panels::Panels>::default(),
        Box::<dashboard::Dashboard>::default(),
        Box::<three_pane::ThreePane>::default(),
        Box::<nav_rail::NavRail>::default(),
        Box::<document_tabs::DocumentTabs>::default(),
        Box::<editor_workspace::EditorWorkspace>::default(),
        Box::<responsive::Responsive>::default(),
        Box::<master_detail::MasterDetail>::default(),
        Box::<wizard::Wizard>::default(),
        Box::<settings::Settings>::default(),
        Box::<kanban::Kanban>::default(),
        Box::<timeline::Timeline>::default(),
        Box::<command_center::CommandCenter>::default(),
        Box::<forms::Forms>::default(),
        Box::<grid::Grids>::default(),
        Box::<bulk_selection::BulkSelection>::default(),
        Box::<inputs::Inputs>::default(),
        Box::<keyboard_nav::KeyboardNav>::default(),
        Box::<buttons::Buttons>::default(),
        Box::<tables::Tables>::default(),
        Box::<diff_view::DiffView>::default(),
        Box::<tree::Tree>::default(),
        Box::<popups::Popups>::default(),
        Box::<split::Splits>::default(),
        Box::<tabs::Tabs>::default(),
        Box::<floating::Floating>::default(),
        Box::<accordion::Accordion>::default(),
        Box::<masonry::Masonry>::default(),
        Box::<plots::Plots>::default(),
        Box::<canvas::Canvas>::default(),
        Box::<progress::Progress>::default(),
        Box::<command_palette::CommandPalette>::default(),
        Box::<inspector::Inspector>::default(),
        Box::<feedback_states::FeedbackStates>::default(),
        Box::<calendar::Calendar>::default(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn registry_has_unique_metadata_for_every_demo() {
        let demos = all_demos();
        assert_eq!(demos.len(), 35);

        let names: HashSet<_> = demos.iter().map(|demo| demo.name()).collect();
        assert_eq!(names.len(), demos.len(), "demo names must be unique");
        assert!(
            demos
                .iter()
                .all(|demo| !demo.description().trim().is_empty())
        );
        assert!(
            Category::ALL
                .iter()
                .all(|category| { demos.iter().any(|demo| demo.category() == *category) })
        );

        for expected in [
            "Dashboard",
            "Master–detail",
            "Wizard / stepper",
            "Settings page",
            "Kanban board",
            "Timeline / activity",
            "Command center",
            "Floating windows",
            "Masonry / flow",
            "Accordion sections",
            "Three-pane workspace",
            "Document tabs",
            "Responsive layout",
            "Command palette",
            "Property inspector",
            "Feedback states",
            "Calendar / scheduler",
            "Editor workspace",
            "Nav rail + breadcrumbs",
            "Grouped bulk selection",
            "Diff / compare",
            "Keyboard focus & shortcuts",
        ] {
            assert!(
                names.contains(expected),
                "missing registered demo: {expected}"
            );
        }
    }

    #[test]
    fn every_demo_can_render_one_frame() {
        let ctx = egui::Context::default();
        let mut demos = all_demos();

        for demo in &mut demos {
            let mut output = ctx.run_ui(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| demo.ui(ui));
            });
            output.textures_delta.clear();
        }
    }
}
