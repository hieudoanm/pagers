//! The page-link graph: every Markdown page is a node, every internal
//! Markdown link an edge. `build` embeds the graph in the site as
//! `graph.html`; `pagers graph` writes it standalone as HTML, JSON or DOT.

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use pulldown_cmark::{Event, Options, Parser, Tag};
use serde_json::{json, Value};

use pagers_markdown::Edge;
use pagers_site::site::{self, Page};

const GRAPH_CSS: &str = include_str!("../assets/graph.css");
const GRAPH_JS: &str = include_str!("../assets/graph.js");
const GRAPH_TEMPLATE: &str = include_str!("../assets/graph.html");

/// One node in the graph payload consumed by `graph.js`.
#[derive(Debug, serde::Serialize)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub degree: usize,
}

/// One directed edge.
#[derive(Debug, serde::Serialize)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
}

/// Build-time stats shown in the graph page header.
#[derive(Debug, Default)]
pub struct Stats {
    pub nodes: usize,
    pub edges: usize,
    pub broken: usize,
}

/// The assembled graph.
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<GraphEdge>,
    pub stats: Stats,
}

/// Assembles the graph from rendered pages and the edges their Markdown
/// produced. Edges are deduplicated per directed pair.
pub fn from_pages(pages: &[Page], edges: &[Edge]) -> Graph {
    let mut degree: HashMap<&str, usize> = HashMap::new();
    let mut seen: HashSet<(&str, &str)> = HashSet::new();
    let mut graph_edges = Vec::new();

    for (from, to) in edges {
        if from == to || !seen.insert((from.as_str(), to.as_str())) {
            continue;
        }
        *degree.entry(from).or_default() += 1;
        *degree.entry(to).or_default() += 1;
        graph_edges.push(GraphEdge {
            from: from.clone(),
            to: to.clone(),
        });
    }

    let nodes: Vec<Node> = pages
        .iter()
        .map(|p| Node {
            id: p.source.clone(),
            label: p.title.clone(),
            path: p.source.clone(),
            url: Some(p.url.clone()),
            degree: degree.get(p.source.as_str()).copied().unwrap_or(0),
        })
        .collect();

    Graph {
        stats: Stats {
            nodes: nodes.len(),
            edges: graph_edges.len(),
            broken: 0,
        },
        nodes,
        edges: graph_edges,
    }
}

/// Rewrites node URLs through the site's `basePath` so the "Open page"
/// links in `graph.html` resolve correctly under sub-path hosting.
pub fn apply_base_path(graph: &mut Graph, config: &pagers_site::config::Config) {
    for node in &mut graph.nodes {
        if let Some(url) = node.url.take() {
            node.url = Some(config.url(&url));
        }
    }
}

/// Serializes the graph to the JSON shape `graph.js` consumes.
pub fn to_json(graph: &Graph) -> Value {
    json!({
        "nodes": graph.nodes,
        "edges": graph.edges,
    })
}

/// Renders the standalone interactive graph page.
pub fn render_html(graph: &Graph, title: &str, theme: &str) -> String {
    let mut out = String::with_capacity(GRAPH_TEMPLATE.len() + 65536);
    let data = to_json(graph).to_string();
    // `</script>` inside JSON strings would close the tag early.
    let data = data.replace('<', "\\u003c");

    let mut rest = GRAPH_TEMPLATE;
    while let Some(start) = rest.find("{{") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        let Some(end) = after.find("}}") else {
            out.push_str(&rest[start..]);
            return out;
        };
        let token = &after[..end];
        let value = match token {
            "TITLE" => pagers_site::util::escape_html(title),
            "THEME" => theme.to_string(),
            "CSS" => GRAPH_CSS.to_string(),
            "JS" => GRAPH_JS.to_string(),
            "DATA" => data.clone(),
            "NODES" => graph.stats.nodes.to_string(),
            "EDGES" => graph.stats.edges.to_string(),
            _ => String::new(),
        };
        out.push_str(&value);
        rest = &after[end + 2..];
    }
    out.push_str(rest);
    out
}

/// Scans a content directory for Markdown links and builds a graph,
/// independently of the site build. Used by `pagers graph`.
pub fn scan(root: &str) -> Result<Graph, String> {
    let files = markdown_files(Path::new(root))?;
    let node_set: HashSet<String> = files.iter().map(|(rel, _)| rel.clone()).collect();

    let mut edges: Vec<GraphEdge> = Vec::new();
    let mut broken = 0usize;
    let mut seen: HashSet<(String, String)> = HashSet::new();

    for (rel, path) in &files {
        let Ok(raw) = std::fs::read_to_string(path) else {
            continue;
        };
        let (_, body) = pagers_site::frontmatter::split(&raw).unwrap_or((None, raw.as_str()));
        let options = Options::ENABLE_TABLES
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_GFM;
        let dir = site::dir_of_pub(rel);
        for dest in link_destinations(body, options) {
            let Some(target) = resolve_scan_target(dir, &dest, &node_set, root) else {
                continue;
            };
            match target {
                ScanTarget::Page(source) => {
                    if source != *rel && seen.insert((rel.clone(), source.clone())) {
                        edges.push(GraphEdge {
                            from: rel.clone(),
                            to: source,
                        });
                    }
                }
                ScanTarget::Broken => broken += 1,
                ScanTarget::Skip => {}
            }
        }
    }

    let mut degree: HashMap<String, usize> = HashMap::new();
    for e in &edges {
        *degree.entry(e.from.clone()).or_default() += 1;
        *degree.entry(e.to.clone()).or_default() += 1;
    }
    let mut nodes: Vec<Node> = files
        .iter()
        .map(|(rel, _)| Node {
            id: rel.clone(),
            label: site::title_from_source_pub(rel),
            path: rel.clone(),
            url: None,
            degree: degree.get(rel).copied().unwrap_or(0),
        })
        .collect();
    nodes.sort_by(|a, b| a.id.cmp(&b.id));
    edges.sort_by(|a, b| (&a.from, &a.to).cmp(&(&b.from, &b.to)));

    Ok(Graph {
        stats: Stats {
            nodes: nodes.len(),
            edges: edges.len(),
            broken,
        },
        nodes,
        edges,
    })
}

enum ScanTarget {
    Page(String),
    Broken,
    Skip,
}

/// Extracts link destinations from Markdown, skipping code spans and fenced
/// blocks via the real parser rather than regex.
fn link_destinations(body: &str, options: Options) -> Vec<String> {
    let mut out = Vec::new();
    for event in Parser::new_ext(body, options) {
        if let Event::Start(Tag::Link { dest_url, .. }) = event {
            out.push(dest_url.to_string());
        }
    }
    out
}

/// Resolves a raw link target against the scanned file set.
fn resolve_scan_target(
    from_dir: &str,
    raw: &str,
    node_set: &HashSet<String>,
    root: &str,
) -> Option<ScanTarget> {
    let raw = raw.trim();
    if raw.is_empty() || raw.starts_with('#') || raw.contains("://") || raw.starts_with("mailto:") {
        return Some(ScanTarget::Skip);
    }
    let target = raw
        .split('#')
        .next()
        .unwrap_or(raw)
        .split('?')
        .next()
        .unwrap_or(raw);
    if target.is_empty() {
        return Some(ScanTarget::Skip);
    }
    let joined = if let Some(rest) = target.strip_prefix('/') {
        rest.to_string()
    } else if from_dir.is_empty() {
        target.to_string()
    } else {
        format!("{from_dir}/{target}")
    };
    let cleaned = normalize_scan(&joined);

    for candidate in [
        cleaned.clone(),
        format!("{cleaned}.md"),
        format!("{cleaned}.markdown"),
        format!("{cleaned}/index.md"),
        format!("{cleaned}/index.markdown"),
    ] {
        if node_set.contains(&candidate) {
            return Some(ScanTarget::Page(candidate));
        }
    }
    // Exists on disk but is not Markdown, or escapes the root: not an edge.
    let abs = Path::new(root).join(&cleaned);
    if abs.exists() {
        return Some(ScanTarget::Skip);
    }
    Some(ScanTarget::Broken)
}

fn normalize_scan(path: &str) -> String {
    let mut parts: Vec<&str> = Vec::new();
    for seg in path.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            s => parts.push(s),
        }
    }
    parts.join("/")
}

/// Collects (relative path, absolute path) for every Markdown file under
/// `root`, honouring the same skip rules as site discovery.
fn markdown_files(root: &Path) -> Result<Vec<(String, PathBuf)>, String> {
    let mut out = Vec::new();
    collect_md(root, root, &mut out)?;
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

fn collect_md(root: &Path, dir: &Path, out: &mut Vec<(String, PathBuf)>) -> Result<(), String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("walk {}: {e}", dir.display()))?;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
        if is_dir {
            if !name.starts_with('.')
                && ![".git", "node_modules", "vendor", "dist"].contains(&name.as_str())
            {
                collect_md(root, &path, out)?;
            }
            continue;
        }
        if name.starts_with('.') || name.starts_with('_') {
            continue;
        }
        let lower = name.to_lowercase();
        if !(lower.ends_with(".md") || lower.ends_with(".markdown")) {
            continue;
        }
        if let Ok(rel) = path.strip_prefix(root) {
            out.push((rel.to_string_lossy().replace('\\', "/"), path));
        }
    }
    Ok(())
}

/// DOT rendering for `pagers graph --format dot`.
pub fn to_dot(graph: &Graph) -> String {
    let mut out = String::from("digraph graph {\n  rankdir=TB;\n  splines=polyline;\n");
    for n in &graph.nodes {
        let _ = writeln!(out, "  {:?} [label={:?}];", n.id, file_stem(&n.id));
    }
    for e in &graph.edges {
        let _ = writeln!(out, "  {:?} -> {:?};", e.from, e.to);
    }
    out.push_str("}\n");
    out
}

fn file_stem(path: &str) -> String {
    let base = path.rsplit('/').next().unwrap_or(path);
    base.trim_end_matches(".markdown")
        .trim_end_matches(".md")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dedupes_directed_edges() {
        let pages = vec![
            Page {
                source: "a.md".into(),
                url: "/a/".into(),
                output: "a/index.html".into(),
                title: "A".into(),
                title_from_heading: false,
                description: None,
                label: None,
                order: None,
                draft: false,
                dir: "".into(),
                content: String::new(),
            },
            Page {
                source: "b.md".into(),
                url: "/b/".into(),
                output: "b/index.html".into(),
                title: "B".into(),
                title_from_heading: false,
                description: None,
                label: None,
                order: None,
                draft: false,
                dir: "".into(),
                content: String::new(),
            },
        ];
        let edges = vec![
            ("a.md".to_string(), "b.md".to_string()),
            ("a.md".to_string(), "b.md".to_string()),
            ("b.md".to_string(), "a.md".to_string()),
        ];
        let g = from_pages(&pages, &edges);
        assert_eq!(g.edges.len(), 2);
        assert_eq!(g.stats.nodes, 2);
    }
}
