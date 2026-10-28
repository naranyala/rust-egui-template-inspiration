# Template gallery roadmap

This file tracks the implementation and quality work for the egui inspiration gallery. Keep it updated whenever a template is added, substantially improved, or deliberately deferred.

## Status legend

- `[ ]` planned
- `[-]` in progress or partially covered
- `[x]` implemented and validated

## Current foundation

- [x] 40 registered demos are exposed through `src/demos/mod.rs`.
- [x] Every demo is listed in `README.md` and `docs/demo-catalog.md`.
- [x] Registry metadata checks enforce unique names and non-empty descriptions.
- [x] Headless smoke test renders every registered demo for one frame.
- [x] Core tests cover form validation, table sorting, tree filtering, and wizard navigation.
- [x] Fullscreen presentation mode supports `--demo` and `--fullscreen`.
- [x] Built-in Glow screenshot mode supports `EFRAME_SCREENSHOT_TO`.

## Core product goals

The project should become a comprehensive egui pattern gallery that covers:

- desktop UI foundations and app shells
- common layout primitives from basic to advanced
- realistic workflow patterns used in production apps
- feedback and state handling
- accessibility and visual polish
- copy-paste-friendly examples with clear metadata and searchability

## High-priority roadmap

### Phase 1: Foundation and catalog experience

- [x] Finalize the gallery metadata model: title, category, difficulty, widgets, description, tags, use case
- [ ] Add a consistent pattern card layout for browsing examples
- [ ] Add search-by-keyword and tag filtering
- [ ] Add category and difficulty filters
- [ ] Add copy-to-clipboard for example code
- [ ] Add a favorites / starred examples list
- [ ] Add a recent examples view
- [ ] Add theme-aware preview rendering for dark and light modes
- [ ] Add a simple full-screen demo preview mode for selected examples
- [ ] Add a docs page describing how to add a pattern and maintain metadata

### Phase 2: Core layout patterns

- [ ] Centered single-column form layout
- [ ] Top bar + content panel shell
- [ ] Sidebar navigation + main content area
- [ ] Three-panel layout (nav, list, detail)
- [ ] Split-pane layout with resizable areas
- [ ] Scrollable content sections and long-form panels
- [ ] Sticky header and sticky footer patterns
- [ ] Grid of cards and tiles
- [ ] Sectioned layouts with headings and grouping
- [ ] Dense and spacious layout variants
- [ ] Responsive narrow-width adaptions

### Phase 3: Form and input patterns

- [ ] Basic form with labels and validation
- [ ] Inline validation states
- [ ] Two-column settings form
- [ ] Wizard / multi-step form
- [ ] Search + filter form with chips and facets
- [ ] Input groups with label, helper text, and action button
- [ ] Conditional fields and advanced settings disclosure
- [ ] Toggle and description groups
- [ ] Compact form density variant
- [ ] Data entry patterns for profile, contact, and settings flows
- [ ] Multi-field composition with grouped sections

### Phase 4: Widget and composition patterns

- [ ] ComboBox + RadioButton + Checkbox combinations
- [ ] Slider and DragValue settings blocks
- [ ] Segmented controls and buttons groups
- [ ] Tabs and sub-navigation examples
- [ ] Selection cards and option pickers
- [ ] Progress bars and progress states
- [ ] Status badges and pills
- [ ] Panels with action buttons and utility controls
- [ ] Empty states with clear next actions
- [ ] Inline help, labels, and hint-text patterns

### Phase 5: Data presentation patterns

- [ ] Searchable list + selection view
- [ ] Detail pane beside a list
- [ ] Simple table view
- [ ] Sortable table with controls
- [ ] Filtered table patterns
- [ ] Data cards / metrics cards
- [ ] Log viewer with status colors
- [ ] Timeline or event stream layout
- [ ] Tree view and expandable file-tree examples
- [ ] Filter-builder and search-builder patterns
- [ ] Pagination and loading/empty/error states in data views

### Phase 6: Navigation and desktop app shells

- [ ] Dashboard shell
- [ ] Workspace shell
- [ ] Tabbed document layout
- [ ] Folder / project navigation layout
- [ ] Breadcrumb navigation
- [ ] Drawer / side panel overlay
- [ ] Multi-level nested navigation
- [ ] Contextual action menus
- [ ] Command palette pattern
- [ ] Quick actions toolbar
- [ ] Vertical navigation rail + content pane variant

### Phase 7: Feedback and state patterns

- [ ] Loading skeleton and spinner states
- [ ] Error panels and retry flows
- [ ] Success and confirmation banners
- [ ] Warning and caution patterns
- [ ] Toast / transient notification stack
- [ ] Empty state screens
- [ ] Disabled/unavailable states
- [ ] Busy states inside sections
- [ ] Save/apply handling patterns
- [ ] Unsaved changes confirmation flows
- [ ] Notification center examples

### Phase 8: Advanced desktop UI patterns

- [ ] Resizable split-pane workspaces
- [ ] Docking-style workspace mockup
- [ ] Multi-column editor layout
- [ ] Kanban board layout
- [ ] Data explorer / inspector panel
- [ ] Inbox-like layout with list + preview
- [ ] CRM / record-detail page layout
- [ ] IDE-like file tree + editor + preview
- [ ] Complex nested settings workspace
- [ ] File explorer + editor + terminal workspace variant
- [ ] Property-inspector and metadata editing patterns

### Phase 9: Visual design and polish

- [ ] Rounded cards and panels
- [ ] Depth and shadow patterns
- [ ] Accent-color callouts and highlight states
- [ ] Compact UI density variants
- [ ] Spacious UI density variants
- [ ] Visual groupings with separators and spacing rhythm
- [ ] Status color conventions
- [ ] Typography emphasis examples
- [ ] Theme-aware surfaces and controls
- [ ] Polished spacing and alignment reference examples

### Phase 10: Accessibility and UX quality

- [ ] Focus-visible styling for interactive elements
- [ ] Keyboard-friendly layouts and navigation
- [ ] Contrast examples and readable states
- [ ] Clear labeling and helper-text examples
- [ ] Large hit target patterns
- [ ] Error messaging that is readable and actionable
- [ ] Reduced-motion / calmer interaction patterns
- [ ] Patterns usable in both dense and roomy layouts
- [ ] Accessible modal and dialog patterns
- [ ] RTL / i18n and locale-formatting example
- [ ] Design-token / theme customization template

## Next implementation batch

- [x] Add a command palette / quick-open template.
- [x] Add a property inspector template for editable object metadata.
- [x] Add a feedback-state template: loading, empty, error, retry, and toast states.
- [x] Add a calendar/scheduler template with day and week views.
- [x] Add drag-and-drop/reorderable list interactions.
- [x] Add a file explorer + editor + terminal workspace variant.
- [x] Add an enterprise approval-center workflow template.
- [x] Add an RBAC access-control matrix template.
- [x] Add an incident operations console template.
- [x] Add a customer 360 workspace template.

## Interaction and production patterns

- [x] Add vertical navigation rail and breadcrumb variants.
- [ ] Add tab overflow, tab context menus, and dirty-tab close confirmation.
- [x] Add grouped lists and bulk-selection actions.
- [ ] Add tree-table and filter-builder data views.
- [x] Add property inspector read-only/edit modes.
- [x] Add date/time and file/folder picker form variants.
- [x] Add unsaved-changes confirmation and discard flows.
- [x] Add notification center and toast stack.
- [x] Add skeleton loading and offline/retry states.
- [x] Add diff/compare viewer.
- [x] Add keyboard focus-order and shortcut demonstrations.

## Web-parity patterns (framework gap analysis)

Patterns modern web UI frameworks provide natively and our gallery should demonstrate how to approximate without first-class support:

- [ ] Add a design-token / theme-customization template (colors-as-patterns, accent scoping).
- [ ] Add a flexbox-style grow/shrink manual layout template.
- [ ] Add an animation and easing template (manual lerp + `request_repaint_after`).
- [ ] Add an RTL / i18n text-direction and locale-formatting template.
- [ ] Add an accessibility labeling and focus-ring styling template.

## Quality and automation

- [x] Add interaction tests for click, keyboard, and state transitions.
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
- [x] Add enterprise approval workflow, access-review, and incident triage patterns.
- [x] Add customer 360 workspace and account-centered view patterns.

## Recommended priority order

The highest-value patterns to add first are:

1. Sidebar + main content + detail pane
2. Split-pane workspace layout
3. Searchable list + detail view
4. Form with validation and advanced settings
5. Dashboard card layout
6. Data table with actions and filters
7. Empty/loading/error states
8. Wizard / stepper flow
9. Resizable multi-panel interface
10. IDE-like shell

## Suggested pattern taxonomy

- Layout fundamentals
- Form patterns
- Navigation patterns
- Data display patterns
- Feedback and status patterns
- Dashboard patterns
- Workspace patterns
- Desktop productivity patterns
- Advanced composition patterns
- Accessibility and polish patterns

## Milestone checklist

### Milestone 1: Single pattern viewer

- [x] Basic app shell
- [x] A few examples loaded into gallery
- [x] Search and selection support
- [x] Copy button works

### Milestone 2: Pattern catalog

- [x] 10+ examples across categories
- [x] Filters and tags
- [x] Metadata and notes

### Milestone 3: Breadth coverage

- [x] 30+ patterns
- [x] Core desktop app categories covered
- [x] Advanced dashboard and workspace layouts included

### Milestone 4: Comprehensive reference

- [ ] 50+ patterns across all major categories
- [ ] Accessibility and polish pass complete
- [ ] Docs and contributor guide complete

## Repository structure direction

- [x] src/
  - [x] app.rs
  - [x] gallery.rs
  - [x] patterns/
    - [x] layout.rs
    - [x] forms.rs
    - [x] data.rs
    - [x] navigation.rs
    - [x] advanced.rs
    - [x] feedback.rs
  - [x] models.rs
  - [x] theme.rs
  - [x] search.rs
  - [x] clipboard.rs
- [x] examples/
  - [x] README.md
- [x] docs/
  - [x] contributing.md
- [x] TODOS.md
- [x] README.md

## Next action

Continue by adding the next high-value workflow and workspace patterns: project portfolio management, AI operations monitoring, and pattern metadata browsing improvements. Use the existing demo registry as the foundation and keep the examples copy-paste friendly.

