use anyhow::Result;

pub(crate) mod app;
pub(crate) mod avatars;
pub(crate) mod config;
pub(crate) mod constants;
pub(crate) mod models;
pub(crate) mod process;
pub(crate) mod profiles;
pub(crate) mod ui;

pub fn run() -> Result<()> {
    use std::env;

    use eframe::egui;

    use crate::app::HeliumApp;
    use crate::config::normalize_url;
    use crate::process::resolve_helium_bin;
    use crate::profiles::load_profiles;

    let url = normalize_url(&env::args().skip(1).collect::<Vec<_>>().join(" "));
    let profiles = load_profiles();

    if profiles.is_empty() {
        eprintln!("No Helium profiles found.");
        std::process::exit(1);
    }

    let helium_bin = resolve_helium_bin();
    let app = HeliumApp::new(url, profiles, helium_bin)?;

    let width = app.window_width();
    let height = app.window_height();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id("helium")
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_inner_size([width, height])
            .with_min_inner_size([width, height])
            .with_max_inner_size([width, height])
            .with_title("Helium"),
        ..Default::default()
    };

    match eframe::run_native(
        "Helium",
        options,
        Box::new(move |cc| Ok(Box::new(app.into_running(cc)))),
    ) {
        Ok(()) => Ok(()),
        Err(err) => Err(anyhow::anyhow!("failed to start GUI: {err}")),
    }
}
