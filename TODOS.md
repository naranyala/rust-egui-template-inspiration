# Template gallery roadmap

This file tracks the implementation and quality work for the egui inspiration gallery. Keep it updated whenever a template is added, substantially improved, or deliberately deferred.

## Status legend

- `[ ]` planned
- `[-]` in progress or partially covered
- `[x]` implemented and validated

## Current foundation

- [x] 37 registered demos are exposed through `src/demos/mod.rs`.
- [x] Every demo is listed in `README.md` and `docs/demo-catalog.md`.
- [x] Registry metadata checks enforce unique names and non-empty descriptions.
- [x] Headless smoke test renders every registered demo for one frame.
- [x] Core tests cover form validation, table sorting, tree filtering, and wizard navigation.
- [x] Fullscreen presentation mode supports `--demo` and `--fullscreen`.
- [x] Built-in Glow screenshot mode supports `EFRAME_SCREENSHOT_TO`.

## Next implementation batch

- [x] Add a command palette / quick-open template.
- [x] Add a property inspector template for editable object metadata.
- [x] Add a feedback-state template: loading, empty, error, retry, and toast states.
- [x] Add a calendar/scheduler template with day and week views.
- [x] Add drag-and-drop/reorderable list interactions.
- [x] Add a file explorer + editor + terminal workspace variant.

## Interaction and production patterns

- [x] Add vertical navigation rail and breadcrumb variants.
- [ ] Add tab overflow, tab context menus, and dirty-tab close confirmation.
- [x] Add grouped lists and bulk-selection actions.
- [ ] Add tree-table and filter-builder data views.
- [-] Add pagination and explicit loading/empty/error states to data views (pagination and shared feedback states are covered; table-specific async data is pending).
- [x] Add property inspector read-only/edit modes.
- [x] Add date/time and file/folder picker form variants.
- [x] Add unsaved-changes confirmation and discard flows.
- [x] Add notification center and toast stack.
- [x] Add skeleton loading and offline/retry states.
- [x] Add diff/compare viewer.
- [x] Add keyboard focus-order and shortcut demonstrations.

## Web-parity patterns (framework gap analysis)

Patterns modern web UI frameworks provide natively and our gallery should
demonstrate how to approximate without first-class support:

- [ ] Add a design-token / theme-customization template (colors-as-patterns, accent scoping).
- [ ] Add a flexbox-style grow/shrink manual layout template.
- [ ] Add an animation and easing template (manual lerp + `request_repaint_after`).
- [ ] Add an RTL / i18n text-direction and locale-formatting template.
- [ ] Add an accessibility labeling and focus-ring styling template.

## Quality and automation

- [-] Add interaction tests for click, keyboard, and state transitions (headless state tests are covered; real pointer replay is pending).
- [x] Add test coverage for screenshot CLI argument parsing.
- [ ] Add visual screenshot fixtures for a small representative set of layouts.
- [x] Add a documentation consistency check for registry, README, and catalog entries.
- [ ] Add screenshot examples to README once a desktop compositor is available.
- [ ] Add platform notes for native X11, Wayland, Windows, and macOS behavior.
- [ ] Add proper MIT and Apache-2.0 license files before distribution.

## Completed quality fixes

- [x] Make floating-window IDs stable and preserve floating search/tool state.
- [x] Preserve grid, settings, master-detail, Kanban, and command-center state.
- [x] Correct progress elapsed-time reporting.
- [x] Make accordion sections mutually exclusive.
- [x] Add responsive, three-pane, document-tab, accordion, and masonry variants.
- [x] Keep `cargo test`, Clippy with warnings denied, and rustfmt clean.
- [x] Add paginated data-table controls for large result sets.
- [x] Add a shared launch-option parser with unit coverage.
