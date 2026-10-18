# rust-egui-template-inspiration

A small desktop application that turns common [`egui`](https://github.com/emilk/egui) layouts and widgets into a searchable, copy-paste-friendly pattern gallery.

The project is deliberately easy to read: the launcher owns navigation and shared state, while each example is an independent `Demo` implementation under `src/demos/`. Use it as a starting point for an egui application, as a reference while learning egui, or as a collection of UI ideas to adapt to your own project.

## What is included

The gallery currently contains twenty examples:

| Category | Pattern | Main ideas |
| --- | --- | --- |
| Layout | Panel layout | Nested panels, columns, wrapping, resize behavior |
| Layout | Canvas / drawing | `allocate_exact_size`, painter primitives, freehand strokes, undo |
| Layout | Dashboard | KPI cards, responsive wrapping, and summary panels |
| Layout | Master–detail | Searchable list, detail pane, and optional metadata |
| Layout | Wizard / stepper | Guided workflow, validation, and guarded navigation |
| Layout | Grid layout | Aligned rows/columns and nested grids |
| Layout | Split panes | Resizable horizontal and vertical work areas |
| Layout | Tabs | Tab navigation built from selectable labels |
| Layout | Settings page | Navigation rail, grouped preferences, and search |
| Layout | Kanban board | Horizontal columns, cards, and compact repeated surfaces |
| Layout | Timeline / activity | Date groups, event markers, and a scrollable activity feed |
| Layout | Command center | Dense operations screen with alerts and live events |
| Widgets | Buttons & actions | Button variants, menus, busy states, selectable groups, shortcuts |
| Widgets | Popups & modals | Tooltips, context menus, menu buttons, blocking dialogs |
| Input | Form & validation | `Grid` rows, validation messages, submit/reset state |
| Input | Input widgets | Sliders, drags, toggles, radio buttons, combo boxes, colors |
| Data | Data table | Sortable and striped rows, resizable columns, selection, virtualisation |
| Data | Tree / hierarchy | Recursive `CollapsingHeader` nodes and filtered branches |
| Data | Charts | A painter-based line and bar chart with hover feedback |
| Feedback | Progress & status | Determinate/indeterminate progress, spinners, repaint timers |

The launcher also provides:

- category-grouped navigation in a resizable sidebar;
- case-insensitive pattern search, with `/` as a search shortcut;
- dark/light themes and a configurable accent color;
- a status bar showing the pattern count and current selection.

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

## Project structure

```text
.
├── Cargo.toml       dependencies, package metadata, and dev profile
├── README.md         project overview and quick start
├── docs/             focused guides for using and extending the project
└── src/
    ├── main.rs       native window setup and eframe startup
    ├── app.rs        launcher shell, navigation, search, and status bar
    ├── theme.rs      dark/light theme and accent-color state
    └── demos/
        ├── mod.rs    Demo trait, categories, and registry
        └── *.rs      one self-contained pattern per module
```

Read [the architecture guide](docs/architecture.md) for the frame layout and state flow, or [the pattern guide](docs/adding-patterns.md) to add an example.

## Adding a pattern

1. Create a module in `src/demos/` and implement the `Demo` trait.
2. Declare the module in `src/demos/mod.rs`.
3. Add a default instance to `all_demos()`.
4. Run the formatting and checks above.

The launcher discovers patterns from the registry and automatically displays their name, description, category, and UI. The full walkthrough and a copy-paste starter are in [docs/adding-patterns.md](docs/adding-patterns.md).

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

## License

`Cargo.toml` declares the intended license expression as `MIT OR Apache-2.0`. Add the corresponding MIT and Apache-2.0 license files when distributing the project.
