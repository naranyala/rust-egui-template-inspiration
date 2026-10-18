# egui and eframe notes

This project targets `egui`/`eframe` 0.37. The APIs below are worth keeping in mind when adapting the examples.

## The app entry point

The app implements `eframe::App::ui` and receives the root `&mut egui::Ui` directly:

```rust
impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Build the frame here.
    }
}
```

Use `ui.ctx()` when a component needs the `egui::Context`, for example to request a repaint or open a modal.

## Panels

In egui 0.37, `egui::Panel` provides the top, bottom, left, and right variants and can be shown inside a `Ui`. Add panels from the outside in and put `CentralPanel` last. A panel that should resize needs content that consumes its available space; the launcher uses a `ScrollArea` in the sidebar for that reason.

## State and deferred changes

egui rebuilds widgets every frame. Store state in the application or demo struct, and use widget responses to update it. When a closure temporarily borrows data—for example, a table body—collect the intended change and apply it after the closure returns. The table demo uses this approach for sorting and row selection.

## Timers and animations

There is no background repaint loop by default. While work is active, request the next frame:

```rust
ui.ctx().request_repaint_after(std::time::Duration::from_millis(50));
```

The progress and buttons demos use this technique to simulate work without spawning a thread.

## Current API details

- Use `ui.close()` where older examples may show `ui.close_menu()`.
- Use `egui::Id::unique("...")` or `id_salt(...)` for local widget identity.
- Apply theme-specific styles with `Context::style_mut_of(theme, ...)`.
- `egui::Modal` is useful for a centred dialog with a blocking backdrop.

## Renderer choice

`Cargo.toml` disables `eframe` default features and enables `glow`, fonts, accessibility, clipboard, links, system fonts, X11, and Wayland. This keeps the dependency tree smaller for a pattern gallery. If your application needs wgpu-specific rendering, change the feature selection deliberately and expect a larger build.

The `accesskit` feature provides the accessibility integration point, but each
demo still needs meaningful labels, focus behavior, and keyboard flows.
Accessibility should be treated as part of each pattern's implementation
rather than assumed to be inferred by the renderer.

Native execution and screenshot capture depend on a working windowing session.
Headless registry and render tests do not require X11, Wayland, or a compositor.

## Charts without egui_plot

The chart demo uses `Painter` primitives because the selected `egui_plot` release is tied to egui 0.36 while this project uses egui 0.37. Mixing those versions can create incompatible `Color32` and related types. Revisit the dependency when a compatible `egui_plot` release is available.
