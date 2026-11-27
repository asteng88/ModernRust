# Contributing to ModernRust

Thank you for considering contributing to **ModernRust**! Whether you are fixing a bug, adding a new Fluent widget, proposing a theme preset, or contributing SVG icons, we appreciate your help.

## Getting Started

1. Fork the repository and clone your fork:
   ```bash
   git clone https://github.com/<your-username>/ModernRust.git
   cd ModernRust
   ```
2. Run the test suite to verify your environment:
   ```bash
   cargo test
   ```
3. Launch the interactive showcase app to preview components and themes live:
   ```bash
   cargo run --example showcase
   ```
   You can also start directly in a specific theme preset via `MODERNRUST_THEME`:
   ```bash
   MODERNRUST_THEME=dracula cargo run --example showcase
   ```

## Adding New SVG Icons

ModernRust automatically discovers every `.svg` file under `assets/icons/<category>/<snake_name>.svg` at compile time via `build.rs` and generates the `modernrust::Icon` enum.

Please follow the design specification in [assets/icons/ICON_SPEC.md](assets/icons/ICON_SPEC.md):
- **ViewBox:** `0 0 24 24` with `2px` optical padding (content inside `2..22`).
- **Stroke Style:** `stroke-width="1.75"`, `stroke-linecap="round"`, `stroke-linejoin="round"`, `fill="none"` by default.
- **Dynamic Theme Tokens:**
  - Use `currentColor` for primary strokes/fills (recolored to the active foreground).
  - Use `#0067C0` for accent strokes and subtle duotone fills (recolored to the active accent/role).
- Run `cargo test` to verify your new icon rasterizes cleanly via `all_discovered_svg_icons_rasterize_cleanly`.

## Adding Widgets or Theme Presets

- **Theme Presets:** Ensure contrast between `role_color` and `role_text` is readable in all semantic `Role` variants (`Primary`, `Secondary`, `Success`, `Info`, `Warning`, `Danger`, `Light`, `Dark`).
- **Widgets & Animations:** Use helpers from `crate::anim` (`animate_hover`, `animate_open`, `animate_scalar`, `ease_out_cubic`) to keep motion consistent across the library.
- **Showcase:** Update `examples/showcase.rs` when introducing new widgets or visual variants so reviewers can inspect them interactively.

## Code Quality & Pull Request Checklist

Before opening a pull request, please make sure:
- [ ] `cargo test` passes with zero failures.
- [ ] `cargo check --examples` compiles cleanly.
- [ ] `cargo fmt --check` passes.
- [ ] `cargo clippy --all-targets` has no new warnings.
- [ ] New public APIs include concise rustdoc comments.
- [ ] Screenshots or GIFs are attached for visual changes.

## Code of Conduct

All participants are expected to uphold our [Code of Conduct](CODE_OF_CONDUCT.md).
