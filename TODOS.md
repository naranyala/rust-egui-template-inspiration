# Demo catalog

This page is a quick map from the gallery entries to the source files and the techniques they illustrate.

| Demo | Source | Useful starting point for |
| --- | --- | --- |
| Panel layout | [`src/demos/panels.rs`](../src/demos/panels.rs) | Nested panels, columns, resizable regions, wrapping |
| Dashboard | [`src/demos/dashboard.rs`](../src/demos/dashboard.rs) | KPI cards, responsive wrapping, summary panels |
| Master–detail | [`src/demos/master_detail.rs`](../src/demos/master_detail.rs) | Searchable list, selected record, detail pane |
| Wizard / stepper | [`src/demos/wizard.rs`](../src/demos/wizard.rs) | Guided workflows, validation, progress navigation |
| Grid layout | [`src/demos/grid.rs`](../src/demos/grid.rs) | Aligned rows and columns, nested grids |
| Split panes | [`src/demos/split.rs`](../src/demos/split.rs) | Resizable horizontal and vertical work areas |
| Floating windows | [`src/demos/floating.rs`](../src/demos/floating.rs) | Movable overlay windows (IDE-style) over a canvas |
| Tabs | [`src/demos/tabs.rs`](../src/demos/tabs.rs) | Tab navigation with selectable labels |
| Settings page | [`src/demos/settings.rs`](../src/demos/settings.rs) | Navigation rail, grouped preferences, search |
| Kanban board | [`src/demos/kanban.rs`](../src/demos/kanban.rs) | Horizontal columns, cards, compact repeated surfaces |
| Timeline / activity | [`src/demos/timeline.rs`](../src/demos/timeline.rs) | Date groups, event markers, scrollable activity |
| Command center | [`src/demos/command_center.rs`](../src/demos/command_center.rs) | Dense operations screen, alerts, metrics, live events |
| Approval center | [`src/demos/approval_center.rs`](../src/demos/approval_center.rs) | Approval queues, risk metadata, decision workflows, enterprise review flows |
| Access control matrix | [`src/demos/access_control.rs`](../src/demos/access_control.rs) | Role-based access reviews, user/resource matrix, permission checks |
| Incident console | [`src/demos/incident_console.rs`](../src/demos/incident_console.rs) | Incident triage, alerts, ownership, escalation, timeline tracking |
| Three-pane workspace | [`src/demos/three_pane.rs`](../src/demos/three_pane.rs) | Explorer, editor, and inspector panes |
| Editor workspace | [`src/demos/editor_workspace.rs`](../src/demos/editor_workspace.rs) | File explorer, editor surface, and collapsible terminal |
| Nav rail + breadcrumbs | [`src/demos/nav_rail.rs`](../src/demos/nav_rail.rs) | Compact vertical navigation and contextual breadcrumbs |
| Document tabs | [`src/demos/document_tabs.rs`](../src/demos/document_tabs.rs) | Closable tabs, dirty indicators, new documents |
| Responsive layout | [`src/demos/responsive.rs`](../src/demos/responsive.rs) | Wide/compact composition based on available width |
| Accordion sections | [`src/demos/accordion.rs`](../src/demos/accordion.rs) | Exclusive collapsible groups and section navigation |
| Masonry / flow | [`src/demos/masonry.rs`](../src/demos/masonry.rs) | Variable-height cards balanced across columns |
| Form & validation | [`src/demos/forms.rs`](../src/demos/forms.rs) | Label/control grids, inline errors, submit/reset flows |
| Input widgets | [`src/demos/inputs.rs`](../src/demos/inputs.rs) | Sliders, drags, toggles, radio groups, combo boxes, colors |
| Keyboard focus & shortcuts | [`src/demos/keyboard_nav.rs`](../src/demos/keyboard_nav.rs) | Arrow-key focus traversal and shortcut discoverability |
| Unsaved changes | [`src/demos/unsaved_changes.rs`](../src/demos/unsaved_changes.rs) | Dirty tracking, guarded navigation, and save/discard confirmation |
| Date, time & folder pickers | [`src/demos/pickers.rs`](../src/demos/pickers.rs) | Segmented date/time fields and a filesystem folder browser |
| Buttons & actions | [`src/demos/buttons.rs`](../src/demos/buttons.rs) | Button styling, menus, disabled state, keyboard hints |
| Grouped bulk selection | [`src/demos/bulk_selection.rs`](../src/demos/bulk_selection.rs) | Group selection, select-all controls, and bulk actions |
| Data table | [`src/demos/tables.rs`](../src/demos/tables.rs) | `egui_extras::TableBuilder`, sorting, selection, pagination, virtualised rows |
| Diff / compare | [`src/demos/diff_view.rs`](../src/demos/diff_view.rs) | Unified and side-by-side source comparison |
| Tree / hierarchy | [`src/demos/tree.rs`](../src/demos/tree.rs) | Recursive nodes, disclosure headers, filtered branches |
| Popups & modals | [`src/demos/popups.rs`](../src/demos/popups.rs) | Tooltips, context menus, menu buttons, modal dialogs |
| Charts | [`src/demos/plots.rs`](../src/demos/plots.rs) | Painter-based axes, lines, bars, hover crosshairs |
| Canvas / drawing | [`src/demos/canvas.rs`](../src/demos/canvas.rs) | Allocated drawing surfaces, pointer input, undo |
| Progress & status | [`src/demos/progress.rs`](../src/demos/progress.rs) | Progress bars, spinners, simulated work, repaint scheduling |
| Command palette | [`src/demos/command_palette.rs`](../src/demos/command_palette.rs) | Keyboard-first searchable actions and shortcuts |
| Property inspector | [`src/demos/inspector.rs`](../src/demos/inspector.rs) | Selectable objects and editable/read-only properties |
| Feedback states | [`src/demos/feedback_states.rs`](../src/demos/feedback_states.rs) | Loading, empty, error, retry, skeleton, success, toast, and notification center |
| Calendar / scheduler | [`src/demos/calendar.rs`](../src/demos/calendar.rs) | Month/week planning and selected-day state |
| Theme tokens & focus | [`src/demos/theme_tokens.rs`](../src/demos/theme_tokens.rs) | Accent colors, palette blocks, and high-contrast focus-ready buttons |
| Accessibility & focus | [`src/demos/accessibility_focus.rs`](../src/demos/accessibility_focus.rs) | Labels, keyboard cues, and strong focus relationships for assistive users |

The examples are intentionally adjacent in the source tree. When adapting one, copy the smallest relevant section into your application and keep the demo's state model as a reference.
