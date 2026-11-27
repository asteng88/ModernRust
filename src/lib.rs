//! **ModernRust** — Windows 11 Fluent & semantic UI component library for `egui`,
//! inspired by `ModernTtk`.

pub mod anim;
pub mod icons;
pub mod menu;
pub mod panel;
pub mod prompt;
pub mod theme;
pub mod widgets;

pub use icons::Icon;
pub use menu::{menu_button, show_menu_body, MenuEntry};
pub use panel::{edge_collapse_tab, section_card, side_panel_header};
pub use prompt::{PromptBox, PromptOutcome, ToastItem, ToastStack};
pub use theme::{Role, ThemePalette, ThemePreset};
pub use widgets::{chip, fluent_switch, modern_button, progress_bar, segmented_pill, ButtonVariant};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_contrast_and_roles_are_valid() {
        for &preset in ThemePreset::ALL {
            let p = preset.palette();
            assert!(!p.name.is_empty());
            let fg_primary = p.role_text(Role::Primary);
            assert_ne!(fg_primary, p.role_color(Role::Primary));
            let _ = p.role_hover(Role::Success);
            let _ = p.role_active(Role::Danger);
            let _ = p.role_subtle_bg(Role::Warning);
        }
    }

    #[test]
    fn animation_curves_respect_unit_bounds() {
        assert_eq!(anim::ease_out_cubic(0.0), 0.0);
        assert_eq!(anim::ease_out_cubic(1.0), 1.0);
        assert!(anim::ease_out_cubic(0.5) > 0.5);
        assert_eq!(anim::ease_in_out_cubic(0.0), 0.0);
        assert_eq!(anim::ease_in_out_cubic(1.0), 1.0);
    }

    #[test]
    fn toast_stack_caps_and_prunes_expired() {
        let mut stack = ToastStack::default();
        for i in 0..7 {
            stack.push_with_duration(format!("T{i}"), "msg", Role::Info, i as f64, 3.0);
        }
        assert_eq!(stack.len(), 5);
        stack.prune(20.0);
        assert!(stack.is_empty());
    }

    #[test]
    fn all_discovered_svg_icons_rasterize_cleanly() {
        assert!(Icon::ALL.len() >= 41, "expected at least 41 icons, got {}", Icon::ALL.len());
        let p = ThemePreset::FluentLight.palette();
        for &ic in Icon::ALL {
            let img = icons::rasterize_svg(ic.svg_source(), 32, p.fg, p.primary)
                .unwrap_or_else(|| panic!("SVG icon {:?} failed to rasterize", ic));
            assert_eq!(img.size, [32, 32]);
            assert!(
                img.pixels.iter().any(|px| px.a() > 0),
                "SVG icon {:?} rendered completely blank",
                ic
            );
        }
    }
}
