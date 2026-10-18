//! Entry point.
//!
//! All the interesting code lives in [`app`] (the launcher shell) and
//! [`demos`] (one module per UI pattern). Start reading at `app.rs`.

// Hide the console window when building a release binary on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod demos;
mod theme;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("egui pattern launcher")
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([760.0, 520.0]),
        ..Default::default()
    };

    // The app name doubles as the persistence id (window position, egui memory).
    eframe::run_native(
        "rust-egui-template-inspiration",
        options,
        Box::new(|cc| {
            // Set up theme/fonts before the first frame is drawn.
            let theme = theme::ThemeState::default();
            theme.apply(&cc.egui_ctx);
            Ok(Box::new(app::LauncherApp::new(cc, theme)))
        }),
    )
}
