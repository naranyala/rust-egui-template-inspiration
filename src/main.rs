mod app;
mod demos;
mod shared;
mod theme;

#[derive(Default)]
struct LaunchOptions {
    demo: Option<String>,
    fullscreen: bool,
}

impl LaunchOptions {
    fn parse() -> Self {
        Self::parse_from(std::env::args().skip(1))
    }

    fn parse_from<I, S>(args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut options = Self::default();
        let mut args = args.into_iter().map(Into::into);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--fullscreen" => options.fullscreen = true,
                "--demo" => options.demo = args.next(),
                value if value.starts_with("--demo=") => {
                    options.demo = Some(value[7..].to_owned());
                }
                "--help" | "-h" => {
                    println!("Usage: rust-egui-template-inspiration [--demo NAME] [--fullscreen]");
                    std::process::exit(0);
                }
                unknown => eprintln!("Ignoring unknown argument: {unknown}"),
            }
        }
        options
    }
}

fn main() -> Result<(), eframe::Error> {
    let launch = LaunchOptions::parse();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("egui pattern launcher")
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([760.0, 520.0])
            .with_fullscreen(launch.fullscreen),
        ..Default::default()
    };

    eframe::run_native(
        "rust-egui-template-inspiration",
        options,
        Box::new(|cc| {
            let theme = theme::ThemeState::default();
            theme.apply(&cc.egui_ctx);
            Ok(Box::new(app::LauncherApp::new(
                cc,
                theme,
                launch.demo.as_deref(),
                launch.fullscreen,
            )))
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::LaunchOptions;

    #[test]
    fn parses_demo_and_fullscreen_flags() {
        let options = LaunchOptions::parse_from([
            "--demo".to_owned(),
            "Kanban board".to_owned(),
            "--fullscreen".to_owned(),
        ]);

        assert_eq!(options.demo.as_deref(), Some("Kanban board"));
        assert!(options.fullscreen);
    }

    #[test]
    fn parses_equals_form_and_ignores_unknown_flags() {
        let options = LaunchOptions::parse_from(["--demo=Dashboard", "--unknown"]);

        assert_eq!(options.demo.as_deref(), Some("Dashboard"));
        assert!(!options.fullscreen);
    }
}
