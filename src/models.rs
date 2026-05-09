use std::path::PathBuf;

use eframe::egui;

#[derive(Clone, Debug)]
pub(crate) struct Profile {
    pub(crate) directory: String,
    pub(crate) name: String,
    pub(crate) avatar_url: Option<String>,
    pub(crate) custom_avatar_picture_file_name: Option<String>,
    pub(crate) is_using_custom_avatar: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct ProfileView {
    pub(crate) profile: Profile,
    pub(crate) avatar_path: Option<PathBuf>,
}

pub(crate) struct AvatarImage {
    pub(crate) texture: egui::TextureHandle,
    pub(crate) size: egui::Vec2,
}
