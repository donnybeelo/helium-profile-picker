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

pub(crate) fn header(ui: &mut egui::Ui, ctx: &egui::Context, url: Option<&str>, app: &mut crate::app::App) {
    let width = ui.available_width();
    let height = HEADER_HEIGHT;
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());

    let painter = ui.painter_at(rect);
    let close_rect = Rect::from_min_size(
        Pos2::new(rect.max.x - URL_ROW_HEIGHT, rect.top() - 5.0),
        Vec2::splat(URL_ROW_HEIGHT),
    );
    let drag_rect = Rect::from_min_max(
        Pos2::new(
            rect.left() - PANEL_INNER_PADDING,
            rect.top() - PANEL_INNER_PADDING,
        ),
        Pos2::new(
            rect.right() + PANEL_INNER_PADDING,
            rect.bottom() - CONTENT_GAP,
        ),
    );
    let drag_response = ui.interact(drag_rect, ui.id().with("header_drag_zone"), Sense::click());
    if drag_response.is_pointer_button_down_on() && ctx.input(|i| i.pointer.primary_pressed()) {
        ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }

    if let Some(url) = url {
        let address_rect = Rect::from_min_size(
            Pos2::new(rect.left() + 2.0, rect.top() - 5.0),
            Vec2::new(rect.width() - 48.0, URL_ROW_HEIGHT),
        );

        let pointer_started_in_address = ctx.input(|i| {
            i.pointer.primary_pressed()
                && i.pointer
                    .interact_pos()
                    .is_some_and(|pos| address_rect.contains(pos))
        });
        if pointer_started_in_address {
            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }

        url_row(ui, address_rect, url, app, ctx);
    } else {
        let title_pos = Pos2::new(rect.center().x, rect.top() + 10.0);
        let title_font = egui::FontId::proportional(16.0);
        for offset in [Vec2::new(-0.1, 0.0), Vec2::ZERO] {
            painter.text(
                title_pos + offset,
                egui::Align2::CENTER_CENTER,
                &app.title,
                title_font.clone(),
                TEXT,
            );
        }
    }

    let close = draw_circle_close_button(ui, close_rect);
    if close.clicked() {
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
}

pub(crate) fn url_row(ui: &mut egui::Ui, rect: Rect, url: &str, app: &mut crate::app::App, ctx: &egui::Context) {
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
        Stroke::new(1.0, BORDER),
        StrokeKind::Outside,
    );

    let response = ui.interact(rect, ui.id().with("url_row_interaction"), Sense::click_and_drag());
    
    if response.secondary_clicked() {
        ui.ctx().copy_text(url.to_string());
        app.show_copy_popup(ctx);
    }

    ui.scope_builder(egui::UiBuilder::new().max_rect(inner), |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.add_space(URL_ROW_HEIGHT / 3.0);
            egui::ScrollArea::horizontal()
                .id_salt("url_row_scroll")
                .max_height(inner.height())
                .max_width(inner.width() - URL_ROW_HEIGHT / 1.5)
                .auto_shrink([false; 2])
                .scroll_source(egui::scroll_area::ScrollSource::MOUSE_WHEEL)
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.add_sized(
                        ui.available_size(),
                        egui::Label::new(
                            egui::RichText::new(url)
                                .font(egui::FontId::proportional(14.0))
                                .color(TEXT)
                                .monospace(),
                        )
                        .selectable(false)
                        .wrap_mode(egui::TextWrapMode::Extend),
                    );
                });
        });
    });
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
    let hovered = ui.rect_contains_pointer(rect);
    let clicked = response.is_pointer_button_down_on();

    let bg = if clicked {
        CARD_BG_CLICKED
    } else if hovered {
        CARD_BG_HOVER
    } else {
        Color32::TRANSPARENT
    };
    painter.rect_filled(rect, CornerRadius::same(18), bg);

    let avatar_size = Vec2::splat(122.0);
    let avatar_rect =
        Rect::from_center_size(Pos2::new(rect.center().x, rect.top() + 78.0), avatar_size);
    painter.rect_filled(avatar_rect, CornerRadius::same(18), AVATAR_BG);
    painter.rect_stroke(
        avatar_rect,
        CornerRadius::same(18),
        Stroke::new(1.0, BORDER),
        StrokeKind::Outside,
    );

    if let Some(avatar) = avatar {
        let fitted = fit_inside(avatar_rect, avatar.size);
        painter.add(
            egui::epaint::RectShape::filled(fitted, CornerRadius::same(18), Color32::WHITE)
                .with_texture(
                    avatar.texture.id(),
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                ),
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

    const BADGE_WIDTH: f32 = 36.0;
    const BADGE_HEIGHT: f32 = 24.0;

    let badge_rect = Rect::from_min_size(
        Pos2::new(
            avatar_rect.max.x - BADGE_WIDTH - 4.0,
            avatar_rect.max.y - BADGE_HEIGHT - 4.0,
        ),
        Vec2::new(BADGE_WIDTH, BADGE_HEIGHT),
    );
    painter.rect_filled(badge_rect, CornerRadius::same(12), BADGE_BG);
    painter.rect_stroke(
        badge_rect,
        CornerRadius::same(12),
        Stroke::new(1.0, BADGE_BORDER),
        StrokeKind::Inside,
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
    let hovered = ui.rect_contains_pointer(rect);
    let fill = if hovered { CLOSE_BG_HOVER } else { CLOSE_BG };
    let center = rect.center();
    let radius = rect.width() / 2.0 - 1.5;
    painter.circle_filled(center, radius, fill);
    painter.circle_stroke(center, radius, Stroke::new(1.0, BORDER));

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
