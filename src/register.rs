use std::path::PathBuf;

use anyhow::Result;
use winreg::enums::*;
use winreg::RegKey;

const PROG_ID: &str = "ChromiumProfilePickerURL";
const APP_KEY: &str = "Software\\ChromiumProfilePicker";
const CAPS_KEY: &str = "Software\\ChromiumProfilePicker\\Capabilities";

fn exe_path() -> Result<PathBuf> {
    std::env::current_exe().map_err(|e| anyhow::anyhow!("Cannot find exe path: {e}"))
}

/// Register the app as a URL handler so it appears in Settings → Default apps.
/// The user still needs to select it there — Windows prevents programmatic assignment.
pub(crate) fn register() -> Result<()> {
    let exe = exe_path()?;
    let exe_str = exe.to_string_lossy();
    let cmd = format!("\"{exe_str}\" \"%1\"");
    let browser_name = crate::browser::selected_name();
    let app_name = format!("{browser_name} Profile Picker");

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // ProgID — describes how to open URLs
    let (prog, _) = hkcu.create_subkey(format!("Software\\Classes\\{PROG_ID}"))?;
    prog.set_value("", &format!("URL:{app_name}"))?;
    prog.set_value("URL Protocol", &"")?;

    let (icon, _) = prog.create_subkey("DefaultIcon")?;
    icon.set_value("", &format!("{exe_str},0"))?;

    let (cmd_key, _) = prog.create_subkey("shell\\open\\command")?;
    cmd_key.set_value("", &cmd)?;

    // Capabilities
    let (caps, _) = hkcu.create_subkey(CAPS_KEY)?;
    caps.set_value("ApplicationName", &app_name.as_str())?;
    caps.set_value("ApplicationDescription", &format!("Picks a profile before opening {browser_name}").as_str())?;

    let (url_assoc, _) = caps.create_subkey("URLAssociations")?;
    url_assoc.set_value("http", &PROG_ID)?;
    url_assoc.set_value("https", &PROG_ID)?;

    // RegisteredApplications — makes the app show up in Default apps
    let (reg_apps, _) = hkcu.create_subkey("Software\\RegisteredApplications")?;
    reg_apps.set_value("ChromiumProfilePicker", &CAPS_KEY)?;

    Ok(())
}

/// Open Windows Settings at the Default apps page so the user can select this app.
pub(crate) fn open_default_apps_settings() {
    let _ = std::process::Command::new("cmd")
        .args(["/c", "start", "ms-settings:defaultapps"])
        .spawn();
}

/// Remove all registry entries written by register().
pub(crate) fn unregister() -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let _ = hkcu.delete_subkey_all(format!("Software\\Classes\\{PROG_ID}"));
    let _ = hkcu.delete_subkey_all(APP_KEY);
    if let Ok(reg_apps) = hkcu.open_subkey_with_flags("Software\\RegisteredApplications", KEY_WRITE) {
        let _ = reg_apps.delete_value("ChromiumProfilePicker");
    }
    Ok(())
}
