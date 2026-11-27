# ModernRust Persistent Icon Set Specification

Every `.svg` file placed inside `assets/icons/<category>/<snake_name>.svg` is **automatically discovered** by `build.rs` and compiled into `modernrust::Icon` (embedded directly in your binary — zero external files at runtime).

## Design Rules (for Visual Consistency Across All Apps)

1. **Grid & ViewBox:** `viewBox="0 0 24 24"`, with `2px` optical padding (content inside `2..22`).
2. **Stroke Style:** `stroke-width="1.75"`, `stroke-linecap="round"`, `stroke-linejoin="round"`, `fill="none"` by default.
3. **Dynamic Theme Tokens:**
   - Use **`currentColor`** for primary strokes/fills — automatically recolored to the active theme's foreground (`palette.fg` or button text color).
   - Use **`#0067C0`** for accent strokes and duotone fills (e.g. `fill="#0067C0" fill-opacity="0.16"`) — automatically recolored to the active theme's `accent` (`palette.primary` or semantic `Role`).
4. **Adding New Icons:**
   - Drop any conforming SVG into `assets/icons/core/`, `assets/icons/editorial/`, `assets/icons/data/`, etc.
   - Example: `assets/icons/editorial/highlighter.svg` automatically becomes `modernrust::Icon::Highlighter` with `category() == "editorial"`.
