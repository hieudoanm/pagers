# pagers-theme

The embedded presentation layer: the compiled `md.css` stylesheet, site
chrome (`styles.css`, `script.js`, `favicon.svg`), the `page.html` Handlebars-
style template, and `render_page()` which fills it per page.

Part of the [pagers](../../README.md) workspace; see
[Theming](../../packages/landing/docs/theming.md).

## API

```rust
pub mod theme;    // asset constants + asset_files()
pub mod template; // View<'a>, render_page(), escape()

// theme.rs — everything embedded at compile time:
pub const MD_CSS: &str = include_str!("../assets/md.css");
pub const STYLES_CSS / SCRIPT_JS / FAVICON_SVG / PAGE_TEMPLATE: &str
pub fn asset_files() -> Vec<(&'static str, &'static [u8])>

// template.rs
pub struct View<'a> { /* title, html, nav, outline, config, page meta… */ }
pub fn render_page(view: &View) -> String
pub fn escape(text: &str) -> String
```

## Assets

`assets/` is embedded via `include_str!` — no runtime file lookups. `md.css`
is a **compiled copy** of `packages/md.scss` (`pnpm run build` there, then
copy `dist/md.css` → `crates/pagers-theme/assets/md.css`).
