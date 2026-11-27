//! Smooth 60–120 FPS easing and stateful animation helpers for ModernRust.

use egui::{Context, Id};

/// Snappy Fluent ease-out cubic curve (`0.0 -> 1.0`).
#[inline]
pub fn ease_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let inv = 1.0 - t;
    1.0 - inv * inv * inv
}

/// Smooth ease-in-out cubic curve (`0.0 -> 1.0`).
#[inline]
pub fn ease_in_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) * 0.5
    }
}

/// Animates a boolean state (`false -> 0.0`, `true -> 1.0`) with ease-out cubic smoothing.
pub fn animate_bool(ctx: &Context, id: Id, on: bool, duration_secs: f32) -> f32 {
    let raw = ctx.animate_bool_with_time(id, on, duration_secs.max(0.01));
    ease_out_cubic(raw)
}

/// Animates a hover state (`110 ms` snappy Fluent hover transition).
pub fn animate_hover(ctx: &Context, id: Id, hovered: bool) -> f32 {
    animate_bool(ctx, id, hovered, 0.11)
}

/// Animates an overlay or menu opening (`130 ms` slide + fade).
pub fn animate_open(ctx: &Context, id: Id, open: bool) -> f32 {
    animate_bool(ctx, id, open, 0.13)
}

/// Animates a scalar position (such as a gliding segmented pill thumb or active indicator bar).
pub fn animate_scalar(ctx: &Context, id: Id, target: f32, duration_secs: f32) -> f32 {
    ctx.animate_value_with_time(id, target, duration_secs.max(0.01))
}
