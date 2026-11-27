//! Fluent Side Panel headers, elevated Section Cards, and docked collapse tabs.

use crate::icons::{self, Icon};
use crate::theme::ThemePalette;
use egui::{vec2, Align2, CornerRadius, FontId, Frame, Margin, RichText, Sense, Stroke, StrokeKind, Ui};

/// Renders a crisp Fluent side-panel header with title, optional badge, and close `✕` button.
/// Returns `true` if the close button was clicked.
pub fn side_panel_header(
    ui: &mut Ui,
    palette: &ThemePalette,
    title: &str,
    badge: Option<&str>,
) -> bool {
    let mut closed = false;
    ui.horizontal(|ui| {
        ui.label(RichText::new(title).strong().size(14.5).color(palette.fg));
        if let Some(b) = badge {
            let (br, _) = ui.allocate_exact_size(vec2(b.len() as f32 * 6.5 + 14.0, 19.0), Sense::hover());
            ui.painter().rect_filled(br, 9.5, palette.surface_alt);
            ui.painter().text(br.center(), Align2::CENTER_CENTER, b, FontId::proportional(10.5), palette.fg_muted);
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let (cr, resp) = ui.allocate_exact_size(vec2(24.0, 24.0), Sense::click());
            if resp.hovered() {
                ui.painter().rect_filled(cr, CornerRadius::same(palette.radius_sm), palette.surface_hover);
            }
            icons::draw(ui.painter(), cr.shrink(5.5), Icon::Close, palette.fg_muted, palette.primary);
            if resp.on_hover_text("Close panel").clicked() {
                closed = true;
            }
        });
    });
    ui.add_space(2.0);
    let (div_r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter().line_segment([div_r.left_center(), div_r.right_center()], Stroke::new(1.0, palette.divider));
    ui.add_space(6.0);
    closed
}

/// Elevated rounded Fluent card container with optional uppercase section title.
pub fn section_card<R>(
    ui: &mut Ui,
    palette: &ThemePalette,
    title: Option<&str>,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    Frame::new()
        .fill(palette.surface)
        .stroke(Stroke::new(1.0, palette.border))
        .corner_radius(CornerRadius::same(palette.radius_lg))
        .inner_margin(Margin::same(14))
        .show(ui, |ui| {
            if let Some(t) = title {
                ui.label(RichText::new(t).strong().size(13.5).color(palette.fg));
                ui.add_space(6.0);
            }
            add_contents(ui)
        })
        .inner
}

/// Renders a sleek docked 18px collapse/expand edge tab (`‹` / `›`) for side panels.
pub fn edge_collapse_tab(ui: &mut Ui, palette: &ThemePalette, collapsed: &mut bool, label: &str) {
    let avail = ui.available_rect_before_wrap();
    let tab_h = 68.0_f32.min(avail.height().max(24.0));
    let tab_rect = egui::Rect::from_min_size(
        egui::pos2(avail.min.x, avail.center().y - tab_h * 0.5),
        vec2(18.0, tab_h),
    );
    let resp = ui.interact(tab_rect, ui.scope_id().with(("mr_edge_tab", label)), Sense::click());
    let bg = if resp.hovered() { palette.surface_hover } else { palette.surface };
    let cr = CornerRadius { nw: 8, sw: 8, ne: 0, se: 0 };
    ui.painter().rect_filled(tab_rect, cr, bg);
    ui.painter().rect_stroke(tab_rect, cr, Stroke::new(1.0, palette.border), StrokeKind::Inside);
    let ic = if *collapsed { Icon::ChevronLeft } else { Icon::ChevronRight };
    let col = if resp.hovered() || *collapsed { palette.primary } else { palette.fg_muted };
    icons::draw(ui.painter(), egui::Rect::from_center_size(tab_rect.center(), vec2(12.0, 12.0)), ic, col, palette.primary);
    let tip = if *collapsed { format!("Show {label}") } else { format!("Hide {label}") };
    if resp.on_hover_text(tip).clicked() {
        *collapsed = !*collapsed;
    }
}
