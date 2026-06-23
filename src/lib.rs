use anyhow::Result;

pub(crate) mod app;
pub(crate) mod avatars;
pub(crate) mod browser;
pub(crate) mod config;
pub(crate) mod constants;
pub(crate) mod models;
pub(crate) mod process;
pub(crate) mod profiles;
pub(crate) mod ui;

pub fn run() -> Result<()> {
    use std::env;

    use eframe::egui;

    use crate::app::App;
    use crate::config::{load_browser_config, normalize_url};
    use crate::profiles::load_profiles;

    let browser_config = load_browser_config().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });

    let url = normalize_url(&env::args().skip(1).collect::<Vec<_>>().join(" "));
    let profiles = load_profiles(browser_config.config_dir_path());

    if profiles.is_empty() {
        eprintln!("No profiles found in {}.", browser_config.config_dir);
        std::process::exit(1);
    }

    let wm_class = browser_config.wm_class.clone();
    let app = App::new(url, profiles, browser_config)?;
    let title = app.title.clone();

    let width = app.window_width();
    let height = app.window_height();

    let viewport = {
        let viewport = egui::ViewportBuilder::default()
            .with_app_id(wm_class.as_deref().unwrap_or("chromium-profile-picker"))
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_inner_size([width, height])
            .with_min_inner_size([width, height])
            .with_max_inner_size([width, height])
            .with_title(&title);

        #[cfg(target_os = "windows")]
        let viewport = viewport.with_transparent(false);

        viewport
    };

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    match eframe::run_native(
        &title,
        options,
        Box::new(move |cc| Ok(Box::new(app.into_running(cc)))),
    ) {
        Ok(()) => Ok(()),
        Err(err) => Err(anyhow::anyhow!("failed to start GUI: {err}")),
    }
}
