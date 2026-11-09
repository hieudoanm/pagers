//! Content discovery and the page model: every Markdown file below the
//! content root becomes a page published at a trailing-slash URL, everything
//! else becomes an asset copied through unchanged.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::frontmatter::{self, Frontmatter};

/// Directory names never descended into.
const SKIPPED_DIRS: &[&str] = &[".git", ".github", "node_modules", "vendor", "dist"];
/// File-name suffixes marking editor or sync leftovers.
const BACKUP_SUFFIXES: &[&str] = &["~", ".bak", ".backup", ".orig", ".swp", ".tmp"];
/// Files that configure the site rather than appear in it.
const PARTIAL_NAMES: &[&str] = &["pagers.yaml", "pagify.yaml"];
const MARKDOWN_EXTS: &[&str] = &[".markdown", ".md"];

/// One Markdown file resolved into everything the build needs.
#[derive(Debug)]
pub struct Page {
    /// Path relative to the content root, e.g. `guide/install.md`.
    pub source: String,
    /// Published URL, e.g. `/guide/install/`.
    pub url: String,
    /// HTML file written relative to the output dir.
    pub output: String,
    pub title: String,
    /// True when the title was derived and may still be adopted from a
    /// leading level-1 heading during rendering.
    pub title_from_heading: bool,
    pub description: Option<String>,
    pub label: Option<String>,
    pub order: Option<i64>,
    pub draft: bool,
    /// Directory holding the source file, `""` at the root.
    pub dir: String,
    /// Markdown body with frontmatter stripped.
    pub content: String,
}

impl Page {
    pub fn nav_title(&self) -> &str {
        self.label.as_deref().unwrap_or(&self.title)
    }

    /// Whether this page is its directory's landing page (`index.md` or
    /// `README.md`), which makes it a navigation group link rather than a leaf.
    pub fn is_index(&self) -> bool {
        let stem = trim_markdown_ext(&self.source);
        let base = stem.rsplit('/').next().unwrap_or("");
        base == "index" || base.eq_ignore_ascii_case("readme")
    }

    /// Adopts the document's leading level-1 heading as the title, unless the
    /// author already supplied one.
    pub fn adopt_heading_title(&mut self, heading: &str) {
        if !heading.is_empty() && self.title_from_heading {
            self.title = heading.to_string();
        }
    }
}

/// Everything discovered below the content root.
pub struct Content {
    pub pages: Vec<Page>,
    /// Non-Markdown files, as (path relative to root, absolute source path).
    pub assets: Vec<(String, PathBuf)>,
    /// Frontmatter of the root landing page, when it exists.
    pub index_frontmatter: Option<Frontmatter>,
}

impl Content {
    pub fn has_landing_page(&self) -> bool {
        self.pages.iter().any(|p| p.url == "/")
    }
}

/// Walks `root`, returning every page and asset below it. Hidden entries,
/// VCS directories, build output and editor leftovers are skipped; symlinks
/// are not followed.
pub fn discover(root: &str) -> Result<Content, String> {
    let root_path = Path::new(root);
    let meta =
        std::fs::metadata(root_path).map_err(|e| format!("read content directory {root}: {e}"))?;
    if !meta.is_dir() {
        return Err(format!("content path {root} is not a directory"));
    }

    let mut content = Content {
        pages: Vec::new(),
        assets: Vec::new(),
        index_frontmatter: None,
    };
    walk(root_path, root_path, &mut content)?;
    if content.pages.is_empty() {
        return Err(format!("no Markdown files found in {root}"));
    }
    sort_pages(&mut content.pages);
    Ok(content)
}

fn walk(root: &Path, dir: &Path, content: &mut Content) -> Result<(), String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("walk {}: {e}", dir.display()))?;
    let mut entries: Vec<_> = entries
        .collect::<Result<_, _>>()
        .map_err(|e| format!("walk {}: {e}", dir.display()))?;
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
        if is_dir {
            if name.starts_with('.') || SKIPPED_DIRS.contains(&name.as_str()) {
                continue;
            }
            walk(root, &path, content)?;
            continue;
        }
        if name.starts_with('.') || is_backup(&name) || is_partial(&name) {
            continue;
        }
        let Ok(rel) = path.strip_prefix(root) else {
            continue;
        };
        let source = rel.to_string_lossy().replace('\\', "/");
        if is_markdown(&source) {
            let raw = std::fs::read_to_string(&path).map_err(|e| format!("read {source}: {e}"))?;
            let page = page_from_source(&source, &raw)?;
            if page.url == "/" && content.index_frontmatter.is_none() {
                content.index_frontmatter = frontmatter_of(&raw);
            }
            content.pages.push(page);
        } else {
            content.assets.push((source, path));
        }
    }
    Ok(())
}

fn page_from_source(source: &str, raw: &str) -> Result<Page, String> {
    let (fm, body) =
        frontmatter::split(raw).map_err(|e| format!("parse frontmatter in {source}: {e}"))?;
    let fm = fm.unwrap_or_default();
    let fallback = title_from_source(source);
    Ok(Page {
        url: url_for_source(source),
        output: output_path_for_source(source),
        title: fm.title_or(&fallback),
        title_from_heading: fm.title.is_none(),
        description: fm.description.clone(),
        label: fm.label.clone(),
        order: fm.order,
        draft: fm.draft,
        dir: dir_of(source).to_string(),
        source: source.to_string(),
        content: body.to_string(),
    })
}

fn frontmatter_of(raw: &str) -> Option<Frontmatter> {
    frontmatter::split(raw).ok().and_then(|(fm, _)| fm)
}

/// Orders pages for navigation: explicit `order` wins, then file name
/// (case-insensitive). The sort is stable for reproducible output.
pub fn sort_pages(pages: &mut [Page]) {
    pages.sort_by(|a, b| match (a.order, b.order) {
        (Some(x), Some(y)) if x != y => x.cmp(&y),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        _ => a.source.to_lowercase().cmp(&b.source.to_lowercase()),
    });
}

/// Maps a content path onto its published URL. Pages get trailing-slash URLs
/// so they resolve on any static host without rewrite rules:
/// `guide/install.md` → `/guide/install/`, `index.md`/`README.md` → `/`.
pub fn url_for_source(source: &str) -> String {
    let base = source.trim_matches('/');
    let trimmed = trim_markdown_ext(base);
    let base_name = trimmed.rsplit('/').next().unwrap_or("");

    if base_name == "index" {
        let dir = trimmed.rsplit_once('/').map(|(d, _)| d).unwrap_or("");
        return if dir.is_empty() {
            "/".to_string()
        } else {
            format!("/{dir}/")
        };
    }
    if base_name.eq_ignore_ascii_case("readme") && !trimmed.contains('/') {
        return "/".to_string();
    }
    if trimmed.is_empty() {
        return "/".to_string();
    }
    format!("/{trimmed}/")
}

/// Maps a content path onto the HTML file written to the output dir.
pub fn output_path_for_source(source: &str) -> String {
    let url = url_for_source(source);
    let url = url.trim_start_matches('/');
    if url.is_empty() {
        "index.html".to_string()
    } else {
        format!("{url}index.html")
    }
}

fn trim_markdown_ext(path: &str) -> &str {
    for ext in MARKDOWN_EXTS {
        if let Some(stem) = path.strip_suffix(ext) {
            return stem;
        }
    }
    path
}

fn is_markdown(source: &str) -> bool {
    MARKDOWN_EXTS.iter().any(|ext| source.ends_with(ext))
}

fn is_backup(name: &str) -> bool {
    BACKUP_SUFFIXES.iter().any(|s| name.ends_with(s))
}

fn is_partial(name: &str) -> bool {
    name.starts_with('_') || PARTIAL_NAMES.contains(&name)
}

fn dir_of(source: &str) -> &str {
    source.rsplit_once('/').map(|(d, _)| d).unwrap_or("")
}

/// Exposed for the standalone graph scanner.
pub fn dir_of_pub(source: &str) -> &str {
    dir_of(source)
}

/// Exposed for the standalone graph scanner.
pub fn title_from_source_pub(source: &str) -> String {
    title_from_source(source)
}

/// Derives a readable title from a file path: `installation.md` →
/// "Installation", `guide/index.md` → "Guide".
fn title_from_source(source: &str) -> String {
    let stem = trim_markdown_ext(source);
    let base = stem.rsplit('/').next().unwrap_or("");
    if base == "index" || base.eq_ignore_ascii_case("readme") {
        let dir = dir_of(source);
        if dir.is_empty() {
            return "Home".to_string();
        }
        return humanize(dir.rsplit('/').next().unwrap_or(dir));
    }
    humanize(base)
}

/// Exposed for the navigation builder, which titles groups after directory
/// names with the same humanizing rule as file names.
pub fn humanize_pub(slug: &str) -> String {
    humanize(slug)
}

fn humanize(slug: &str) -> String {
    let words: Vec<String> = slug
        .split(['-', '_'])
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect();
    if words.is_empty() {
        slug.to_string()
    } else {
        words.join(" ")
    }
}

/// Lookup table from source path to page, for link resolution.
pub fn index_by_source(pages: &[Page]) -> HashMap<String, usize> {
    pages
        .iter()
        .enumerate()
        .map(|(i, p)| (p.source.clone(), i))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_markdown_paths_to_urls() {
        assert_eq!(url_for_source("index.md"), "/");
        assert_eq!(url_for_source("README.md"), "/");
        assert_eq!(url_for_source("guide/index.md"), "/guide/");
        assert_eq!(url_for_source("guide/install.md"), "/guide/install/");
        assert_eq!(
            output_path_for_source("guide/install.md"),
            "guide/install/index.html"
        );
        assert_eq!(output_path_for_source("index.md"), "index.html");
    }

    #[test]
    fn humanizes_slugs() {
        assert_eq!(humanize("getting-started"), "Getting Started");
        assert_eq!(humanize("cli_reference"), "Cli Reference");
    }
}
