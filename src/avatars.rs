use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use eframe::egui;
use sha1::{Digest, Sha1};

use crate::config::app_cache_dir;

pub(crate) fn cache_key(text: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(text.as_bytes());
    hex::encode(hasher.finalize())
}

pub(crate) fn chrome_pak_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    #[cfg(target_os = "linux")]
    {
        candidates.extend(
            [
                "/opt/google/chrome/chrome_100_percent.pak",
                "/opt/google/chrome/resources.pak",
                "/usr/share/chromium/chrome_100_percent.pak",
                "/usr/share/chromium/resources.pak",
                "/usr/lib/chromium/chrome_100_percent.pak",
                "/usr/lib/chromium/resources.pak",
            ]
            .into_iter()
            .map(PathBuf::from),
        );
    }

    #[cfg(target_os = "macos")]
    {
        candidates.extend([
            "/Applications/Google Chrome.app/Contents/Frameworks/Google Chrome Framework.framework/Versions/Current/Resources/chrome_100_percent.pak",
            "/Applications/Google Chrome.app/Contents/Frameworks/Google Chrome Framework.framework/Versions/Current/Resources/resources.pak",
            "/Applications/Chromium.app/Contents/Frameworks/Chromium Framework.framework/Versions/Current/Resources/chrome_100_percent.pak",
            "/Applications/Chromium.app/Contents/Frameworks/Chromium Framework.framework/Versions/Current/Resources/resources.pak",
        ]
        .into_iter()
        .map(PathBuf::from));
    }

    #[cfg(target_os = "windows")]
    {
        let roots = [
            std::env::var("PROGRAMFILES").ok(),
            std::env::var("PROGRAMFILES(X86)").ok(),
            std::env::var("LOCALAPPDATA").ok(),
        ];
        let subpaths = [
            "Google/Chrome/Application/chrome_100_percent.pak",
            "Google/Chrome/Application/resources.pak",
            "Chromium/Application/chrome_100_percent.pak",
            "Chromium/Application/resources.pak",
            "Microsoft/Edge/Application/chrome_100_percent.pak",
            "Microsoft/Edge/Application/resources.pak",
        ];
        for root in roots.into_iter().flatten() {
            for sub in subpaths {
                candidates.push(PathBuf::from(&root).join(sub));
            }
        }
    }

    candidates
}

pub(crate) fn extract_pngs_from_pak(pak_path: &Path, out_dir: &Path) -> Result<Vec<PathBuf>> {
    fs::create_dir_all(out_dir)?;
    let marker = out_dir.join(".done");
    if marker.exists() {
        let mut pngs = vec![];
        for entry in fs::read_dir(out_dir)? {
            let entry = entry?;
            if entry.path().extension().and_then(|e| e.to_str()) == Some("png") {
                pngs.push(entry.path());
            }
        }
        pngs.sort();
        return Ok(pngs);
    }

    let data = fs::read(pak_path)?;
    let sig = b"\x89PNG\r\n\x1a\n";
    let mut idx = 0;
    let mut count = 0;
    let mut pngs = Vec::new();

    while let Some(start) = find_bytes(&data, sig, idx) {
        let mut pos = start + sig.len();
        while pos + 8 <= data.len() {
            let length = u32::from_be_bytes(data[pos..pos + 4].try_into().unwrap()) as usize;
            let ctype = &data[pos + 4..pos + 8];
            pos += 8 + length + 4;
            if ctype == b"IEND" {
                let file = out_dir.join(format!("{count:03}_{start}.png"));
                fs::write(&file, &data[start..pos])?;
                pngs.push(file);
                count += 1;
                idx = pos;
                break;
            }
        }
    }

    fs::write(marker, count.to_string())?;
    pngs.sort();
    Ok(pngs)
}

pub(crate) fn find_bytes(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    haystack[start..]
        .windows(needle.len())
        .position(|w| w == needle)
        .map(|i| start + i)
}

pub(crate) fn ensure_avatar_png(avatar_url: &str) -> Option<PathBuf> {
    let cache_root = app_cache_dir().join("avatars");
    let _ = fs::create_dir_all(&cache_root);

    for pak_path in chrome_pak_candidates() {
        if !pak_path.exists() {
            continue;
        }
        let pak_cache = cache_root.join(pak_path.file_stem()?.to_string_lossy().to_string());
        if let Ok(pngs) = extract_pngs_from_pak(&pak_path, &pak_cache) {
            let idx = avatar_url
                .rsplit('_')
                .next()
                .and_then(|s| s.parse::<usize>().ok())
                .map(|n| n + 74)
                .unwrap_or(usize::MAX);
            if idx < pngs.len() {
                return Some(pngs[idx].clone());
            }
        }
    }

    let out = cache_root.join(format!("{}.png", cache_key(avatar_url)));
    if out.exists() && out.metadata().map(|m| m.len()).unwrap_or(0) > 0 {
        return Some(out);
    }
    None
}

pub(crate) fn load_avatar_texture(
    ctx: &egui::Context,
    path: &Path,
) -> Result<crate::models::AvatarImage> {
    let bytes = fs::read(path).with_context(|| format!("reading avatar {path:?}"))?;
    let image =
        image::load_from_memory(&bytes).with_context(|| format!("decoding image {path:?}"))?;
    let rgba = image.to_rgba8();
    let size = egui::Vec2::new(rgba.width() as f32, rgba.height() as f32);
    let color_image = egui::ColorImage::from_rgba_unmultiplied(
        [rgba.width() as usize, rgba.height() as usize],
        &rgba,
    );
    let texture = ctx.load_texture(
        path.to_string_lossy(),
        color_image,
        egui::TextureOptions::LINEAR,
    );
    Ok(crate::models::AvatarImage { texture, size })
}
