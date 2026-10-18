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
- Launch-option parsing covers `--demo`, `--demo=NAME`, and `--fullscreen`.
- Unsaved-change state transitions cover checkpointing and discard behavior.

The render test intentionally calls every demo through the same `Demo` trait used by the launcher. It catches broken registration, invalid widget construction, and many borrow/state regressions without trying to assert pixels.

The current suite contains 12 tests, including one-frame rendering for all 37
registered demos. Keep this count descriptive rather than treating it as a
hard limit; the registry invariant is the source of truth.

## Adding tests with a new demo

1. Register the demo in `all_demos()`.
2. Add metadata assertions if it introduces a new category or important public pattern.
3. Add a module-local test for validation, sorting, filtering, or navigation rules.
4. Keep rendering smoke-test-safe: do not assume a native display exists.

For visual verification, run `cargo run` on a desktop session and manually exercise the new pattern in both dark and light themes.

For screenshot-enabled compilation without opening a window, run:

```bash
cargo check --features screenshots
```

Actual PNG capture still requires a compositor. It is a documentation and
visual-QA step, not a replacement for the headless test.

## Capturing a template screenshot

The project exposes a reproducible presentation mode for screenshots:

```bash
mkdir -p screenshots
EFRAME_SCREENSHOT_TO=screenshots/dashboard.png \
  cargo run --features screenshots -- \
  --demo "Dashboard" --fullscreen
```

`--demo` accepts a case-insensitive name or name fragment. `--fullscreen` hides the launcher chrome and renders only the selected template. eframe captures the rendered Glow viewport after it settles, writes the PNG, and exits.
