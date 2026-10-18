# Architecture

This application is a thin launcher around a registry of independent egui demonstrations. It has no separate model, service, or persistence layer; each demo owns the state needed to make its example interactive.

## Frame layout

`LauncherApp::ui` in `src/app.rs` builds the root UI in this order:

```text
Panel::top       title, theme picker
Panel::left      search field and category navigation
Panel::bottom    pattern count and selection status
CentralPanel     selected demo content
```

Panel order matters. Panels carve space from the parent `Ui`, and `CentralPanel` is shown last so it receives the remaining area. The central content is wrapped in a vertical `ScrollArea`, allowing demos to be taller than the window.

## State ownership

`LauncherApp` owns cross-demo state:

- `demos`: the registered `Box<dyn Demo>` values;
- `selected`: the index of the active pattern;
- `filter`: the sidebar search text;
- `focus_search`: the one-frame handoff used by the `/` shortcut;
- `theme`: the shared theme and accent-color state.

Each demo struct owns only its own interaction state. For example, the table owns sorting and selection, the tree owns its filter and selected path, and the progress demo owns its simulated task timer.

## Demo boundary

The `Demo` trait in `src/demos/mod.rs` is intentionally small:

```rust
pub trait Demo {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn category(&self) -> Category;
    fn ui(&mut self, ui: &mut egui::Ui);
}
```

The launcher does not need to know how a pattern is drawn. It only uses the metadata for navigation and passes a mutable `Ui` to the selected demo.

## Startup flow

`src/main.rs` creates `eframe::NativeOptions`, applies the initial `ThemeState` during the creation callback, constructs `LauncherApp`, and starts `eframe::run_native`. The application name is also used as the egui persistence identifier.

## Why the examples are self-contained

The gallery is intended as a reference, so demos favor local sample data and visible interactions over application infrastructure. A production application would usually move domain data, commands, persistence, and asynchronous work behind separate modules or services.
