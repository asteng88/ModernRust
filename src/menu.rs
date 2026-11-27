//! Animated Fluent dropdown & context menus for ModernRust.
//!
//! Features smooth vertical slide-fade entrance, animated hover highlight with a vertical
//! Fluent left accent indicator pill, crisp vector icon slots, checkmarks, semantic danger
//! tinting, and right-aligned keyboard shortcut badges.

use crate::anim;
use crate::icons::{self, Icon};
use crate::theme::{blend, Role, ThemePalette};
use egui::{pos2, vec2, Align2, Color32, CornerRadius, FontId, Frame, Margin, Rect, Sense, Stroke, StrokeKind, Ui};

/// A single item inside an `AnimatedMenu`.
#[derive(Clone, Debug)]
pub enum MenuEntry {
    Command {
        id: &'static str,
        label: String,
        icon: Option<Icon>,
        shortcut: String,
        role: Option<Role>,
        enabled: bool,
    },
    Check {
        id: &'static str,
        label: String,
        checked: bool,
        shortcut: String,
    },
    Separator,
    Section(String),
}

impl MenuEntry {
    pub fn command(id: &'static str, label: impl Into<String>) -> Self {
        Self::Command {
            id,
            label: label.into(),
            icon: None,
            shortcut: String::new(),
            role: None,
            enabled: true,
        }
    }

    pub fn with_icon(mut self, ic: Icon) -> Self {
        if let Self::Command { icon, .. } = &mut self {
            *icon = Some(ic);
        }
        self
    }

    pub fn with_shortcut(mut self, sc: impl Into<String>) -> Self {
        match &mut self {
            Self::Command { shortcut, .. } | Self::Check { shortcut, .. } => *shortcut = sc.into(),
            _ => {}
        }
        self
    }

    pub fn with_role(mut self, r: Role) -> Self {
        if let Self::Command { role, .. } = &mut self {
            *role = Some(r);
        }
        self
    }

    pub fn check(id: &'static str, label: impl Into<String>, checked: bool) -> Self {
        Self::Check {
            id,
            label: label.into(),
            checked,
            shortcut: String::new(),
        }
    }
}

/// Renders an animated Fluent menu surface inside a popup/overlay and returns the clicked item's `id`.
pub fn show_menu_body(
    ui: &mut Ui,
    palette: &ThemePalette,
    menu_id: egui::Id,
    width: f32,
    entries: &[MenuEntry],
) -> Option<&'static str> {
    let open_t = anim::animate_open(ui.ctx(), menu_id.with("open"), true);
    let slide_y = (1.0 - open_t) * -6.0;
    let mut clicked: Option<&'static str> = None;

    ui.add_space(slide_y.max(0.0));
    Frame::new()
        .fill(palette.surface.gamma_multiply(0.35 + 0.65 * open_t))
        .stroke(Stroke::new(1.0, palette.border))
        .corner_radius(CornerRadius::same(palette.radius_md))
        .inner_margin(Margin::symmetric(6, 6))
        .show(ui, |ui| {
            ui.set_min_width(width);
            ui.spacing_mut().item_spacing.y = 2.0;

            for (idx, entry) in entries.iter().enumerate() {
                match entry {
                    MenuEntry::Separator => {
                        let (r, _) = ui.allocate_exact_size(vec2(width, 9.0), Sense::hover());
                        ui.painter().line_segment(
                            [pos2(r.min.x + 8.0, r.center().y), pos2(r.max.x - 8.0, r.center().y)],
                            Stroke::new(1.0, palette.divider),
                        );
                    }
                    MenuEntry::Section(title) => {
                        let (r, _) = ui.allocate_exact_size(vec2(width, 20.0), Sense::hover());
                        ui.painter().text(
                            pos2(r.min.x + 12.0, r.center().y),
                            Align2::LEFT_CENTER,
                            title.to_uppercase(),
                            FontId::proportional(10.0),
                            palette.fg_subtle,
                        );
                    }
                    MenuEntry::Command {
                        id,
                        label,
                        icon,
                        shortcut,
                        role,
                        enabled,
                    } => {
                        let (r, resp) = ui.allocate_exact_size(
                            vec2(width, 30.0),
                            if *enabled { Sense::click() } else { Sense::hover() },
                        );
                        let hov_t = anim::animate_hover(ui.ctx(), menu_id.with(("row", idx)), *enabled && resp.hovered());
                        let accent = role.map_or(palette.primary, |rl| palette.role_color(rl));

                        if hov_t > 0.01 {
                            let bg = if let Some(rl) = role {
                                blend(palette.surface, palette.role_subtle_bg(*rl), hov_t)
                            } else {
                                blend(palette.surface, palette.surface_hover, hov_t)
                            };
                            ui.painter().rect_filled(r, CornerRadius::same(palette.radius_sm), bg);

                            // Animated Fluent vertical accent indicator pill on the left
                            let bar_h = 14.0 * hov_t;
                            let bar = Rect::from_center_size(pos2(r.min.x + 3.5, r.center().y), vec2(3.0, bar_h));
                            ui.painter().rect_filled(bar, 1.5, accent.gamma_multiply(hov_t));
                        }

                        let fg = if !*enabled {
                            palette.fg_subtle
                        } else if role.is_some() {
                            accent
                        } else if hov_t > 0.4 {
                            palette.fg
                        } else {
                            blend(palette.fg, palette.fg_muted, 0.15)
                        };

                        if let Some(ic) = icon {
                            let ir = Rect::from_center_size(pos2(r.min.x + 20.0, r.center().y), vec2(15.0, 15.0));
                            icons::draw(ui.painter(), ir, *ic, fg, accent);
                        }
                        ui.painter().text(
                            pos2(r.min.x + 36.0, r.center().y),
                            Align2::LEFT_CENTER,
                            label,
                            FontId::proportional(12.5),
                            fg,
                        );
                        if !shortcut.is_empty() {
                            ui.painter().text(
                                pos2(r.max.x - 10.0, r.center().y),
                                Align2::RIGHT_CENTER,
                                shortcut,
                                FontId::proportional(11.0),
                                palette.fg_subtle,
                            );
                        }
                        if *enabled && resp.clicked() {
                            clicked = Some(*id);
                        }
                    }
                    MenuEntry::Check {
                        id,
                        label,
                        checked,
                        shortcut,
                    } => {
                        let (r, resp) = ui.allocate_exact_size(vec2(width, 30.0), Sense::click());
                        let hov_t = anim::animate_hover(ui.ctx(), menu_id.with(("chk", idx)), resp.hovered());
                        if hov_t > 0.01 {
                            let bg = blend(palette.surface, palette.surface_hover, hov_t);
                            ui.painter().rect_filled(r, CornerRadius::same(palette.radius_sm), bg);
                            let bar = Rect::from_center_size(pos2(r.min.x + 3.5, r.center().y), vec2(3.0, 14.0 * hov_t));
                            ui.painter().rect_filled(bar, 1.5, palette.primary.gamma_multiply(hov_t));
                        }
                        if *checked {
                            let ir = Rect::from_center_size(pos2(r.min.x + 20.0, r.center().y), vec2(14.0, 14.0));
                            icons::draw(ui.painter(), ir, Icon::Check, palette.primary, palette.primary);
                        }
                        ui.painter().text(
                            pos2(r.min.x + 36.0, r.center().y),
                            Align2::LEFT_CENTER,
                            label,
                            FontId::proportional(12.5),
                            palette.fg,
                        );
                        if !shortcut.is_empty() {
                            ui.painter().text(
                                pos2(r.max.x - 10.0, r.center().y),
                                Align2::RIGHT_CENTER,
                                shortcut,
                                FontId::proportional(11.0),
                                palette.fg_subtle,
                            );
                        }
                        if resp.clicked() {
                            clicked = Some(*id);
                        }
                    }
                }
            }
        });

    clicked
}

/// Top-bar button that opens a sleek animated Fluent dropdown menu underneath it.
pub fn menu_button(
    ui: &mut Ui,
    palette: &ThemePalette,
    label: &str,
    menu_width: f32,
    entries: &[MenuEntry],
) -> Option<&'static str> {
    let btn_id = ui.scope_id().with(("mr_menu_btn", label));
    let (r, resp) = ui.allocate_exact_size(vec2(label.len() as f32 * 7.8 + 20.0, 26.0), Sense::click());
    let popup_id = egui::Popup::default_response_id(&resp);
    let is_open = egui::Popup::is_id_open(ui.ctx(), popup_id);
    let hov_t = anim::animate_hover(ui.ctx(), btn_id, resp.hovered() || is_open);

    if hov_t > 0.01 {
        let fill = if is_open {
            palette.role_subtle_bg(Role::Primary)
        } else {
            blend(Color32::TRANSPARENT, palette.surface_hover, hov_t)
        };
        ui.painter().rect_filled(r, CornerRadius::same(palette.radius_sm), fill);
        if is_open {
            ui.painter().rect_stroke(r, CornerRadius::same(palette.radius_sm), Stroke::new(1.0, palette.primary.gamma_multiply(0.45)), StrokeKind::Inside);
        }
    }
    ui.painter().text(
        r.center(),
        Align2::CENTER_CENTER,
        label,
        FontId::proportional(12.5),
        if is_open { palette.primary } else { palette.fg },
    );

    let mut picked = None;
    egui::Popup::menu(&resp)
        .frame(Frame::NONE)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClick)
        .show(|ui| {
            picked = show_menu_body(ui, palette, popup_id, menu_width, entries);
        });
    picked
}
