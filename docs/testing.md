# Testing guide

The project uses lightweight unit tests plus a headless egui render smoke test. This keeps the examples testable without requiring a window manager, compositor, or GPU.

## Run the suite

```bash
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt -- --check
```

## What is covered

- The demo registry contains exactly the expected catalogue size.
- Every demo name is unique and every description is non-empty.
- All categories are represented, including the new layout templates.
- Every registered demo can build one complete egui frame in a headless context.
- Form validation rejects incomplete input and accepts a complete form.
- Table sorting orders rows in both directions.
- Tree filtering preserves matching ancestor folders.
- Tree file icons map known extensions correctly.
- Wizard navigation blocks incomplete steps.

The render test intentionally calls every demo through the same `Demo` trait used by the launcher. It catches broken registration, invalid widget construction, and many borrow/state regressions without trying to assert pixels.

## Adding tests with a new demo

1. Register the demo in `all_demos()`.
2. Add metadata assertions if it introduces a new category or important public pattern.
3. Add a module-local test for validation, sorting, filtering, or navigation rules.
4. Keep rendering smoke-test-safe: do not assume a native display exists.

For visual verification, run `cargo run` on a desktop session and manually exercise the new pattern in both dark and light themes.
