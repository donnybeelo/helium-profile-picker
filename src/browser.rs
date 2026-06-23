#[cfg(all(feature = "arc", target_os = "linux"))]
compile_error!("Arc Browser is not available on Linux.");

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Persisted browser configuration written to browser.json on first run.
/// Edit this file to override auto-detected paths.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct BrowserConfig {
    /// Human-readable browser name.
    pub name: String,
    /// Path to the browser executable.
    pub binary: String,
    /// Path to the browser's user data directory (the one containing "Local State").
    pub config_dir: String,
    /// X11 WM_CLASS for window manager rules (Linux).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wm_class: Option<String>,
    /// macOS bundle identifier, e.g. for `open -b` or default-browser tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
}

impl BrowserConfig {
    pub fn config_dir_path(&self) -> PathBuf {
        PathBuf::from(&self.config_dir)
    }
}

#[allow(dead_code)]
struct BrowserDef {
    name: &'static str,
    wm_class: Option<&'static str>,
    bundle_id: Option<&'static str>,
    linux_bins: &'static [&'static str],
    linux_config: &'static str,
    macos_app: &'static str,
    macos_bin_subpath: &'static str,
    macos_config: &'static str,
    /// (env_var_name, relative_path) pairs tried in order.
    windows_bins: &'static [(&'static str, &'static str)],
    windows_config: (&'static str, &'static str),
}

const CHROME: BrowserDef = BrowserDef {
    name: "Google Chrome",
    wm_class: Some("google-chrome"),
    bundle_id: Some("com.google.Chrome"),
    linux_bins: &["google-chrome-stable", "google-chrome", "chrome"],
    linux_config: ".config/google-chrome",
    macos_app: "Google Chrome.app",
    macos_bin_subpath: "Contents/MacOS/Google Chrome",
    macos_config: "Google/Chrome",
    windows_bins: &[("LOCALAPPDATA", "Google\\Chrome\\Application\\chrome.exe")],
    windows_config: ("LOCALAPPDATA", "Google\\Chrome\\User Data"),
};

const CHROMIUM: BrowserDef = BrowserDef {
    name: "Chromium",
    wm_class: Some("chromium"),
    bundle_id: Some("org.chromium.Chromium"),
    linux_bins: &["chromium-browser", "chromium"],
    linux_config: ".config/chromium",
    macos_app: "Chromium.app",
    macos_bin_subpath: "Contents/MacOS/Chromium",
    macos_config: "Chromium",
    windows_bins: &[("LOCALAPPDATA", "Chromium\\Application\\chrome.exe")],
    windows_config: ("LOCALAPPDATA", "Chromium\\User Data"),
};

const BRAVE: BrowserDef = BrowserDef {
    name: "Brave Browser",
    wm_class: Some("brave-browser"),
    bundle_id: Some("com.brave.Browser"),
    linux_bins: &["brave-browser", "brave"],
    linux_config: ".config/BraveSoftware/Brave-Browser",
    macos_app: "Brave Browser.app",
    macos_bin_subpath: "Contents/MacOS/Brave Browser",
    macos_config: "BraveSoftware/Brave-Browser",
    windows_bins: &[("LOCALAPPDATA", "BraveSoftware\\Brave-Browser\\Application\\brave.exe")],
    windows_config: ("LOCALAPPDATA", "BraveSoftware\\Brave-Browser\\User Data"),
};

const EDGE: BrowserDef = BrowserDef {
    name: "Microsoft Edge",
    wm_class: Some("microsoft-edge"),
    bundle_id: Some("com.microsoft.edgemac"),
    linux_bins: &["microsoft-edge-stable", "microsoft-edge"],
    linux_config: ".config/microsoft-edge",
    macos_app: "Microsoft Edge.app",
    macos_bin_subpath: "Contents/MacOS/Microsoft Edge",
    macos_config: "Microsoft Edge",
    windows_bins: &[
        ("ProgramFiles(x86)", "Microsoft\\Edge\\Application\\msedge.exe"),
        ("LOCALAPPDATA", "Microsoft\\Edge\\Application\\msedge.exe"),
    ],
    windows_config: ("LOCALAPPDATA", "Microsoft\\Edge\\User Data"),
};

const VIVALDI: BrowserDef = BrowserDef {
    name: "Vivaldi",
    wm_class: Some("vivaldi-stable"),
    bundle_id: Some("com.vivaldi.Vivaldi"),
    linux_bins: &["vivaldi-stable", "vivaldi"],
    linux_config: ".config/vivaldi",
    macos_app: "Vivaldi.app",
    macos_bin_subpath: "Contents/MacOS/Vivaldi",
    macos_config: "Vivaldi",
    windows_bins: &[("LOCALAPPDATA", "Vivaldi\\Application\\vivaldi.exe")],
    windows_config: ("LOCALAPPDATA", "Vivaldi\\User Data"),
};

const OPERA: BrowserDef = BrowserDef {
    name: "Opera",
    wm_class: Some("opera"),
    bundle_id: Some("com.operasoftware.Opera"),
    linux_bins: &["opera"],
    linux_config: ".config/opera",
    macos_app: "Opera.app",
    macos_bin_subpath: "Contents/MacOS/Opera",
    macos_config: "com.operasoftware.Opera",
    windows_bins: &[("LOCALAPPDATA", "Programs\\Opera\\launcher.exe")],
    windows_config: ("APPDATA", "Opera Software\\Opera Stable"),
};

const ARC: BrowserDef = BrowserDef {
    name: "Arc",
    wm_class: None,
    bundle_id: Some("company.thebrowser.Browser"),
    linux_bins: &[],
    linux_config: "",
    macos_app: "Arc.app",
    macos_bin_subpath: "Contents/MacOS/Arc",
    macos_config: "Arc/User Data",
    windows_bins: &[("LOCALAPPDATA", "Programs\\Arc\\Arc.exe")],
    windows_config: ("LOCALAPPDATA", "Arc\\User Data"),
};

const HELIUM: BrowserDef = BrowserDef {
    name: "Helium",
    wm_class: Some("helium"),
    bundle_id: Some("net.imput.helium"),
    linux_bins: &["helium"],
    linux_config: ".config/net.imput.helium",
    macos_app: "Helium.app",
    macos_bin_subpath: "Contents/MacOS/Helium",
    macos_config: "net.imput.helium",
    windows_bins: &[],
    windows_config: ("LOCALAPPDATA", "net.imput.helium"),
};

fn selected_def() -> &'static BrowserDef {
    if cfg!(feature = "brave") { &BRAVE }
    else if cfg!(feature = "edge") { &EDGE }
    else if cfg!(feature = "vivaldi") { &VIVALDI }
    else if cfg!(feature = "opera") { &OPERA }
    else if cfg!(feature = "arc") { &ARC }
    else if cfg!(feature = "helium") { &HELIUM }
    else if cfg!(feature = "chromium") { &CHROMIUM }
    else { &CHROME }
}

pub(crate) fn selected_name() -> &'static str {
    selected_def().name
}

pub(crate) fn detect_browser_config() -> anyhow::Result<BrowserConfig> {
    let def = selected_def();
    Ok(BrowserConfig {
        name: def.name.to_owned(),
        binary: detect_binary(def)?,
        config_dir: detect_config_dir(def)?.to_string_lossy().to_string(),
        wm_class: def.wm_class.map(str::to_owned),
        bundle_id: def.bundle_id.map(str::to_owned),
    })
}

// ── Linux ─────────────────────────────────────────────────────────────────────

#[cfg(target_os = "linux")]
fn detect_binary(def: &BrowserDef) -> anyhow::Result<String> {
    use crate::process::which;
    def.linux_bins.iter().find_map(|&n| which(n)).ok_or_else(|| {
        anyhow::anyhow!("{} not found in PATH. Set 'binary' in browser.json.", def.name)
    })
}

#[cfg(target_os = "linux")]
fn detect_config_dir(def: &BrowserDef) -> anyhow::Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine home directory"))?;
    Ok(home.join(def.linux_config))
}

// ── macOS ─────────────────────────────────────────────────────────────────────

#[cfg(target_os = "macos")]
fn detect_binary(def: &BrowserDef) -> anyhow::Result<String> {
    let mut search = vec![PathBuf::from("/Applications")];
    if let Some(home) = dirs::home_dir() {
        search.push(home.join("Applications"));
    }
    search
        .into_iter()
        .map(|base| base.join(def.macos_app).join(def.macos_bin_subpath))
        .find(|p| p.exists())
        .map(|p| p.to_string_lossy().to_string())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "{} not found in /Applications or ~/Applications. Set 'binary' in browser.json.",
                def.name
            )
        })
}

#[cfg(target_os = "macos")]
fn detect_config_dir(def: &BrowserDef) -> anyhow::Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine home directory"))?;
    Ok(home.join("Library").join("Application Support").join(def.macos_config))
}

// ── Windows ───────────────────────────────────────────────────────────────────

#[cfg(target_os = "windows")]
fn detect_binary(def: &BrowserDef) -> anyhow::Result<String> {
    def.windows_bins
        .iter()
        .filter_map(|(var, rel)| std::env::var(var).ok().map(|base| PathBuf::from(base).join(rel)))
        .find(|p| p.exists())
        .map(|p| p.to_string_lossy().to_string())
        .ok_or_else(|| {
            anyhow::anyhow!("{} not found. Set 'binary' in browser.json.", def.name)
        })
}

#[cfg(target_os = "windows")]
fn detect_config_dir(def: &BrowserDef) -> anyhow::Result<PathBuf> {
    let (var, rel) = def.windows_config;
    let base = std::env::var(var).map_err(|_| anyhow::anyhow!("{var} not set"))?;
    Ok(PathBuf::from(base).join(rel))
}

// ── Other platforms ───────────────────────────────────────────────────────────

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn detect_binary(_def: &BrowserDef) -> anyhow::Result<String> {
    Err(anyhow::anyhow!("Unsupported platform. Set 'binary' in browser.json."))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn detect_config_dir(_def: &BrowserDef) -> anyhow::Result<PathBuf> {
    Err(anyhow::anyhow!("Unsupported platform. Set 'config_dir' in browser.json."))
}
