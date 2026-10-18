# Adding a pattern

New examples are registered explicitly. This keeps the catalogue easy to scan and makes each pattern's order and category obvious.

## 1. Create the module

Add `src/demos/my_pattern.rs`:

```rust
use super::{Category, Demo};

#[derive(Default)]
pub struct MyPattern {
    // State that must survive from one frame to the next.
    count: u32,
}

impl Demo for MyPattern {
    fn name(&self) -> &'static str {
        "My pattern"
    }

    fn description(&self) -> &'static str {
        "A short explanation of the egui technique this pattern demonstrates."
    }

    fn category(&self) -> Category {
        Category::Widgets
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        if ui.button("Increment").clicked() {
            self.count += 1;
        }
        ui.label(format!("Clicked {} times", self.count));
    }
}
```

Put durable interaction state in the struct. Avoid creating it as a local variable in `ui`; locals are recreated on every frame.

## 2. Declare and register it

In `src/demos/mod.rs`, add the module declaration:

```rust
pub mod my_pattern;
```

Then add a default instance to `all_demos()`:

```rust
Box::<my_pattern::MyPattern>::default(),
```

The sidebar will use `name`, `description`, and `category` automatically.

## 3. Keep the example focused

A good pattern should demonstrate one technique clearly. Prefer:

- small, deterministic sample data;
- controls that visibly change the example;
- comments explaining an egui-specific choice;
- a description that says what a reader can reuse.

If the example needs a timer or animation, call `ui.ctx().request_repaint_after(...)` while it is active. Immediate-mode UIs do not repaint just because time has passed.

## 4. Verify the change

```bash
cargo fmt -- --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo run
```

Confirm that the pattern appears under the expected category, remains usable after switching patterns, and behaves correctly in both dark and light themes.
