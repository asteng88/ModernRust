# Changelog

All notable changes to **ModernRust** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-11-26

### Added
- **Theme Engine (`theme`):** 8 built-in theme presets (`FluentLight`, `FluentDark`, `Cosmo`, `Darkly`, `Cyborg`, `Minty`, `Dracula`, `ObsidianPaper`) and 8 semantic color roles (`Primary`, `Secondary`, `Success`, `Info`, `Warning`, `Danger`, `Light`, `Dark`) with automatic `egui` visual styling.
- **Persistent SVG Icon Engine (`icons`):** Compile-time auto-discovery of 65 embedded `24×24` SVG icons (`core` and `editorial` sets) via `build.rs`, dynamic `currentColor` + accent (`#0067C0`) recoloring, `resvg`/`tiny-skia` HiDPI rasterization, and GPU `TextureHandle` caching.
- **Fluent Widgets (`widgets`):** `modern_button` (`Filled`, `Outline`, `Ghost`, `Pill`), animated gliding `segmented_pill`, `fluent_switch`, selectable `chip`, and semantic `progress_bar`.
- **Animated Menus (`menu`):** `menu_button` and `show_menu_body` with slide-fade entrance, Fluent accent indicator pill, icon slots, check items, separators, section headers, and shortcut badges.
- **Panels & Cards (`panel`):** `side_panel_header`, elevated `section_card`, and docked `edge_collapse_tab`.
- **Prompts & Toasts (`prompt`):** `PromptBox` confirmation cards and auto-expiring `ToastStack` notifications.
- **Animation Helpers (`anim`):** Ease-out/ease-in-out cubic curves and stateful hover, open, and scalar transitions.
- **Interactive Showcase (`examples/showcase.rs`):** Live demo of all themes, widgets, menus, prompts, toasts, and SVG icons.
