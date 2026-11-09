//! Embedded theme assets: the binary carries everything a site needs, so
//! building requires no Node.js, no network and no files beside the
//! executable. `md.css` supplies Markdown typography and its light/dark
//! palettes; `styles.css` supplies the site chrome.

/// Output-relative directory theme assets are written to.
pub const ASSET_DIR: &str = "assets";

pub const MD_CSS: &str = include_str!("../assets/md.css");
pub const STYLES_CSS: &str = include_str!("../assets/styles.css");
pub const SCRIPT_JS: &str = include_str!("../assets/script.js");
pub const FAVICON_SVG: &str = include_str!("../assets/favicon.svg");
pub const FAVICON_32_PNG: &[u8] = include_bytes!("../assets/favicon-32.png");
pub const FAVICON_64_PNG: &[u8] = include_bytes!("../assets/favicon-64.png");
pub const APPLE_TOUCH_PNG: &[u8] = include_bytes!("../assets/apple-touch-icon.png");
pub const LOGO_PNG: &[u8] = include_bytes!("../assets/logo.png");
pub const PAGE_TEMPLATE: &str = include_str!("../assets/page.html");

/// All theme assets as (file name, bytes) pairs, written under `assets/`.
pub fn asset_files() -> Vec<(&'static str, &'static [u8])> {
    vec![
        ("md.css", MD_CSS.as_bytes()),
        ("styles.css", STYLES_CSS.as_bytes()),
        ("script.js", SCRIPT_JS.as_bytes()),
        ("favicon.svg", FAVICON_SVG.as_bytes()),
        ("favicon-32.png", FAVICON_32_PNG),
        ("favicon-64.png", FAVICON_64_PNG),
        ("apple-touch-icon.png", APPLE_TOUCH_PNG),
        ("logo.png", LOGO_PNG),
    ]
}
