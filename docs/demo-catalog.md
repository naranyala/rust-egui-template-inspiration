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
| Tabs | [`src/demos/tabs.rs`](../src/demos/tabs.rs) | Tab navigation with selectable labels |
| Settings page | [`src/demos/settings.rs`](../src/demos/settings.rs) | Navigation rail, grouped preferences, search |
| Kanban board | [`src/demos/kanban.rs`](../src/demos/kanban.rs) | Horizontal columns, cards, compact repeated surfaces |
| Timeline / activity | [`src/demos/timeline.rs`](../src/demos/timeline.rs) | Date groups, event markers, scrollable activity |
| Command center | [`src/demos/command_center.rs`](../src/demos/command_center.rs) | Dense operations screen, alerts, metrics, live events |
| Form & validation | [`src/demos/forms.rs`](../src/demos/forms.rs) | Label/control grids, inline errors, submit/reset flows |
| Input widgets | [`src/demos/inputs.rs`](../src/demos/inputs.rs) | Sliders, drags, toggles, radio groups, combo boxes, colors |
| Buttons & actions | [`src/demos/buttons.rs`](../src/demos/buttons.rs) | Button styling, menus, disabled state, keyboard hints |
| Data table | [`src/demos/tables.rs`](../src/demos/tables.rs) | `egui_extras::TableBuilder`, sorting, selection, virtualised rows |
| Tree / hierarchy | [`src/demos/tree.rs`](../src/demos/tree.rs) | Recursive nodes, disclosure headers, filtered branches |
| Popups & modals | [`src/demos/popups.rs`](../src/demos/popups.rs) | Tooltips, context menus, menu buttons, modal dialogs |
| Charts | [`src/demos/plots.rs`](../src/demos/plots.rs) | Painter-based axes, lines, bars, hover crosshairs |
| Canvas / drawing | [`src/demos/canvas.rs`](../src/demos/canvas.rs) | Allocated drawing surfaces, pointer input, undo |
| Progress & status | [`src/demos/progress.rs`](../src/demos/progress.rs) | Progress bars, spinners, simulated work, repaint scheduling |

The examples are intentionally adjacent in the source tree. When adapting one, copy the smallest relevant section into your application and keep the demo's state model as a reference.
