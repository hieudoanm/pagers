//! pagers-build: the build pipeline — discover → render → write pages →
//! copy assets → emit the link graph. One function turns a content directory
//! into a deployable static site.

use std::collections::HashSet;
use std::path::Path;

use pagers_markdown as markdown;
use pagers_markdown::Rendered;
mod meta;
use pagers_site::config::Config;
use pagers_site::nav;
use pagers_site::resolve::Resolver;
use pagers_site::site::{self, Content, Page};
use pagers_theme::template::{self, View};
use pagers_theme::theme;

/// What a build produced, for the CLI summary line.
pub struct Stats {
    pub pages: usize,
    pub assets: usize,
}

/// Builds the site described by `content_dir` into `output_dir`, replacing
/// any previous output so deleted pages never survive a rebuild.
pub fn build(content_dir: &str, output_dir: &str) -> Result<Stats, String> {
    let mut content = site::discover(content_dir)?;
    content.pages.retain(|p| !p.draft);
    if content.pages.is_empty() {
        return Err(format!(
            "no publishable Markdown files found in {content_dir}"
        ));
    }

    let config = pagers_site::config::load(content_dir, content.index_frontmatter.as_ref())?;
    let rendered = render_all(&content, &config)?;

    clear_output(output_dir)?;

    for (page, doc) in content.pages.iter_mut().zip(rendered.iter()) {
        page.adopt_heading_title(doc.title.as_deref().unwrap_or(""));
    }
    let nav = nav::build(&content.pages);

    write_pages(output_dir, &config, &content.pages, &rendered, &nav)?;
    if !content.has_landing_page() {
        write_landing(output_dir, &config, &content.pages, &nav)?;
    }
    let assets = write_assets(output_dir, &content)?;
    meta::write_meta_files(
        output_dir,
        content_dir,
        &config,
        &content.pages,
        !content.has_landing_page(),
        &content,
    )?;

    let edges: Vec<markdown::Edge> = rendered.iter().flat_map(|r| r.edges.clone()).collect();
    let mut graph = pagers_graph::from_pages(&content.pages, &edges);
    pagers_graph::apply_base_path(&mut graph, &config);
    let graph_title = format!("{} — Graph", config.title);
    let graph_html = pagers_graph::render_html(&graph, &graph_title, &config.theme);
    write_file(output_dir, "graph.html", graph_html.as_bytes())?;

    Ok(Stats {
        pages: content.pages.len(),
        assets,
    })
}

/// Renders every page's Markdown, resolving links against the site.
fn render_all(content: &Content, config: &Config) -> Result<Vec<Rendered>, String> {
    let by_source = site::index_by_source(&content.pages);
    let page_urls: Vec<String> = content.pages.iter().map(|p| p.url.clone()).collect();
    let page_sources: Vec<String> = content.pages.iter().map(|p| p.source.clone()).collect();
    let asset_set: HashSet<String> = content.assets.iter().map(|(s, _)| s.clone()).collect();
    let prefix = |url: &str| config.url(url);

    let mut rendered = Vec::with_capacity(content.pages.len());
    for page in &content.pages {
        let resolver = Resolver::new(
            &page.dir,
            &by_source,
            &page_urls,
            &page_sources,
            &asset_set,
            &prefix,
        );
        rendered.push(markdown::render(
            &page.source,
            &page.content,
            Some(&resolver),
        ));
    }
    Ok(rendered)
}

/// Writes every page through the template.
fn write_pages(
    output_dir: &str,
    config: &Config,
    pages: &[Page],
    rendered: &[Rendered],
    nav: &nav::Nav,
) -> Result<(), String> {
    for (page, doc) in pages.iter().zip(rendered.iter()) {
        let html = template::render_page(&View {
            config,
            title: &page.title,
            description: page.description.as_deref(),
            body_html: &doc.html,
            outline: &doc.outline,
            nav: Some(nav),
            pages,
            current_url: &page.url,
            is_landing: false,
        });
        write_file(output_dir, &page.output, html.as_bytes())?;
    }
    Ok(())
}

/// Writes the generated landing page for content with no `index.md`/`README.md`.
fn write_landing(
    output_dir: &str,
    config: &Config,
    pages: &[Page],
    nav: &nav::Nav,
) -> Result<(), String> {
    let html = template::render_page(&View {
        config,
        title: &config.title,
        description: None,
        body_html: "",
        outline: &[],
        nav: Some(nav),
        pages,
        current_url: "/",
        is_landing: true,
    });
    write_file(output_dir, "index.html", html.as_bytes())
}

/// Copies theme assets under `assets/` and content assets to their
/// content-relative paths at the output root. Returns files written.
fn write_assets(output_dir: &str, content: &Content) -> Result<usize, String> {
    let mut count = 0;
    for (name, bytes) in theme::asset_files() {
        let path = format!("{}/{name}", theme::ASSET_DIR);
        write_file(output_dir, &path, bytes)?;
        count += 1;
    }
    for (source, abs) in &content.assets {
        let bytes = std::fs::read(abs).map_err(|e| format!("read asset {source}: {e}"))?;
        write_file(output_dir, source, &bytes)?;
        count += 1;
    }
    Ok(count)
}

/// Removes a previous build so deleted pages do not survive, then recreates
/// the output directory.
fn clear_output(output_dir: &str) -> Result<(), String> {
    let path = Path::new(output_dir);
    if path.exists() {
        std::fs::remove_dir_all(path)
            .map_err(|e| format!("clear output directory {output_dir}: {e}"))?;
    }
    std::fs::create_dir_all(path).map_err(|e| format!("create output directory {output_dir}: {e}"))
}

/// Writes `data` to `name` relative to `output_dir`, creating parents.
pub(crate) fn write_file(output_dir: &str, name: &str, data: &[u8]) -> Result<(), String> {
    let target = Path::new(output_dir).join(name);
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("create directory for {name}: {e}"))?;
    }
    std::fs::write(&target, data).map_err(|e| format!("write {name}: {e}"))
}
