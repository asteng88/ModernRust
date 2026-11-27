//! Theme palettes and semantic color roles for ModernRust.
//!
//! Combines Windows 11 Fluent Design surface/elevation tokens with Bootstrap semantic
//! color roles (`Primary`, `Secondary`, `Success`, `Info`, `Warning`, `Danger`, `Light`, `Dark`).

use egui::{Color32, CornerRadius, Stroke, Visuals};

/// Semantic color role for buttons, badges, toasts, alerts, and indicators.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
pub enum Role {
    #[default]
    Primary,
    Secondary,
    Success,
    Info,
    Warning,
    Danger,
    Light,
    Dark,
}

/// Built-in ModernRust theme presets (matching ModernTtk's palette suite + ObsidianPaper).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
pub enum ThemePreset {
    #[default]
    FluentLight,
    FluentDark,
    Cosmo,
    Darkly,
    Cyborg,
    Minty,
    Dracula,
    ObsidianPaper,
}

impl ThemePreset {
    pub const ALL: &'static [Self] = &[
        Self::FluentLight,
        Self::FluentDark,
        Self::Cosmo,
        Self::Darkly,
        Self::Cyborg,
        Self::Minty,
        Self::Dracula,
        Self::ObsidianPaper,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::FluentLight => "Fluent Light",
            Self::FluentDark => "Fluent Dark",
            Self::Cosmo => "Cosmo",
            Self::Darkly => "Darkly",
            Self::Cyborg => "Cyborg",
            Self::Minty => "Minty",
            Self::Dracula => "Dracula",
            Self::ObsidianPaper => "Obsidian Paper",
        }
    }

    pub fn palette(self) -> ThemePalette {
        ThemePalette::from_preset(self)
    }
}

/// Complete Fluent + semantic color token set for a ModernRust theme.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThemePalette {
    pub name: &'static str,
    pub dark: bool,

    // Surface & Elevation
    pub bg: Color32,
    pub surface: Color32,
    pub surface_alt: Color32,
    pub surface_hover: Color32,
    pub surface_active: Color32,

    // Typography
    pub fg: Color32,
    pub fg_muted: Color32,
    pub fg_subtle: Color32,

    // Borders & Dividers
    pub border: Color32,
    pub border_strong: Color32,
    pub divider: Color32,

    // Inputs
    pub input_bg: Color32,
    pub input_border: Color32,

    // Semantic roles
    pub primary: Color32,
    pub secondary: Color32,
    pub success: Color32,
    pub info: Color32,
    pub warning: Color32,
    pub danger: Color32,
    pub light: Color32,
    pub dark_role: Color32,

    // Geometry
    pub radius_sm: u8,
    pub radius_md: u8,
    pub radius_lg: u8,
}

pub const fn rgb(r: u8, g: u8, b: u8) -> Color32 {
    Color32::from_rgb(r, g, b)
}

/// Linearly blends `a` toward `b` by `t` in `[0.0, 1.0]`.
pub fn blend(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let inv = 1.0 - t;
    Color32::from_rgba_unmultiplied(
        (a.r() as f32 * inv + b.r() as f32 * t).round() as u8,
        (a.g() as f32 * inv + b.g() as f32 * t).round() as u8,
        (a.b() as f32 * inv + b.b() as f32 * t).round() as u8,
        (a.a() as f32 * inv + b.a() as f32 * t).round() as u8,
    )
}

/// Adjusts brightness by `delta` (`> 0` lightens toward white, `< 0` darkens toward black).
pub fn adjust_brightness(c: Color32, delta: f32) -> Color32 {
    if delta >= 0.0 {
        blend(c, Color32::WHITE, delta)
    } else {
        blend(c, Color32::BLACK, -delta)
    }
}

/// Returns crisp dark or white foreground text based on perceived luminance.
pub fn contrast_text(bg: Color32) -> Color32 {
    let lum = 0.299 * (bg.r() as f32) + 0.587 * (bg.g() as f32) + 0.114 * (bg.b() as f32);
    if lum > 152.0 {
        rgb(15, 23, 42)
    } else {
        Color32::WHITE
    }
}

impl ThemePalette {
    pub fn from_preset(preset: ThemePreset) -> Self {
        match preset {
            ThemePreset::FluentLight => Self {
                name: "Fluent Light",
                dark: false,
                bg: rgb(243, 243, 243),
                surface: rgb(251, 251, 251),
                surface_alt: rgb(240, 240, 240),
                surface_hover: rgb(234, 234, 234),
                surface_active: rgb(224, 224, 224),
                fg: rgb(26, 26, 26),
                fg_muted: rgb(92, 92, 92),
                fg_subtle: rgb(156, 163, 175),
                border: rgb(214, 214, 214),
                border_strong: rgb(184, 184, 184),
                divider: rgb(229, 229, 229),
                input_bg: rgb(255, 255, 255),
                input_border: rgb(209, 213, 219),
                primary: rgb(0, 103, 192),
                secondary: rgb(92, 99, 106),
                success: rgb(21, 128, 61),
                info: rgb(2, 132, 199),
                warning: rgb(217, 119, 6),
                danger: rgb(220, 38, 38),
                light: rgb(248, 250, 252),
                dark_role: rgb(30, 41, 59),
                radius_sm: 6,
                radius_md: 8,
                radius_lg: 12,
            },
            ThemePreset::FluentDark => Self {
                name: "Fluent Dark",
                dark: true,
                bg: rgb(28, 28, 28),
                surface: rgb(39, 39, 39),
                surface_alt: rgb(50, 50, 50),
                surface_hover: rgb(56, 56, 56),
                surface_active: rgb(66, 66, 66),
                fg: rgb(245, 245, 245),
                fg_muted: rgb(163, 163, 163),
                fg_subtle: rgb(107, 114, 128),
                border: rgb(61, 61, 61),
                border_strong: rgb(82, 82, 82),
                divider: rgb(51, 51, 51),
                input_bg: rgb(36, 36, 36),
                input_border: rgb(69, 69, 69),
                primary: rgb(76, 194, 255),
                secondary: rgb(100, 116, 139),
                success: rgb(34, 197, 94),
                info: rgb(56, 189, 248),
                warning: rgb(245, 158, 11),
                danger: rgb(239, 68, 68),
                light: rgb(226, 232, 240),
                dark_role: rgb(15, 23, 42),
                radius_sm: 6,
                radius_md: 8,
                radius_lg: 12,
            },
            ThemePreset::Cosmo => Self {
                name: "Cosmo",
                dark: false,
                bg: rgb(248, 250, 252),
                surface: rgb(255, 255, 255),
                surface_alt: rgb(241, 245, 249),
                surface_hover: rgb(226, 232, 240),
                surface_active: rgb(203, 213, 225),
                fg: rgb(15, 23, 42),
                fg_muted: rgb(71, 85, 105),
                fg_subtle: rgb(148, 163, 184),
                border: rgb(226, 232, 240),
                border_strong: rgb(203, 213, 225),
                divider: rgb(241, 245, 249),
                input_bg: rgb(255, 255, 255),
                input_border: rgb(203, 213, 225),
                primary: rgb(37, 99, 235),
                secondary: rgb(71, 85, 105),
                success: rgb(22, 163, 74),
                info: rgb(8, 145, 178),
                warning: rgb(234, 88, 12),
                danger: rgb(225, 29, 72),
                light: rgb(248, 250, 252),
                dark_role: rgb(15, 23, 42),
                radius_sm: 6,
                radius_md: 8,
                radius_lg: 12,
            },
            ThemePreset::Darkly => Self {
                name: "Darkly",
                dark: true,
                bg: rgb(15, 23, 42),
                surface: rgb(30, 41, 59),
                surface_alt: rgb(39, 53, 73),
                surface_hover: rgb(51, 65, 85),
                surface_active: rgb(71, 85, 105),
                fg: rgb(248, 250, 252),
                fg_muted: rgb(148, 163, 184),
                fg_subtle: rgb(100, 116, 139),
                border: rgb(51, 65, 85),
                border_strong: rgb(71, 85, 105),
                divider: rgb(30, 41, 59),
                input_bg: rgb(22, 32, 50),
                input_border: rgb(51, 65, 85),
                primary: rgb(59, 130, 246),
                secondary: rgb(100, 116, 139),
                success: rgb(16, 185, 129),
                info: rgb(6, 182, 212),
                warning: rgb(245, 158, 11),
                danger: rgb(244, 63, 94),
                light: rgb(241, 245, 249),
                dark_role: rgb(9, 13, 22),
                radius_sm: 6,
                radius_md: 8,
                radius_lg: 12,
            },
            ThemePreset::Cyborg => Self {
                name: "Cyborg",
                dark: true,
                bg: rgb(9, 9, 11),
                surface: rgb(20, 20, 24),
                surface_alt: rgb(31, 31, 38),
                surface_hover: rgb(39, 39, 48),
                surface_active: rgb(50, 50, 62),
                fg: rgb(250, 250, 250),
                fg_muted: rgb(161, 161, 170),
                fg_subtle: rgb(82, 82, 91),
                border: rgb(39, 39, 42),
                border_strong: rgb(63, 63, 70),
                divider: rgb(31, 31, 35),
                input_bg: rgb(18, 18, 22),
                input_border: rgb(46, 46, 54),
                primary: rgb(6, 182, 212),
                secondary: rgb(113, 113, 122),
                success: rgb(34, 197, 94),
                info: rgb(139, 92, 246),
                warning: rgb(234, 179, 8),
                danger: rgb(244, 63, 94),
                light: rgb(228, 228, 231),
                dark_role: rgb(9, 9, 11),
                radius_sm: 6,
                radius_md: 8,
                radius_lg: 12,
            },
            ThemePreset::Minty => Self {
                name: "Minty",
                dark: false,
                bg: rgb(242, 251, 249),
                surface: rgb(255, 255, 255),
                surface_alt: rgb(230, 246, 242),
                surface_hover: rgb(213, 239, 233),
                surface_active: rgb(191, 229, 220),
                fg: rgb(19, 42, 38),
                fg_muted: rgb(73, 106, 99),
                fg_subtle: rgb(136, 168, 161),
                border: rgb(205, 229, 223),
                border_strong: rgb(173, 207, 199),
                divider: rgb(227, 242, 238),
                input_bg: rgb(255, 255, 255),
                input_border: rgb(194, 223, 216),
                primary: rgb(13, 148, 136),
                secondary: rgb(244, 63, 94),
                success: rgb(22, 163, 74),
                info: rgb(2, 132, 199),
                warning: rgb(245, 158, 11),
                danger: rgb(239, 68, 68),
                light: rgb(240, 253, 250),
                dark_role: rgb(19, 78, 74),
                radius_sm: 6,
                radius_md: 8,
                radius_lg: 12,
            },
            ThemePreset::Dracula => Self {
                name: "Dracula",
                dark: true,
                bg: rgb(40, 42, 54),
                surface: rgb(48, 51, 66),
                surface_alt: rgb(58, 61, 78),
                surface_hover: rgb(68, 71, 90),
                surface_active: rgb(80, 84, 108),
                fg: rgb(248, 248, 242),
                fg_muted: rgb(189, 195, 214),
                fg_subtle: rgb(98, 114, 164),
                border: rgb(68, 71, 90),
                border_strong: rgb(98, 114, 164),
                divider: rgb(56, 59, 76),
                input_bg: rgb(36, 38, 49),
                input_border: rgb(68, 71, 90),
                primary: rgb(189, 147, 249),
                secondary: rgb(98, 114, 164),
                success: rgb(80, 250, 123),
                info: rgb(139, 233, 253),
                warning: rgb(255, 184, 108),
                danger: rgb(255, 85, 85),
                light: rgb(248, 248, 242),
                dark_role: rgb(33, 34, 44),
                radius_sm: 6,
                radius_md: 8,
                radius_lg: 12,
            },
            ThemePreset::ObsidianPaper => Self {
                name: "Obsidian Paper",
                dark: true,
                bg: rgb(22, 24, 29),
                surface: rgb(31, 34, 41),
                surface_alt: rgb(40, 44, 54),
                surface_hover: rgb(49, 54, 66),
                surface_active: rgb(60, 66, 82),
                fg: rgb(242, 244, 248),
                fg_muted: rgb(156, 165, 180),
                fg_subtle: rgb(104, 113, 130),
                border: rgb(54, 59, 72),
                border_strong: rgb(78, 86, 104),
                divider: rgb(44, 48, 58),
                input_bg: rgb(26, 29, 35),
                input_border: rgb(58, 64, 78),
                primary: rgb(59, 130, 246),
                secondary: rgb(107, 114, 128),
                success: rgb(34, 197, 94),
                info: rgb(56, 189, 248),
                warning: rgb(245, 158, 11),
                danger: rgb(239, 68, 68),
                light: rgb(241, 245, 249),
                dark_role: rgb(15, 17, 21),
                radius_sm: 6,
                radius_md: 8,
                radius_lg: 12,
            },
        }
    }

    pub fn role_color(&self, role: Role) -> Color32 {
        match role {
            Role::Primary => self.primary,
            Role::Secondary => self.secondary,
            Role::Success => self.success,
            Role::Info => self.info,
            Role::Warning => self.warning,
            Role::Danger => self.danger,
            Role::Light => self.light,
            Role::Dark => self.dark_role,
        }
    }

    pub fn role_hover(&self, role: Role) -> Color32 {
        let base = self.role_color(role);
        if self.dark {
            adjust_brightness(base, 0.14)
        } else {
            adjust_brightness(base, -0.10)
        }
    }

    pub fn role_active(&self, role: Role) -> Color32 {
        let base = self.role_color(role);
        if self.dark {
            adjust_brightness(base, -0.08)
        } else {
            adjust_brightness(base, -0.20)
        }
    }

    pub fn role_subtle_bg(&self, role: Role) -> Color32 {
        blend(self.surface, self.role_color(role), if self.dark { 0.18 } else { 0.12 })
    }

    pub fn role_text(&self, role: Role) -> Color32 {
        match role {
            Role::Light => rgb(15, 23, 42),
            Role::Dark => rgb(248, 250, 252),
            _ => contrast_text(self.role_color(role)),
        }
    }

    /// Applies this ModernRust palette to an `egui::Context`'s global visuals.
    pub fn apply_to_egui(&self, ctx: &egui::Context) {
        let mut v = if self.dark { Visuals::dark() } else { Visuals::light() };
        v.panel_fill = self.surface;
        v.window_fill = self.surface;
        v.faint_bg_color = self.surface_alt;
        v.extreme_bg_color = self.input_bg;
        v.window_stroke = Stroke::new(1.0, self.border);
        v.window_corner_radius = CornerRadius::same(self.radius_lg);
        v.menu_corner_radius = CornerRadius::same(self.radius_md);
        v.selection.bg_fill = self.role_subtle_bg(Role::Primary);
        v.selection.stroke = Stroke::new(1.0, self.primary);
        v.widgets.noninteractive.bg_fill = self.surface;
        v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, self.fg);
        v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, self.divider);
        v.widgets.inactive.bg_fill = self.surface_alt;
        v.widgets.inactive.fg_stroke = Stroke::new(1.0, self.fg);
        v.widgets.inactive.bg_stroke = Stroke::new(1.0, self.border);
        v.widgets.inactive.corner_radius = CornerRadius::same(self.radius_sm);
        v.widgets.hovered.bg_fill = self.surface_hover;
        v.widgets.hovered.fg_stroke = Stroke::new(1.0, self.fg);
        v.widgets.hovered.bg_stroke = Stroke::new(1.0, self.primary);
        v.widgets.hovered.corner_radius = CornerRadius::same(self.radius_sm);
        v.widgets.active.bg_fill = self.surface_active;
        v.widgets.active.fg_stroke = Stroke::new(1.0, self.fg);
        v.widgets.active.bg_stroke = Stroke::new(1.5, self.primary);
        v.widgets.active.corner_radius = CornerRadius::same(self.radius_sm);
        ctx.set_visuals(v);
    }
}
