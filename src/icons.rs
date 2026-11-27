//! Persistent `24×24` SVG icon engine for ModernRust backed by `resvg` + `tiny-skia`.
//!
//! Icons are auto-discovered from `assets/icons/**/*.svg` by `build.rs`, embedded directly into
//! the binary, dynamically recolored to match any active `ThemePalette` (`currentColor` -> fg,
//! `#0067C0` -> accent), rasterized with true subpixel anti-aliasing at exact HiDPI resolution,
//! and cached as GPU `egui::TextureHandle`s.

use egui::{pos2, vec2, Color32, ColorImage, Context, Id, Painter, Rect, TextureHandle, TextureOptions};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

include!(concat!(env!("OUT_DIR"), "/generated_icons.rs"));

type CacheKey = (Icon, u32, Color32, Color32);

#[derive(Clone, Default)]
struct IconTextureCache {
    map: Arc<Mutex<HashMap<CacheKey, TextureHandle>>>,
}

fn hex_color(c: Color32) -> String {
    format!("#{:02x}{:02x}{:02x}", c.r(), c.g(), c.b())
}

/// Rasterizes an SVG icon with dynamic `fg` (`currentColor`) and `accent` (`#0067C0`) recoloring
/// into a premultiplied RGBA `egui::ColorImage` of size `px x px`.
pub fn rasterize_svg(svg: &str, px: u32, fg: Color32, accent: Color32) -> Option<ColorImage> {
    let px = px.clamp(8, 256);
    let fg_hex = hex_color(fg);
    let acc_hex = hex_color(accent);
    let themed = svg
        .replace("currentColor", &fg_hex)
        .replace("#0067C0", &acc_hex)
        .replace("#0067c0", &acc_hex);

    let opts = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_str(&themed, &opts).ok()?;
    let sz = tree.size();
    let sx = (px as f32) / sz.width().max(1.0);
    let sy = (px as f32) / sz.height().max(1.0);

    let mut pixmap = resvg::tiny_skia::Pixmap::new(px, px)?;
    let tf = resvg::tiny_skia::Transform::from_scale(sx, sy);
    resvg::render(&tree, tf, &mut pixmap.as_mut());

    Some(ColorImage::from_rgba_premultiplied(
        [px as usize, px as usize],
        pixmap.data(),
    ))
}

fn get_or_load_texture(
    ctx: &Context,
    icon: Icon,
    px: u32,
    fg: Color32,
    accent: Color32,
) -> Option<TextureHandle> {
    let cache_id = Id::unique("modernrust_svg_icon_cache");
    let cache = ctx.data_mut(|d| {
        d.get_temp_mut_or_default::<IconTextureCache>(cache_id).clone()
    });
    let key: CacheKey = (icon, px, fg, accent);
    if let Ok(guard) = cache.map.lock() {
        if let Some(tex) = guard.get(&key) {
            return Some(tex.clone());
        }
    }
    let img = rasterize_svg(icon.svg_source(), px, fg, accent)?;
    let tex = ctx.load_texture(
        format!("mr_icon_{:?}_{}_{:02x}{:02x}{:02x}_{:02x}{:02x}{:02x}", icon, px, fg.r(), fg.g(), fg.b(), accent.r(), accent.g(), accent.b()),
        img,
        TextureOptions::LINEAR,
    );
    if let Ok(mut guard) = cache.map.lock() {
        if guard.len() > 512 {
            guard.clear();
        }
        guard.insert(key, tex.clone());
    }
    Some(tex)
}

/// Draws a ModernRust SVG icon centered inside `rect` with exact HiDPI subpixel anti-aliasing.
pub fn draw(painter: &Painter, rect: Rect, icon: Icon, color: Color32, accent: Color32) {
    let ctx = painter.ctx();
    let s = rect.width().min(rect.height()).max(8.0);
    let px = (s * ctx.pixels_per_point()).round().clamp(12.0, 256.0) as u32;
    let draw_rect = Rect::from_center_size(rect.center(), vec2(s, s));
    if let Some(tex) = get_or_load_texture(ctx, icon, px, color, accent) {
        painter.image(
            tex.id(),
            draw_rect,
            Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
            Color32::WHITE,
        );
    }
}

/// Convenience helper: allocates a square `size x size` region and paints `icon`.
pub fn widget(ui: &mut egui::Ui, icon: Icon, size: f32, color: Color32, accent: Color32) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(size, size), egui::Sense::hover());
    draw(ui.painter(), rect, icon, color, accent);
    resp
}
