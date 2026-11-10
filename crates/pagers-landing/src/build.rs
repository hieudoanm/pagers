//! Build pipeline: load → validate → render → write. Port of `build.go`.

use crate::color;
use crate::config::{Config, OpenGraph};
use crate::load;
use crate::og;
use crate::validate;
use askama::Template;
use std::path::{Path, PathBuf};

/// Errors from the build pipeline.
#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error(transparent)]
    Load(#[from] load::LoadError),
    #[error("unknown theme {0:?} (available: {1})")]
    UnknownTheme(String, String),
    #[error("{path} is invalid:\n  - {}", .problems.join("\n  - "))]
    Invalid { path: String, problems: Vec<String> },
    #[error("theme: {0}")]
    Color(#[from] color::ColorError),
    #[error("template: {0}")]
    Template(#[from] askama::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("og card: {0}")]
    Og(String),
}

/// Shared template context for every page layout.
pub struct Page<'a> {
    pub config: &'a Config,
    pub og: OpenGraph,
    pub theme_css: String,
}

macro_rules! page_template {
    ($name:ident, $path:literal) => {
        #[derive(Template)]
        #[template(path = $path)]
        pub struct $name<'a> {
            pub config: &'a Config,
            pub og: OpenGraph,
            pub theme_css: String,
        }
    };
}

page_template!(ProductPage, "template_product.html");
page_template!(WaitlistPage, "template_waitlist.html");
page_template!(EventPage, "template_event.html");
page_template!(DownloadPage, "template_download.html");
page_template!(AppPage, "template_app.html");
page_template!(PricingPage, "template_pricing.html");
page_template!(PortfolioPage, "template_portfolio.html");
page_template!(DocsPage, "template_docs.html");
page_template!(FaqPage, "template_faq.html");
page_template!(TeamPage, "template_team.html");
page_template!(StatusPage, "template_status.html");
page_template!(LinktreePage, "template_linktree.html");

/// Askama wrapper for the OG card (templates/og_card.html).
#[derive(Template)]
#[template(path = "og_card.html")]
pub struct OgCardTemplate<'a> {
    pub card: &'a og::OgCard,
}

/// What a build wrote.
#[derive(Debug)]
pub struct BuildResult {
    pub page: PathBuf,
    pub card: Option<PathBuf>,
    pub meta: Vec<PathBuf>,
}

/// `render` returns the landing page HTML for `cfg`. Unknown types fall back
/// to product so hand-built configs still render.
pub fn render(cfg: &Config) -> Result<String, BuildError> {
    let kind = validate::normalize_type(&cfg.page_type);
    let kind = if validate::is_known_type(kind) {
        kind
    } else {
        "product"
    };
    let tokens = color::tokens(&cfg.theme)?;
    let theme_css = theme_css(&tokens);
    let og_meta = cfg.social();
    macro_rules! page {
        ($t:ident) => {
            $t {
                config: cfg,
                og: og_meta.clone(),
                theme_css: theme_css.clone(),
            }
            .render()?
        };
    }
    Ok(match kind {
        "product" => page!(ProductPage),
        "waitlist" => page!(WaitlistPage),
        "event" => page!(EventPage),
        "download" => page!(DownloadPage),
        "app" => page!(AppPage),
        "pricing" => page!(PricingPage),
        "portfolio" => page!(PortfolioPage),
        "docs" => page!(DocsPage),
        "faq" => page!(FaqPage),
        "team" => page!(TeamPage),
        "status" => page!(StatusPage),
        "linktree" => page!(LinktreePage),
        _ => unreachable!(),
    })
}

/// `build_file` loads `path`, validates it, renders the page, and writes it
/// to `output` (creating parents). A config with `site.og` also gets its
/// 1200×630 card written beside the page. `theme_name` overrides the YAML
/// theme with a built-in preset when non-empty.
pub fn build_file(path: &Path, output: &Path, theme_name: &str) -> Result<BuildResult, BuildError> {
    let cfg = load::render_config(path, theme_name)?;
    let html = render(&cfg)?;
    if let Some(dir) = output.parent() {
        if !dir.as_os_str().is_empty() {
            std::fs::create_dir_all(dir)?;
        }
    }
    std::fs::write(output, html)?;
    let mut result = BuildResult {
        page: output.to_path_buf(),
        card: None,
        meta: Vec::new(),
    };
    if cfg.site.open_graph.configured() {
        write_og_card(&cfg, output)?;
        result.card = Some(og::og_card_path(output));
    }
    result.meta = crate::meta::write_meta(&cfg, output)
        .map_err(BuildError::Io)?
        .files;
    write_styles(output, &mut result.meta)?;
    Ok(result)
}

/// Emit the shared `styles.css` beside the page unless one already exists —
/// a hand-tuned stylesheet always wins, like the metadata files.
fn write_styles(output: &Path, meta: &mut Vec<PathBuf>) -> Result<(), BuildError> {
    let path = output
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("styles.css");
    if path.exists() {
        return Ok(());
    }
    std::fs::write(&path, STYLES_CSS)?;
    meta.push(path);
    Ok(())
}

const STYLES_CSS: &str = include_str!("../styles.css");

/// `render_file` loads and renders without writing — the read-only
/// counterpart of `build_file`.
pub fn render_file(path: &Path, theme_name: &str) -> Result<String, BuildError> {
    let cfg = load::render_config(path, theme_name)?;
    render(&cfg)
}

/// Write the social card beside the page; `site.og.image` should point at a
/// PNG rasterized from it.
fn write_og_card(cfg: &Config, output: &Path) -> Result<(), BuildError> {
    let card = og::render_og(cfg)?;
    let path = og::og_card_path(output);
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(&path, card)?;
    Ok(())
}

/// `theme_css` renders the `:root` declarations as `--name: value;` lines in
/// sorted token order so output is stable across runs.
fn theme_css(tokens: &color::Tokens) -> String {
    // BTreeMap iterates in sorted key order already.
    let mut s = String::new();
    for (k, v) in tokens {
        s.push_str(&format!("        --{k}: {v};\n"));
    }
    s
}
