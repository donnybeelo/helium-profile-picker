use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;
use eframe::{egui, egui::Vec2};
use egui::{Color32, CornerRadius, Sense, Stroke, StrokeKind};

use crate::avatars::load_avatar_texture;
use crate::config::helium_config_dir;
use crate::constants::*;
use crate::models::{AvatarImage, Profile, ProfileView};
use crate::process::launch;
use crate::ui::{header, profile_card, url_row};

pub(crate) struct HeliumApp {
    pub(crate) url: Option<String>,
    pub(crate) profiles: Vec<ProfileView>,
    pub(crate) avatars: HashMap<String, AvatarImage>,
    pub(crate) helium_bin: String,
    pub(crate) error: Option<String>,
}

impl HeliumApp {
    pub(crate) fn new(
        url: Option<String>,
        profiles: Vec<Profile>,
        helium_bin: String,
    ) -> Result<Self> {
        let mut views = Vec::with_capacity(profiles.len());
        let config_dir = helium_config_dir();

        for profile in profiles {
            let avatar_path = if profile.is_using_custom_avatar {
                profile
                    .custom_avatar_picture_file_name
                    .as_ref()
                    .and_then(|name| {
                        let path = config_dir.join(&profile.directory).join(name);
                        if path.is_file() {
                            Some(path)
                        } else {
                            None
                        }
                    })
            } else {
                None
            };

            views.push(ProfileView {
                profile,
                avatar_path,
            });
        }

        Ok(Self {
            url,
            profiles: views,
            avatars: HashMap::new(),
            helium_bin,
            error: None,
        })
    }

    pub(crate) fn into_running(mut self, cc: &eframe::CreationContext<'_>) -> Self {
        for profile in &self.profiles {
            if let Some(path) = self.avatar_path_for(profile) {
                let key = path.to_string_lossy().to_string();
                if !self.avatars.contains_key(&key) {
                    if let Ok(img) = load_avatar_texture(&cc.egui_ctx, &path) {
                        self.avatars.insert(key, img);
                    }
                }
            }
        }
        self
    }

    pub(crate) fn window_width(&self) -> f32 {
        let cols = self.column_count();
        let gaps = cols.saturating_sub(1) as f32 * CARD_MARGIN_X;
        2.0 * PANEL_INNER_PADDING + cols as f32 * CARD_WIDTH + gaps
    }

    pub(crate) fn window_height(&self) -> f32 {
        let rows = ((self.profiles.len() as f32) / self.column_count() as f32).ceil() as usize;
        let header = HEADER_HEIGHT;
        let url = if self.url.is_some() {
            URL_ROW_HEIGHT + CONTENT_GAP * 0.75
        } else {
            0.0
        };
        let grid = rows as f32 * CARD_HEIGHT + rows.saturating_sub(1) as f32 * PROFILE_ROW_GAP;
        2.0 * PANEL_INNER_PADDING + header + url + CONTENT_GAP + grid
    }

    pub(crate) fn column_count(&self) -> usize {
        self.profiles.len().clamp(1, 4)
    }

    pub(crate) fn avatar_path_for(&self, view: &ProfileView) -> Option<PathBuf> {
        if let Some(path) = &view.avatar_path {
            return Some(path.clone());
        }
        view.profile
            .avatar_url
            .as_ref()
            .and_then(|url| crate::avatars::ensure_avatar_png(url))
    }

    pub(crate) fn launch_profile(&mut self, profile_dir: &str) -> Result<()> {
        let result = launch(&self.helium_bin, self.url.as_deref(), profile_dir);
        match &result {
            Ok(_) => self.error = None,
            Err(err) => self.error = Some(format!("Could not launch Helium: {err:#}")),
        }
        result
    }

    fn profile_grid(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let columns = self.column_count();
        let rows = ((self.profiles.len() as f32) / columns as f32).ceil() as usize;

        for row in 0..rows {
            let start = row * columns;
            let end = ((row + 1) * columns).min(self.profiles.len());
            let count = end.saturating_sub(start);
            let row_width =
                count as f32 * CARD_WIDTH + count.saturating_sub(1) as f32 * CARD_MARGIN_X;

            ui.horizontal_centered(|ui| {
                ui.allocate_ui_with_layout(
                    Vec2::new(row_width, CARD_HEIGHT),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(CARD_MARGIN_X, 0.0);
                        for idx in start..end {
                            let (card_rect, _) = ui.allocate_exact_size(
                                Vec2::new(CARD_WIDTH, CARD_HEIGHT),
                                Sense::hover(),
                            );

                            let view = &self.profiles[idx];
                            let path = self.avatar_path_for(view);
                            let avatar = path
                                .as_ref()
                                .and_then(|p| self.avatars.get(&p.to_string_lossy().to_string()));
                            let response = profile_card(ui, card_rect, &view.profile, idx, avatar);
                            if response.clicked() {
                                let dir = view.profile.directory.clone();
                                let _ = self.launch_profile(&dir);
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        }
                    },
                );
            });
            if row + 1 < rows {
                ui.add_space(PROFILE_ROW_GAP);
            }
        }
    }
}

impl eframe::App for HeliumApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals {
            panel_fill: WINDOW_BG,
            window_fill: WINDOW_BG,
            override_text_color: Some(MUTED_TEXT),
            ..egui::Visuals::dark()
        });
        ctx.request_repaint();

        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        let pressed = [
            (egui::Key::Num1, 0usize),
            (egui::Key::Num2, 1),
            (egui::Key::Num3, 2),
            (egui::Key::Num4, 3),
            (egui::Key::Num5, 4),
            (egui::Key::Num6, 5),
            (egui::Key::Num7, 6),
            (egui::Key::Num8, 7),
            (egui::Key::Num9, 8),
            (egui::Key::Num0, 9),
        ];
        for (key, idx) in pressed {
            if ctx.input(|i| i.key_pressed(key)) && idx < self.profiles.len() {
                let dir = self.profiles[idx].profile.directory.clone();
                let _ = self.launch_profile(&dir);
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                break;
            }
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(Color32::TRANSPARENT))
            .show(ctx, |ui| {
                let panel_rect = ui.max_rect();
                let painter = ui.painter();
                painter.rect_filled(panel_rect, CornerRadius::same(30), PANEL_BG);
                painter.rect_stroke(
                    panel_rect,
                    CornerRadius::same(30),
                    Stroke::new(1.0, Color32::from_rgb(0x2f, 0x2f, 0x2f)),
                    StrokeKind::Outside,
                );

                ui.scope_builder(
                    egui::UiBuilder::new().max_rect(panel_rect.shrink(PANEL_INNER_PADDING)),
                    |ui| {
                        ui.vertical(|ui| {
                            header(ui, ctx);
                            if let Some(url) = &self.url {
                                url_row(ui, url);
                                ui.add_space(6.0);
                            }
                            ui.add_space(8.0);
                            self.profile_grid(ui, ctx);
                            if let Some(err) = &self.error {
                                ui.add_space(12.0);
                                ui.label(
                                    egui::RichText::new(err)
                                        .color(Color32::from_rgb(0xff, 0x95, 0x95)),
                                );
                            }
                        });
                    },
                );
            });
    }
}
