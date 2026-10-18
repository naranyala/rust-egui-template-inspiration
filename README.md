# rust-egui-template-inspiration

A small desktop application that turns common [`egui`](https://github.com/emilk/egui) layouts and widgets into a searchable, copy-paste-friendly pattern gallery.

The project is deliberately easy to read: the launcher owns navigation and shared state, while each example is an independent `Demo` implementation under `src/demos/`. Use it as a starting point for an egui application, as a reference while learning egui, or as a collection of UI ideas to adapt to your own project.

## What is included

The gallery currently contains **thirty-seven** examples. Every entry below is a real, self-contained implementation in `src/demos/`, linked to its source so you can copy it directly. They are grouped by the launcher's categories.

### Layout

- **Panel layout** — [`src/demos/panels.rs`](src/demos/panels.rs): nested `egui::Panel`s, columns, wrapping, and resize behavior.
- **Canvas / drawing** — [`src/demos/canvas.rs`](src/demos/canvas.rs): `allocate_exact_size`, painter primitives, freehand strokes, and undo.
- **Dashboard** — [`src/demos/dashboard.rs`](src/demos/dashboard.rs): KPI cards, responsive wrapping, and summary panels.
- **Master–detail** — [`src/demos/master_detail.rs`](src/demos/master_detail.rs): searchable list, detail pane, and optional metadata.
- **Wizard / stepper** — [`src/demos/wizard.rs`](src/demos/wizard.rs): guided workflow, validation, and guarded navigation.
- **Grid layout** — [`src/demos/grid.rs`](src/demos/grid.rs): aligned rows/columns and nested grids.
- **Split panes** — [`src/demos/split.rs`](src/demos/split.rs): resizable horizontal and vertical work areas.
- **Tabs** — [`src/demos/tabs.rs`](src/demos/tabs.rs): tab navigation built from selectable labels.
- **Document tabs** — [`src/demos/document_tabs.rs`](src/demos/document_tabs.rs): closable tabs with dirty-state indicators and new-document actions.
- **Floating windows** — [`src/demos/floating.rs`](src/demos/floating.rs): movable overlay windows over a canvas.
- **Three-pane workspace** — [`src/demos/three_pane.rs`](src/demos/three_pane.rs): explorer, editor, and inspector panes.
- **Editor workspace** — [`src/demos/editor_workspace.rs`](src/demos/editor_workspace.rs): file explorer, editor surface, and collapsible terminal.
- **Nav rail + breadcrumbs** — [`src/demos/nav_rail.rs`](src/demos/nav_rail.rs): compact vertical navigation with active breadcrumb context.
- **Responsive layout** — [`src/demos/responsive.rs`](src/demos/responsive.rs): wide card grid that adapts to compact space.
- **Accordion sections** — [`src/demos/accordion.rs`](src/demos/accordion.rs): exclusive collapsible groups and section navigation.
- **Masonry / flow** — [`src/demos/masonry.rs`](src/demos/masonry.rs): variable-height cards balanced across columns.
- **Settings page** — [`src/demos/settings.rs`](src/demos/settings.rs): navigation rail, grouped preferences, and search.
- **Kanban board** — [`src/demos/kanban.rs`](src/demos/kanban.rs): horizontal columns, cards, and compact repeated surfaces.
- **Timeline / activity** — [`src/demos/timeline.rs`](src/demos/timeline.rs): date groups, event markers, and a scrollable activity feed.
- **Command center** — [`src/demos/command_center.rs`](src/demos/command_center.rs): dense operations screen with alerts and live events.
- **Property inspector** — [`src/demos/inspector.rs`](src/demos/inspector.rs): selectable objects with editable or read-only properties.

### Widgets

- **Buttons & actions** — [`src/demos/buttons.rs`](src/demos/buttons.rs): button variants, menus, busy states, selectable groups, shortcuts.
- **Popups & modals** — [`src/demos/popups.rs`](src/demos/popups.rs): tooltips, context menus, menu buttons, blocking dialogs.
- **Command palette** — [`src/demos/command_palette.rs`](src/demos/command_palette.rs): keyboard-first searchable actions in a transient window.
- **Grouped bulk selection** — [`src/demos/bulk_selection.rs`](src/demos/bulk_selection.rs): grouped checkboxes, select-all controls, and bulk actions.

### Input

- **Form & validation** — [`src/demos/forms.rs`](src/demos/forms.rs): `Grid` rows, validation messages, submit/reset state.
- **Input widgets** — [`src/demos/inputs.rs`](src/demos/inputs.rs): sliders, drags, toggles, radio buttons, combo boxes, colors.
- **Keyboard focus & shortcuts** — [`src/demos/keyboard_nav.rs`](src/demos/keyboard_nav.rs): arrow-key focus traversal, activation, and shortcut discoverability.
- **Unsaved changes** — [`src/demos/unsaved_changes.rs`](src/demos/unsaved_changes.rs): dirty tracking with guarded navigation and save/discard confirmation.
- **Date, time & folder pickers** — [`src/demos/pickers.rs`](src/demos/pickers.rs): segmented date/time fields and a filesystem folder browser.

### Data

- **Data table** — [`src/demos/tables.rs`](src/demos/tables.rs): sortable and striped rows, resizable columns, selection, pagination, and virtualisation.
- **Diff / compare** — [`src/demos/diff_view.rs`](src/demos/diff_view.rs): unified and side-by-side source comparison with change highlighting.
- **Tree / hierarchy** — [`src/demos/tree.rs`](src/demos/tree.rs): recursive `CollapsingHeader` nodes and filtered branches.
- **Charts** — [`src/demos/plots.rs`](src/demos/plots.rs): a painter-based line and bar chart with hover feedback.
- **Calendar / scheduler** — [`src/demos/calendar.rs`](src/demos/calendar.rs): month/week planning with selected-day state.

### Feedback

- **Progress & status** — [`src/demos/progress.rs`](src/demos/progress.rs): determinate/indeterminate progress, spinners, repaint timers.
- **Feedback states** — [`src/demos/feedback_states.rs`](src/demos/feedback_states.rs): loading, empty, error, retry, skeleton, success, toast, and notification-center states.

The launcher also provides:

- category-grouped navigation in a resizable sidebar;
- case-insensitive pattern search, with `/` as a search shortcut;
- dark/light themes and a configurable accent color;
- a status bar showing the pattern count and current selection.

The per-file map in [`docs/demo-catalog.md`](docs/demo-catalog.md) mirrors this list and adds a "useful starting point for" column.

## Quick start

Install a recent stable Rust toolchain, then run:

```bash
cargo run
```

For a faster, optimized local build:

```bash
cargo run --release
```

Useful checks before committing changes:

```bash
cargo fmt -- --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
```

The application opens at 1200×800 and supports shrinking to 760×520. On Linux, the selected `eframe` features use X11 or Wayland; the exact native runtime libraries depend on the distribution.

For CI or machines without a compositor, use `cargo check`, `cargo test`, and
the headless render test. Native `cargo run` and PNG capture require a working
desktop session and graphics backend.

## Project structure

```text
.
├── Cargo.toml       dependencies, package metadata, and dev profile
├── TODOS.md         roadmap and completion tracking
├── README.md        project overview and quick start
├── docs/            focused guides for using and extending the project
└── src/
    ├── main.rs       native window setup and eframe startup
    ├── app.rs        launcher shell, navigation, search, and status bar
    ├── theme.rs      dark/light theme and accent-color state
    └── demos/
        ├── mod.rs    Demo trait, categories, and registry
        └── *.rs      one self-contained pattern per module
```

Read [the architecture guide](docs/architecture.md) for the frame layout and state flow, or [the pattern guide](docs/adding-patterns.md) to add an example.

## Documentation map

- [Demo catalog](docs/demo-catalog.md) — every registered pattern and its source file.
- [Architecture](docs/architecture.md) — launcher structure, panel order, and state ownership.
- [Adding patterns](docs/adding-patterns.md) — registration, design guidance, and verification steps.
- [Testing](docs/testing.md) — unit, headless-render, lint, and screenshot checks.
- [egui notes](docs/egui-notes.md) — version-specific API and renderer guidance.
- [TODOS](TODOS.md) — completed work and the remaining roadmap.

## Adding a pattern

1. Create a module in `src/demos/` and implement the `Demo` trait.
2. Declare the module in `src/demos/mod.rs`.
3. Add a default instance to `all_demos()`, bump the catalogue-size assertion
   in the registry test, and add the name to its expected list.
4. Update README.md and docs/demo-catalog.md.
5. Run the formatting and checks above.

The launcher discovers patterns from the registry and automatically displays their name, description, category, and UI. Documentation coverage for every registered pattern is enforced by the headless docs-consistency test. The full walkthrough and a copy-paste starter are in [docs/adding-patterns.md](docs/adding-patterns.md).

## Design notes

- `eframe` uses the `glow` renderer with default features disabled, avoiding the heavier `wgpu` dependency tree for this small reference app.
- The chart example uses `egui::Painter` directly. `egui_plot` is not included because the currently selected dependency versions would introduce an incompatible egui 0.36/0.37 split.
- egui is immediate-mode: state lives in the demo struct, and the UI is rebuilt each frame. Timed examples explicitly request repaints with `request_repaint_after`.

See [docs/egui-notes.md](docs/egui-notes.md) for the version-specific API choices and common pitfalls.

## Testing

The project includes registry invariants, validation/sorting/filtering tests, and a headless render smoke test that exercises every registered demo. Run the full local gate with:

```bash
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt -- --check
```

See [docs/testing.md](docs/testing.md) for coverage details and guidance for testing new patterns.

## Screenshot mode

Generate a full-screen screenshot of a specific template with the built-in capture mode:

```bash
mkdir -p screenshots
EFRAME_SCREENSHOT_TO=screenshots/three-pane.png \
  cargo run --features screenshots -- \
  --demo "Three-pane workspace" --fullscreen
```

Use `cargo run -- --help` to see the available launch flags. The screenshot feature uses eframe's Glow renderer and is intended for visual documentation and README updates.

Screenshot capture is not a headless test: it needs a native compositor and
windowing environment. Keep it as a visual documentation step alongside the
headless checks in [docs/testing.md](docs/testing.md).

## License

`Cargo.toml` declares the intended license expression as `MIT OR Apache-2.0`. Add the corresponding MIT and Apache-2.0 license files when distributing the project.
