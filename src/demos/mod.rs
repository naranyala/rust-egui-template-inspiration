//! The pattern catalogue.
//!
//! Every demo is a self-contained struct implementing [`Demo`]. The launcher
//! (`app.rs`) only knows about the trait, so adding a new pattern is just:
//!
//! 1. create `src/demos/my_pattern.rs`,
//! 2. add `pub mod my_pattern;` below,
//! 3. push `Box::new(my_pattern::MyPattern::default())` in [`all_demos`].

pub mod buttons;
pub mod canvas;
pub mod forms;
pub mod grid;
pub mod inputs;
pub mod panels;
pub mod plots;
pub mod popups;
pub mod progress;
pub mod tables;
pub mod tree;

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
        Box::<forms::Forms>::default(),
        Box::<grid::Grids>::default(),
        Box::<inputs::Inputs>::default(),
        Box::<buttons::Buttons>::default(),
        Box::<tables::Tables>::default(),
        Box::<tree::Tree>::default(),
        Box::<popups::Popups>::default(),
        Box::<plots::Plots>::default(),
        Box::<canvas::Canvas>::default(),
        Box::<progress::Progress>::default(),
    ]
}
