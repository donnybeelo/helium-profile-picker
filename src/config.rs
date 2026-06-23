use std::env;
use std::path::PathBuf;

use crate::browser::{detect_browser_config, BrowserConfig};
use crate::constants::{APP_ID, APP_NAME};

pub(crate) fn normalize_url(raw: &str) -> Option<String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return None;
    }

    if raw.contains("://") {
        return Some(raw.to_owned());
    } else if raw.starts_with("//") {
        return Some(format!("https:{raw}"));
    } else if raw.starts_with('/') {
        return Some(raw.to_owned());
    }

    Some(format!("https://{raw}"))
}

pub(crate) fn app_config_dir() -> PathBuf {
    dirs::config_dir().unwrap_or_else(|| PathBuf::from(".")).join(APP_ID)
}

pub(crate) fn app_cache_dir() -> PathBuf {
    dirs::cache_dir().unwrap_or_else(|| PathBuf::from(".")).join(APP_NAME)
}

/// Load browser.json, using the cache only if it was generated for the same compiled-in browser.
/// BROWSER_BIN and BROWSER_CONFIG_DIR env vars override the file values.
pub(crate) fn load_browser_config() -> anyhow::Result<BrowserConfig> {
    let dir = app_config_dir();
    let path = dir.join("browser.json");

    let cached = std::fs::read_to_string(&path)
        .ok()
        .and_then(|text| serde_json::from_str::<BrowserConfig>(&text).ok())
        .filter(|c| c.name == crate::browser::selected_name());

    let mut config = if let Some(c) = cached {
        c
    } else {
        let detected = detect_browser_config().map_err(|e| {
            anyhow::anyhow!(
                "{e}\n\nCreate {} with at least:\n  {{\n    \"name\": \"...\",\n    \"binary\": \"/path/to/browser\",\n    \"config_dir\": \"/path/to/profile/dir\"\n  }}",
                path.display()
            )
        })?;
        std::fs::create_dir_all(&dir)?;
        std::fs::write(&path, serde_json::to_string_pretty(&detected)?)?;
        detected
    };

    if let Ok(bin) = env::var("BROWSER_BIN") {
        config.binary = bin;
    }
    if let Ok(cfg_dir) = env::var("BROWSER_CONFIG_DIR") {
        config.config_dir = cfg_dir;
    }

    Ok(config)
}
