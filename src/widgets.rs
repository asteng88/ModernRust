//! Core ModernRust Fluent widgets: semantic buttons, animated segmented pill,
//! animated toggle switch, quick-insert chips, badges, and progress bar.

use crate::anim;
use crate::icons::{self, Icon};
use crate::theme::{blend, Role, ThemePalette};
use egui::{pos2, vec2, Align2, Color32, CornerRadius, FontId, Rect, Response, Sense, Stroke, StrokeKind, Ui};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ButtonVariant {
    #[default]
    Filled,
    Outline,
    Ghost,
    Pill,
}

/// Semantic Fluent button with optional leading vector icon and smooth hover animation.
pub fn modern_button(
    ui: &mut Ui,
    palette: &ThemePalette,
    label: &str,
    role: Role,
    variant: ButtonVariant,
    icon: Option<Icon>,
) -> Response {
    let icon_w = if icon.is_some() { 20.0 } else { 0.0 };
    let text_w = label.chars().count() as f32 * 7.2;
    let w = (text_w + icon_w + 24.0).max(68.0);
    let h = 30.0;
    let (rect, resp) = ui.allocate_exact_size(vec2(w, h), Sense::click());

    let id = ui.scope_id().with(("mr_btn", label, role));
    let hov_t = anim::animate_hover(ui.ctx(), id, resp.hovered());
    let radius = if variant == ButtonVariant::Pill {
        CornerRadius::same(15)
    } else {
        CornerRadius::same(palette.radius_sm)
    };

    let (fill, stroke_col, fg) = match variant {
        ButtonVariant::Filled | ButtonVariant::Pill => {
            let base = palette.role_color(role);
            let bg = if resp.is_pointer_button_down_on() {
                palette.role_active(role)
            } else {
                blend(base, palette.role_hover(role), hov_t)
            };
            (bg, Color32::TRANSPARENT, palette.role_text(role))
        }
        ButtonVariant::Outline => {
            let accent = palette.role_color(role);
            let bg = blend(palette.surface, palette.role_subtle_bg(role), hov_t);
            (bg, blend(palette.border_strong, accent, hov_t), blend(palette.fg, accent, hov_t * 0.6))
        }
        ButtonVariant::Ghost => {
            let bg = blend(Color32::TRANSPARENT, palette.surface_hover, hov_t);
            (bg, Color32::TRANSPARENT, palette.fg)
        }
    };

    ui.painter().rect_filled(rect, radius, fill);
    if stroke_col != Color32::TRANSPARENT {
        ui.painter().rect_stroke(rect, radius, Stroke::new(1.0, stroke_col), StrokeKind::Inside);
    }

    let content_x = if let Some(ic) = icon {
        let ir = Rect::from_center_size(pos2(rect.min.x + 17.0, rect.center().y), vec2(14.0, 14.0));
        icons::draw(ui.painter(), ir, ic, fg, fg);
        rect.min.x + 30.0
    } else {
        rect.center().x
    };

    let align = if icon.is_some() { Align2::LEFT_CENTER } else { Align2::CENTER_CENTER };
    ui.painter().text(pos2(content_x, rect.center().y), align, label, FontId::proportional(12.5), fg);
    resp
}

/// Animated Fluent segmented pill control with a gliding active thumb (ideal for `[ Read | Edit ]`).
/// Returns `true` if `selected` changed.
pub fn segmented_pill(
    ui: &mut Ui,
    palette: &ThemePalette,
    id_salt: &str,
    options: &[&str],
    selected: &mut usize,
) -> bool {
    if options.is_empty() {
        return false;
    }
    let count = options.len();
    let seg_w = 64.0;
    let total_w = seg_w * count as f32;
    let h = 28.0;
    let (rect, _) = ui.allocate_exact_size(vec2(total_w, h), Sense::hover());

    ui.painter().rect_filled(rect, CornerRadius::same(14), palette.surface_alt);
    ui.painter().rect_stroke(rect, CornerRadius::same(14), Stroke::new(1.0, palette.border), StrokeKind::Inside);

    let anim_id = ui.scope_id().with(("mr_seg_pill", id_salt));
    let target_x = (*selected).min(count - 1) as f32 * seg_w;
    let cur_x = anim::animate_scalar(ui.ctx(), anim_id, target_x, 0.14);

    let thumb = Rect::from_min_size(pos2(rect.min.x + cur_x + 2.0, rect.min.y + 2.0), vec2(seg_w - 4.0, h - 4.0));
    ui.painter().rect_filled(thumb, CornerRadius::same(12), palette.primary);

    let mut changed = false;
    for (idx, lbl) in options.iter().enumerate() {
        let sr = Rect::from_min_size(pos2(rect.min.x + idx as f32 * seg_w, rect.min.y), vec2(seg_w, h));
        let resp = ui.interact(sr, anim_id.with(idx), Sense::click());
        let active = *selected == idx;
        let fg = if active {
            palette.role_text(Role::Primary)
        } else if resp.hovered() {
            palette.fg
        } else {
            palette.fg_muted
        };
        ui.painter().text(sr.center(), Align2::CENTER_CENTER, *lbl, FontId::proportional(12.0), fg);
        if resp.clicked() && !active {
            *selected = idx;
            changed = true;
        }
    }
    changed
}

/// Smooth animated Windows 11 Fluent toggle switch.
pub fn fluent_switch(ui: &mut Ui, palette: &ThemePalette, label: &str, on: &mut bool) -> Response {
    let mut clicked = false;
    let resp = ui
        .horizontal(|ui| {
            let (track, r) = ui.allocate_exact_size(vec2(38.0, 20.0), Sense::click());
            let t = anim::animate_bool(ui.ctx(), ui.scope_id().with(("mr_sw", label)), *on, 0.14);
            let bg = blend(palette.surface_alt, palette.primary, t);
            let stroke = blend(palette.border_strong, palette.primary, t);
            ui.painter().rect_filled(track, CornerRadius::same(10), bg);
            ui.painter().rect_stroke(track, CornerRadius::same(10), Stroke::new(1.0, stroke), StrokeKind::Inside);

            let knob_x = track.min.x + 10.0 + t * (track.width() - 20.0);
            let knob_r = if r.hovered() { 7.0 } else { 6.2 };
            let knob_col = blend(palette.fg_muted, palette.role_text(Role::Primary), t);
            ui.painter().circle_filled(pos2(knob_x, track.center().y), knob_r, knob_col);

            ui.add_space(4.0);
            let lbl_r = ui.add(egui::Label::new(egui::RichText::new(label).size(12.5).color(palette.fg)).sense(Sense::click()));
            if r.clicked() || lbl_r.clicked() {
                *on = !*on;
                clicked = true;
            }
            r
        })
        .inner;
    let _ = clicked;
    resp
}

/// Clickable Fluent pill chip (ideal for quick-insert tags like `"Page X of Y"`, `"Strictly Confidential"`).
pub fn chip(ui: &mut Ui, palette: &ThemePalette, label: &str, selected: bool) -> Response {
    let w = (label.chars().count() as f32 * 6.8 + 18.0).max(46.0);
    let (rect, resp) = ui.allocate_exact_size(vec2(w, 24.0), Sense::click());
    let hov_t = anim::animate_hover(ui.ctx(), ui.scope_id().with(("mr_chip", label)), resp.hovered());
    let bg = if selected {
        palette.role_subtle_bg(Role::Primary)
    } else {
        blend(palette.surface_alt, palette.surface_hover, hov_t)
    };
    let border = if selected { palette.primary } else { blend(palette.border, palette.primary, hov_t * 0.5) };
    let fg = if selected { palette.primary } else { palette.fg };

    ui.painter().rect_filled(rect, CornerRadius::same(12), bg);
    ui.painter().rect_stroke(rect, CornerRadius::same(12), Stroke::new(1.0, border), StrokeKind::Inside);
    ui.painter().text(rect.center(), Align2::CENTER_CENTER, label, FontId::proportional(11.5), fg);
    resp
}

/// Smooth rounded progress bar with optional percentage label.
pub fn progress_bar(ui: &mut Ui, palette: &ThemePalette, fraction: f32, role: Role, show_percent: bool) {
    let frac = fraction.clamp(0.0, 1.0);
    let h = if show_percent { 18.0 } else { 8.0 };
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width().max(80.0), h), Sense::hover());
    let r = CornerRadius::same((h * 0.5) as u8);
    ui.painter().rect_filled(rect, r, palette.surface_alt);
    ui.painter().rect_stroke(rect, r, Stroke::new(1.0, palette.border), StrokeKind::Inside);
    let fill_w = rect.width() * frac;
    if fill_w > 2.0 {
        let bar = Rect::from_min_size(rect.min, vec2(fill_w.max(h), h));
        ui.painter().rect_filled(bar, r, palette.role_color(role));
    }
    if show_percent {
        let fg = if frac > 0.52 { palette.role_text(role) } else { palette.fg };
        ui.painter().text(rect.center(), Align2::CENTER_CENTER, format!("{}%", (frac * 100.0).round() as u32), FontId::proportional(10.5), fg);
    }
}
