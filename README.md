# rust-egui-template-inspiration

A small desktop application that turns common [`egui`](https://github.com/emilk/egui) layouts and widgets into a searchable, copy-paste-friendly pattern gallery.

This repository is both a live desktop reference app and a small codebase for building enterprise-style desktop UI layouts in Rust. The launcher owns navigation and shared state, while each example is an independent `Demo` implementation under `src/demos/`.

## What this repo offers

The project currently contains a full catalog of 39 real `egui` patterns, organized by category and exposed through a searchable launcher UI.

Included capabilities:

- searchable sidebar with category grouping
- case-insensitive pattern search (`/` keyboard shortcut)
- dark/light theme switching
- configurable accent color
- presentation mode for focused previewing
- headless smoke-render validation for every demo
- docs for adding new patterns and maintaining the registry

## Codebase at a glance

- `src/main.rs` — application entry point and native app setup
- `src/app.rs` — launcher shell, sidebar, status bar, and selected-demo host
- `src/demos/mod.rs` — registry, metadata, category taxonomy, and shared `Demo` trait
- `src/demos/` — self-contained demo implementations for each UI pattern
- `src/theme.rs` — theme state and accent configuration
- `src/shared.rs` — shared helpers and app-wide values
- `docs/` — architecture, adding-patterns, demo catalog, egui notes, and testing guidance

## Current catalog

The gallery currently contains 39 examples. Every entry below is a real, self-contained implementation in `src/demos/`, linked to its source so you can copy it directly.

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
- **Theme tokens & focus** — [`src/demos/theme_tokens.rs`](src/demos/theme_tokens.rs): tint system, accent groups, and visible keyboard focus states.
- **Accessibility & focus** — [`src/demos/accessibility_focus.rs`](src/demos/accessibility_focus.rs): labels, focus rings, and keyboard-first choices for clear desktop UX.

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
- a status bar showing the pattern count and current selection;
- presentation mode for a minimal preview experience.

The per-file map in [`docs/demo-catalog.md`](docs/demo-catalog.md) mirrors this list and adds a “useful starting point for” column.

## Documentation

- [`docs/architecture.md`](docs/architecture.md) — code structure and design notes
- [`docs/adding-patterns.md`](docs/adding-patterns.md) — how to add a demo and keep registry metadata aligned
- [`docs/demo-catalog.md`](docs/demo-catalog.md) — searchable catalog with use-case mapping
- [`docs/egui-notes.md`](docs/egui-notes.md) — useful egui implementation notes
- [`docs/testing.md`](docs/testing.md) — validation and regression checks

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

## License

`Cargo.toml` declares the intended license expression as `MIT OR Apache-2.0`. Add the corresponding MIT and Apache-2.0 license files when distributing the project.
