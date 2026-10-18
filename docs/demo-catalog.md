# Demo catalog

This page is a quick map from the gallery entries to the source files and the techniques they illustrate.

| Demo | Source | Useful starting point for |
| --- | --- | --- |
| Panel layout | [`src/demos/panels.rs`](../src/demos/panels.rs) | Nested panels, columns, resizable regions, wrapping |
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
