use std::env;
use std::path::PathBuf;

use crate::constants::{APP_ID, APP_NAME};

pub(crate) fn normalize_url(raw: &str) -> Option<String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return None;
    }

    if raw.contains("://") {
        return Some(raw.to_owned());
    }
    if raw.starts_with("//") {
        return Some(format!("https:{raw}"));
    }
    Some(format!("https://{raw}"))
}

pub(crate) fn app_config_dir() -> PathBuf {
    if let Ok(override_dir) = env::var("HELIUM_CONFIG_DIR") {
        return PathBuf::from(override_dir);
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(base) = env::var("LOCALAPPDATA").or_else(|_| env::var("APPDATA")) {
            return PathBuf::from(base).join(APP_ID);
        }
        if let Some(home) = dirs_home() {
            return home.join("AppData").join("Local").join(APP_ID);
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(home) = dirs_home() {
            return home
                .join("Library")
                .join("Application Support")
                .join(APP_ID);
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
            return PathBuf::from(xdg).join(APP_ID);
        }
        if let Some(home) = dirs_home() {
            return home.join(".config").join(APP_ID);
        }
    }

    PathBuf::from(".")
}

pub(crate) fn app_cache_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Ok(base) = env::var("LOCALAPPDATA").or_else(|_| env::var("APPDATA")) {
            return PathBuf::from(base).join(APP_NAME).join("cache");
        }
        if let Some(home) = dirs_home() {
            return home
                .join("AppData")
                .join("Local")
                .join(APP_NAME)
                .join("cache");
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(home) = dirs_home() {
            return home.join("Library").join("Caches").join(APP_NAME);
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        if let Ok(xdg) = env::var("XDG_CACHE_HOME") {
            return PathBuf::from(xdg).join(APP_NAME);
        }
        if let Some(home) = dirs_home() {
            return home.join(".cache").join(APP_NAME);
        }
    }

    PathBuf::from(".")
}

pub(crate) fn dirs_home() -> Option<PathBuf> {
    dirs::home_dir()
}

pub(crate) fn helium_config_dir() -> PathBuf {
    app_config_dir()
}
