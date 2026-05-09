use eframe::egui;
use egui::{Color32, CornerRadius, Pos2, Rect, Response, Sense, Stroke, StrokeKind, Vec2};

use crate::constants::*;
use crate::models::{AvatarImage, Profile};
use crate::profiles::initials;

pub(crate) fn fit_inside(container: Rect, image_size: Vec2) -> Rect {
    let scale = (container.width() / image_size.x).min(container.height() / image_size.y);
    let size = image_size * scale;
    Rect::from_center_size(container.center(), size)
}

pub(crate) fn header(ui: &mut egui::Ui, ctx: &egui::Context) {
    let width = ui.available_width();
    let height = HEADER_HEIGHT;
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(width, height), Sense::click_and_drag());
    if response.drag_started() {
        ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }

    let painter = ui.painter_at(rect);
    painter.text(
        Pos2::new(rect.center().x, rect.center().y + 1.0),
        egui::Align2::CENTER_CENTER,
        "Helium",
        egui::FontId::proportional(18.0),
        TEXT,
    );

    let close_rect = Rect::from_min_size(
        Pos2::new(rect.max.x - 38.0, rect.center().y - 18.0),
        Vec2::splat(36.0),
    );
    let close = draw_circle_close_button(ui, close_rect);
    if close.clicked() {
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
}

pub(crate) fn url_row(ui: &mut egui::Ui, url: &str) {
    let desired = Vec2::new(ui.available_width(), URL_ROW_HEIGHT);
    let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
    let painter = ui.painter_at(rect);
    let inner = rect.shrink(1.0);
    painter.rect_filled(
        inner,
        CornerRadius::same(19),
        Color32::from_rgb(0x30, 0x30, 0x30),
    );
    painter.rect_stroke(
        inner,
        CornerRadius::same(19),
        Stroke::new(1.0, Color32::from_rgb(0x3a, 0x3a, 0x3a)),
        StrokeKind::Outside,
    );
    painter.text(
        inner.center(),
        egui::Align2::CENTER_CENTER,
        url,
        egui::FontId::proportional(14.0),
        TEXT,
    );
}

pub(crate) fn profile_card(
    ui: &mut egui::Ui,
    rect: Rect,
    profile: &Profile,
    index: usize,
    avatar: Option<&AvatarImage>,
) -> Response {
    let response = ui.interact(rect, ui.id().with(("profile_card", index)), Sense::click());
    let painter = ui.painter_at(rect);

    let bg = if response.hovered() {
        CARD_BG_HOVER
    } else {
        CARD_BG
    };
    painter.rect_filled(rect, CornerRadius::same(18), bg);
    painter.rect_stroke(
        rect,
        CornerRadius::same(18),
        Stroke::new(
            1.0,
            if response.hovered() {
                Color32::from_rgb(0x44, 0x44, 0x44)
            } else {
                BORDER
            },
        ),
        StrokeKind::Outside,
    );

    let avatar_size = Vec2::splat(122.0);
    let avatar_rect =
        Rect::from_center_size(Pos2::new(rect.center().x, rect.top() + 78.0), avatar_size);
    painter.rect_filled(avatar_rect, CornerRadius::same(18), AVATAR_BG);
    painter.rect_stroke(
        avatar_rect,
        CornerRadius::same(18),
        Stroke::new(1.0, AVATAR_BORDER),
        StrokeKind::Outside,
    );

    if let Some(avatar) = avatar {
        let fitted = fit_inside(avatar_rect.shrink(4.0), avatar.size);
        painter.image(
            avatar.texture.id(),
            fitted,
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            Color32::WHITE,
        );
    } else {
        painter.text(
            avatar_rect.center(),
            egui::Align2::CENTER_CENTER,
            initials(&profile.name),
            egui::FontId::proportional(24.0),
            MUTED_TEXT,
        );
    }

    let badge_rect = Rect::from_min_size(
        Pos2::new(avatar_rect.max.x - 36.0, avatar_rect.max.y - 26.0),
        Vec2::new(36.0, 24.0),
    );
    painter.rect_filled(badge_rect, CornerRadius::same(12), BADGE_BG);
    painter.rect_stroke(
        badge_rect,
        CornerRadius::same(12),
        Stroke::new(1.0, BADGE_BORDER),
        StrokeKind::Outside,
    );
    painter.text(
        badge_rect.center(),
        egui::Align2::CENTER_CENTER,
        &(if index + 1 < 10 {
            (index + 1).to_string()
        } else {
            "0".to_string()
        }),
        egui::FontId::proportional(13.0),
        Color32::WHITE,
    );

    let name_pos = Pos2::new(rect.center().x, rect.bottom() - 26.0);
    painter.text(
        name_pos,
        egui::Align2::CENTER_CENTER,
        &profile.name,
        egui::FontId::proportional(15.0),
        TEXT,
    );

    response
}

pub(crate) fn draw_circle_close_button(ui: &mut egui::Ui, rect: Rect) -> Response {
    let response = ui.allocate_rect(rect, Sense::click());
    let painter = ui.painter_at(rect);
    let fill = if response.hovered() {
        CLOSE_BG_HOVER
    } else {
        CLOSE_BG
    };
    let center = rect.center();
    let radius = rect.width() / 2.0;
    painter.circle_filled(center, radius, fill);

    let stroke = Stroke::new(2.0, TEXT);
    let d = 4.0;
    painter.line_segment(
        [
            Pos2::new(center.x - d, center.y - d),
            Pos2::new(center.x + d, center.y + d),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(center.x - d, center.y + d),
            Pos2::new(center.x + d, center.y - d),
        ],
        stroke,
    );
    response
}
