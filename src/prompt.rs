//! Fluent Prompt Boxes (`PromptBox`) and Semantic Toast Stack (`ToastStack`) for ModernRust.

use crate::anim;
use crate::icons::{self, Icon};
use crate::theme::{Role, ThemePalette};
use crate::widgets::{modern_button, ButtonVariant};
use egui::{pos2, vec2, Align2, Area, Color32, Context, CornerRadius, FontId, Frame, Id, Margin, Order, Rect, Sense, Stroke, StrokeKind, Ui};

/// Result returned by a `PromptBox`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PromptOutcome {
    Pending,
    Confirm,
    Secondary,
    Cancel,
}

/// Clean Fluent confirmation / input prompt card (usable inline in a side panel or centered overlay).
pub struct PromptBox<'a> {
    pub title: &'a str,
    pub message: &'a str,
    pub role: Role,
    pub confirm_label: &'a str,
    pub secondary_label: Option<&'a str>,
    pub cancel_label: &'a str,
}

impl<'a> PromptBox<'a> {
    pub fn new(title: &'a str, message: &'a str) -> Self {
        Self {
            title,
            message,
            role: Role::Primary,
            confirm_label: "Confirm",
            secondary_label: None,
            cancel_label: "Cancel",
        }
    }

    pub fn role(mut self, role: Role) -> Self {
        self.role = role;
        self
    }

    pub fn confirm_label(mut self, label: &'a str) -> Self {
        self.confirm_label = label;
        self
    }

    pub fn secondary_label(mut self, label: &'a str) -> Self {
        self.secondary_label = Some(label);
        self
    }

    pub fn cancel_label(mut self, label: &'a str) -> Self {
        self.cancel_label = label;
        self
    }

    /// Renders the prompt card inside any `Ui` (e.g., side panel or showcase card).
    pub fn show_inline(&self, ui: &mut Ui, palette: &ThemePalette) -> PromptOutcome {
        let mut outcome = PromptOutcome::Pending;
        let accent = palette.role_color(self.role);
        let icon = match self.role {
            Role::Success => Icon::Check,
            Role::Warning => Icon::Warning,
            Role::Danger => Icon::Trash,
            _ => Icon::Info,
        };

        Frame::new()
            .fill(palette.surface)
            .stroke(Stroke::new(1.0, palette.border))
            .corner_radius(CornerRadius::same(palette.radius_lg))
            .inner_margin(Margin::same(16))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let (badge_r, _) = ui.allocate_exact_size(vec2(30.0, 30.0), Sense::hover());
                    ui.painter().rect_filled(badge_r, 8.0, palette.role_subtle_bg(self.role));
                    icons::draw(ui.painter(), badge_r.shrink(6.5), icon, accent, accent);
                    ui.add_space(6.0);
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(self.title).strong().size(14.5).color(palette.fg));
                        ui.add_space(2.0);
                        ui.label(egui::RichText::new(self.message).size(12.5).color(palette.fg_muted));
                    });
                });
                ui.add_space(14.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if modern_button(ui, palette, self.confirm_label, self.role, ButtonVariant::Filled, None).clicked() {
                        outcome = PromptOutcome::Confirm;
                    }
                    if let Some(sec) = self.secondary_label {
                        if modern_button(ui, palette, sec, Role::Secondary, ButtonVariant::Outline, None).clicked() {
                            outcome = PromptOutcome::Secondary;
                        }
                    }
                    if modern_button(ui, palette, self.cancel_label, Role::Secondary, ButtonVariant::Ghost, None).clicked() {
                        outcome = PromptOutcome::Cancel;
                    }
                });
            });

        outcome
    }
}

/// A single non-blocking semantic toast notification.
#[derive(Clone, Debug)]
pub struct ToastItem {
    pub id: u64,
    pub title: String,
    pub message: String,
    pub role: Role,
    pub created_at: f64,
    pub duration_secs: f64,
}

/// Manages and paints an animated bottom-right stack of semantic Fluent toast notifications.
#[derive(Default)]
pub struct ToastStack {
    items: Vec<ToastItem>,
    next_id: u64,
}

impl ToastStack {
    pub fn push(&mut self, title: impl Into<String>, message: impl Into<String>, role: Role, now: f64) {
        self.push_with_duration(title, message, role, now, 3.6);
    }

    pub fn push_with_duration(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
        role: Role,
        now: f64,
        duration_secs: f64,
    ) {
        self.next_id += 1;
        self.items.push(ToastItem {
            id: self.next_id,
            title: title.into(),
            message: message.into(),
            role,
            created_at: now,
            duration_secs: duration_secs.max(0.8),
        });
        if self.items.len() > 5 {
            self.items.remove(0);
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn prune(&mut self, now: f64) {
        self.items.retain(|t| now - t.created_at < t.duration_secs);
    }

    /// Renders active toasts in the bottom-right corner of the viewport.
    pub fn show(&mut self, ctx: &Context, palette: &ThemePalette) {
        let now = ctx.input(|i| i.time);
        self.prune(now);
        if self.items.is_empty() {
            return;
        }
        ctx.request_repaint();

        let screen = ctx.content_rect();
        let toast_w = 310.0;
        let toast_h = 58.0;
        let gap = 10.0;
        let mut dismiss_id: Option<u64> = None;

        for (idx, item) in self.items.iter().enumerate() {
            let elapsed = (now - item.created_at).max(0.0) as f32;
            let remain_ratio = (1.0 - elapsed / item.duration_secs as f32).clamp(0.0, 1.0);
            let enter_t = anim::ease_out_cubic((elapsed / 0.18).clamp(0.0, 1.0));
            let slide_x = (1.0 - enter_t) * 24.0;

            let y = screen.max.y - 20.0 - (idx as f32 + 1.0) * (toast_h + gap) + gap;
            let x = screen.max.x - toast_w - 20.0 + slide_x;

            Area::new(Id::unique(("mr_toast", item.id)))
                .order(Order::Foreground)
                .fixed_pos(pos2(x, y))
                .show(ctx, |ui| {
                    let (rect, _) = ui.allocate_exact_size(vec2(toast_w, toast_h), Sense::hover());
                    let accent = palette.role_color(item.role);
                    let icon = match item.role {
                        Role::Success => Icon::Check,
                        Role::Warning => Icon::Warning,
                        Role::Danger => Icon::Close,
                        _ => Icon::Info,
                    };

                    // Soft drop shadow + elevated card
                    ui.painter().rect_filled(rect.translate(vec2(0.0, 3.0)), CornerRadius::same(palette.radius_md), Color32::from_black_alpha(38));
                    ui.painter().rect_filled(rect, CornerRadius::same(palette.radius_md), palette.surface);
                    ui.painter().rect_stroke(rect, CornerRadius::same(palette.radius_md), Stroke::new(1.0, palette.border), StrokeKind::Inside);

                    // Left semantic accent stripe
                    let stripe = Rect::from_min_size(pos2(rect.min.x + 1.0, rect.min.y + 8.0), vec2(3.5, rect.height() - 16.0));
                    ui.painter().rect_filled(stripe, 2.0, accent);

                    // Icon badge
                    let ibox = Rect::from_center_size(pos2(rect.min.x + 26.0, rect.center().y), vec2(26.0, 26.0));
                    ui.painter().rect_filled(ibox, 7.0, palette.role_subtle_bg(item.role));
                    icons::draw(ui.painter(), ibox.shrink(5.5), icon, accent, accent);

                    // Title + Message
                    let text_x = rect.min.x + 48.0;
                    if item.message.is_empty() {
                        ui.painter().text(
                            pos2(text_x, rect.center().y),
                            Align2::LEFT_CENTER,
                            &item.title,
                            FontId::proportional(13.0),
                            palette.fg,
                        );
                    } else {
                        ui.painter().text(
                            pos2(text_x, rect.min.y + 18.0),
                            Align2::LEFT_CENTER,
                            &item.title,
                            FontId::proportional(12.5),
                            palette.fg,
                        );
                        ui.painter().text(
                            pos2(text_x, rect.min.y + 37.0),
                            Align2::LEFT_CENTER,
                            &item.message,
                            FontId::proportional(11.5),
                            palette.fg_muted,
                        );
                    }

                    // Close button
                    let close_r = Rect::from_center_size(pos2(rect.max.x - 16.0, rect.min.y + 16.0), vec2(18.0, 18.0));
                    let close_resp = ui.interact(close_r, Id::unique(("mr_toast_x", item.id)), Sense::click());
                    let x_col = if close_resp.hovered() { palette.fg } else { palette.fg_subtle };
                    icons::draw(ui.painter(), close_r.shrink(4.0), Icon::Close, x_col, x_col);
                    if close_resp.clicked() {
                        dismiss_id = Some(item.id);
                    }

                    // Auto-dismiss countdown progress bar along bottom edge
                    let prog_w = (rect.width() - 16.0) * remain_ratio;
                    if prog_w > 1.0 {
                        let pbar = Rect::from_min_size(pos2(rect.min.x + 8.0, rect.max.y - 3.5), vec2(prog_w, 2.0));
                        ui.painter().rect_filled(pbar, 1.0, accent.gamma_multiply(0.75));
                    }
                });
        }

        if let Some(id) = dismiss_id {
            self.items.retain(|t| t.id != id);
        }
    }
}
