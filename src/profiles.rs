use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use serde_json::Value;

use crate::models::Profile;

pub(crate) fn load_profiles(config_dir: PathBuf) -> Vec<Profile> {
    let mut profiles = Vec::new();
    let local_state = config_dir.join("Local State");

    let add_profile = |dir_name: String, info: &Value, profiles: &mut Vec<Profile>| {
        if dir_name == "System Profile" {
            return;
        }
        profiles.push(Profile {
            directory: dir_name,
            name: info
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_owned(),
            avatar_url: info
                .get("avatar_icon")
                .and_then(Value::as_str)
                .map(str::to_owned),
            custom_avatar_picture_file_name: info
                .get("custom_avatar_picture_file_name")
                .and_then(Value::as_str)
                .map(str::to_owned),
            is_using_custom_avatar: info
                .get("is_using_custom_avatar")
                .and_then(Value::as_bool)
                .unwrap_or(false),
        });
    };

    if let Ok(text) = fs::read_to_string(&local_state) {
        if let Ok(value) = serde_json::from_str::<Value>(&text) {
            if let Some(cache) = value
                .get("profile")
                .and_then(|p| p.get("info_cache"))
                .and_then(Value::as_object)
            {
                let order = value
                    .get("profile")
                    .and_then(|p| p.get("profiles_order"))
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default();

                let mut seen = HashSet::new();
                for entry in order {
                    if let Some(dir) = entry.as_str() {
                        if let Some(info) = cache.get(dir) {
                            add_profile(dir.to_owned(), info, &mut profiles);
                            seen.insert(dir.to_owned());
                        }
                    }
                }

                for (dir, info) in cache {
                    if !seen.contains(dir) {
                        add_profile(dir.to_owned(), info, &mut profiles);
                    }
                }
            }
        }
    }

    if profiles.is_empty() && config_dir.exists() {
        if let Ok(entries) = fs::read_dir(&config_dir) {
            let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();
            entries.sort_by_key(|e| e.file_name());
            for entry in entries {
                let path = entry.path();
                if path.is_dir() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name != "System Profile" {
                        profiles.push(Profile {
                            directory: name.clone(),
                            name,
                            avatar_url: None,
                            custom_avatar_picture_file_name: None,
                            is_using_custom_avatar: false,
                        });
                    }
                }
            }
        }
    }

    for profile in &mut profiles {
        if profile.name.is_empty() {
            profile.name = profile.directory.clone();
        }
    }

    profiles
}

pub(crate) fn initials(name: &str) -> String {
    let replaced = name.replace('_', " ");
    let parts: Vec<_> = replaced
        .split_whitespace()
        .filter(|p| !p.is_empty())
        .collect();
    match parts.as_slice() {
        [] => "?".to_owned(),
        [one] => one.chars().take(2).collect::<String>().to_uppercase(),
        [a, b, ..] => format!(
            "{}{}",
            a.chars().next().unwrap_or('?').to_uppercase(),
            b.chars().next().unwrap_or('?').to_uppercase()
        ),
    }
}
