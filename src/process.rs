use std::env;

use std::process::{Command, Stdio};

use anyhow::{Context, Result};

pub(crate) fn which(exe: &str) -> Option<String> {
    let paths = env::var_os("PATH")?;
    for path in env::split_paths(&paths) {
        let candidate = path.join(exe);
        if candidate.is_file() {
            return Some(candidate.to_string_lossy().to_string());
        }
        #[cfg(target_os = "windows")]
        {
            let candidate_exe = path.join(format!("{exe}.exe"));
            if candidate_exe.is_file() {
                return Some(candidate_exe.to_string_lossy().to_string());
            }
        }
    }
    None
}

pub(crate) fn launch(browser_bin: &str, url: Option<&str>, profile_dir: &str) -> Result<()> {
    let mut cmd = Command::new(browser_bin);
    cmd.arg(format!("--profile-directory={profile_dir}"));
    if let Some(url) = url {
        cmd.arg(url);
    }

    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const DETACHED_PROCESS: u32 = 0x0000_0008;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;
        cmd.creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP);
    }

    cmd.spawn()
        .with_context(|| format!("launching {browser_bin}"))?;
    Ok(())
}
