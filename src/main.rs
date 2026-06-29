#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

fn main() -> anyhow::Result<()> {
    chromium_profile_picker::run()
}
