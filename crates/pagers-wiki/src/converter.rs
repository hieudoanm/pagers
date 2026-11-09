//! HTML → Markdown conversion. Cleans the article DOM on htmd's mutable
//! tree first — removing site chrome, absolutizing links, unwrapping
//! Parsoid sections, turning MathML into LaTeX — then converts with htmd
//! and applies Wikipedia-specific element handlers.

use std::rc::Rc;

use htmd::{HtmlToMarkdown, Node};
use markup5ever::{local_name, ns};
use markup5ever_rcdom::NodeData;

use crate::error::WikiError;
use crate::metadata::file_slug;
use crate::url::WikiArticle;

/// Class fragments that mark non-content elements safe to drop. Matching is
/// substring-based because Parsoid appends state classes.
const NOISE_CLASSES: &[&str] = &[
    "mw-editsection",
    "noprint",
    "navbox",
    "metadata",
    "ambox",
    "hatnote-no",
    "sistersitebox",
    "vertical-navbox",
    "navbar",
    "toc",
    ".mw-empty-elt",
];

/// Tags that never carry article content.
const SKIP_TAGS: &[&str] = &[
    "script", "style", "link", "meta", "noscript", "template", "iframe", "object", "embed",
];

/// Everything the tree cleaner needs to know about the article.
pub struct CleanContext<'a> {
    pub article: &'a WikiArticle,
    pub include_references: bool,
    /// Rewrite internal `/wiki/` links to `<slug>.md` filenames, keeping the
    /// absolute article URL in the link `title` attribute.
    pub local_links: bool,
}

/// Converts an extracted article HTML fragment to Markdown.
pub fn to_markdown(html: &str, ctx: &CleanContext) -> Result<String, WikiError> {
    let converter = HtmlToMarkdown::new();
    let root = converter
        .html_to_tree(html)
        .map_err(|e| WikiError::Conversion(e.to_string()))?;
    clean_children(&root, ctx);
    if !ctx.include_references {
        drop_references_sections(&root);
    }
    let markdown = converter.tree_to_markdown(&root);
    if markdown.trim().is_empty() {
        return Err(WikiError::EmptyContent(ctx.article.title.clone()));
    }
    Ok(markdown)
}

/// Recursively rebuilds a node's child list, dropping noise and applying
/// per-element transforms. `RefCell::take` avoids aliasing the borrow while
/// children are rewritten.
fn clean_children(node: &Rc<Node>, ctx: &CleanContext) {
    let children = node.children.take();
    let mut kept: Vec<Rc<Node>> = Vec::with_capacity(children.len());
    for child in children {
        match &child.data {
            NodeData::Element { .. } => {
                if let Some(replacement) = transform_element(&child, ctx) {
                    kept.extend(replacement);
                }
            }
            NodeData::Text { .. } => kept.push(child.clone()),
            _ => {}
        }
    }
    *node.children.borrow_mut() = kept;
}

/// Applies one element's transform. Returns:
/// - `None` → drop the element entirely
/// - `Some(vec![])` → unwrap: splice children in place of the element
/// - `Some(vec![node])` → replace with a single node
/// - `Some(vec![node, ...])` → replace with several nodes
fn transform_element(node: &Rc<Node>, ctx: &CleanContext) -> Option<Vec<Rc<Node>>> {
    let NodeData::Element { name, attrs, .. } = &node.data else {
        return Some(vec![node.clone()]);
    };
    let tag = name.local.as_ref();

    if SKIP_TAGS.contains(&tag) || has_noise_class(attrs) || is_hidden(attrs) {
        return None;
    }
    if !ctx.include_references && is_reference_container(attrs) {
        return None;
    }
    if is_reference_marker(node, ctx) {
        return None;
    }
    if is_math(node) {
        return Some(vec![math_text_node(node)]);
    }
    if tag == "section" {
        // Parsoid wraps sections in <section data-mw-section-id>; the tag
        // carries no semantics Markdown needs, so unwrap it.
        clean_children(node, ctx);
        return Some(node.children.take());
    }

    rewrite_attrs(attrs, ctx);
    clean_children(node, ctx);
    Some(vec![node.clone()])
}

type Attrs = std::cell::RefCell<Vec<markup5ever::Attribute>>;

fn has_noise_class(attrs: &Attrs) -> bool {
    let Some(class) = attr_value(attrs, "class") else {
        return false;
    };
    let class = class.to_ascii_lowercase();
    NOISE_CLASSES.iter().any(|c| class.contains(c))
}

fn is_hidden(attrs: &Attrs) -> bool {
    attrs.borrow().iter().any(|a| {
        a.name.local.as_ref() == "hidden"
            || (a.name.local.as_ref() == "style"
                && a.value.to_ascii_lowercase().contains("display:none"))
    })
}

fn is_reference_marker(node: &Rc<Node>, ctx: &CleanContext) -> bool {
    if ctx.include_references {
        return false;
    }
    let NodeData::Element { name, attrs, .. } = &node.data else {
        return false;
    };
    if name.local.as_ref() != "sup" {
        return false;
    }
    let class = attr_value(attrs, "class").unwrap_or_default();
    class.contains("reference") || class.contains("mw-ref")
}

/// Matches the containers that hold the references/footnotes/reflist
/// sections when `--include-references` is off.
fn is_reference_container(attrs: &Attrs) -> bool {
    let Some(class) = attr_value(attrs, "class") else {
        return false;
    };
    let class = class.to_ascii_lowercase();
    ["mw-references", "reflist", "mw-reference-text", "footnotes"]
        .iter()
        .any(|c| class.contains(c))
}

fn is_math(node: &Rc<Node>) -> bool {
    let NodeData::Element { name, attrs, .. } = &node.data else {
        return false;
    };
    if name.local.as_ref() == "math" {
        return true;
    }
    attr_value(attrs, "class")
        .map(|c| c.contains("mwe-math"))
        .unwrap_or(false)
}

/// Replaces a MathML/math-span element with a text node holding LaTeX.
fn math_text_node(node: &Rc<Node>) -> Rc<Node> {
    let tex = find_annotation(node).unwrap_or_else(|| all_text(node));
    let tex = tex.trim();
    if tex.is_empty() {
        return Node::new(NodeData::Text {
            contents: std::cell::RefCell::new(" ".into()),
        });
    }
    let display = is_display_math(node);
    let md = if display {
        format!("\n\n$${tex}$$\n\n")
    } else {
        format!(" ${tex}$ ")
    };
    Node::new(NodeData::Text {
        contents: std::cell::RefCell::new(md.as_str().into()),
    })
}

fn is_display_math(node: &Rc<Node>) -> bool {
    let NodeData::Element { attrs, .. } = &node.data else {
        return false;
    };
    let class = attr_value(attrs, "class").unwrap_or_default();
    class.contains("mwe-math-display") || class.contains("mwe-math-fallback")
}

/// Finds the `<annotation encoding="application/x-tex">` LaTeX source inside
/// a MathML tree, falling back handled by the caller.
fn find_annotation(node: &Rc<Node>) -> Option<String> {
    if let NodeData::Element { name, attrs, .. } = &node.data {
        if name.local.as_ref() == "annotation"
            && attr_value(attrs, "encoding")
                .map(|e| e.contains("x-tex"))
                .unwrap_or(false)
        {
            return Some(all_text(node));
        }
    }
    for child in node.children.borrow().iter() {
        if let Some(found) = find_annotation(child) {
            return Some(found);
        }
    }
    None
}

fn all_text(node: &Rc<Node>) -> String {
    let mut out = String::new();
    collect_text(node, &mut out);
    out
}

fn collect_text(node: &Rc<Node>, out: &mut String) {
    match &node.data {
        NodeData::Text { contents } => out.push_str(&contents.borrow()),
        _ => {
            for child in node.children.borrow().iter() {
                collect_text(child, out);
            }
        }
    }
}

/// Rewrites `href`/`src` attributes to absolute URLs and strips
/// Parsoid-internal attributes (`data-mw`, `about`, `typeof`, `rel`,
/// `style`) that carry no Markdown meaning.
fn rewrite_attrs(attrs: &Attrs, ctx: &CleanContext) {
    let mut attrs = attrs.borrow_mut();
    let is_link = attrs.iter().any(|a| a.name.local.as_ref() == "href");
    for attr in attrs.iter_mut() {
        let name = attr.name.local.as_ref();
        if name == "href" || name == "src" {
            if let Some(abs) = absolutize(&attr.value, ctx, is_link) {
                attr.value = abs.into();
            }
        }
    }
    if ctx.local_links && is_link {
        localize_wiki_link(&mut attrs, ctx);
    }
    attrs.retain(|a| !is_stripped_name(a.name.local.as_ref()));
}

/// MediaWiki namespaces whose `/wiki/` links are not articles.
const NAMESPACES: &[&str] = &[
    "file",
    "image",
    "category",
    "wikipedia",
    "template",
    "portal",
    "special",
    "help",
    "talk",
    "draft",
    "module",
    "mediawiki",
    "mos",
    "user",
];

/// Rewrites an internal `/wiki/<title>` link to `<slug>.md`, preserving the
/// absolute article URL in the link `title` (emitted as `[t](f.md "url")`).
/// Non-article namespaces and non-wiki links are left untouched.
fn localize_wiki_link(attrs: &mut Vec<markup5ever::Attribute>, ctx: &CleanContext) {
    let prefix = format!("{}/wiki/", ctx.article.wiki_root);
    let Some(href_idx) = attrs.iter().position(|a| a.name.local.as_ref() == "href") else {
        return;
    };
    let href = attrs[href_idx].value.to_string();
    let Some(rest) = href.strip_prefix(&prefix) else {
        return;
    };
    // Split off `#fragment` / `?query`; only the page title becomes a filename.
    let cut = rest.find(['#', '?']).unwrap_or(rest.len());
    let (title_part, suffix) = rest.split_at(cut);
    let title = percent_decode(title_part);
    if !is_article_title(&title) {
        return;
    }
    let page_url = format!("{prefix}{title_part}");
    attrs[href_idx].value = format!("{}.md{suffix}", file_slug(&title)).into();
    if let Some(t) = attrs.iter_mut().find(|a| a.name.local.as_ref() == "title") {
        t.value = page_url.into();
    } else {
        attrs.push(markup5ever::Attribute {
            name: markup5ever::QualName::new(None, ns!(), local_name!("title")),
            value: page_url.into(),
        });
    }
}

/// True when the decoded `/wiki/` title is an article (no `Namespace:` prefix).
fn is_article_title(title: &str) -> bool {
    match title.split_once(':') {
        Some((ns, _)) => !NAMESPACES.contains(&ns.to_lowercase().as_str()),
        None => !title.trim().is_empty(),
    }
}

/// Minimal percent-decoding for URL path segments (`%C3%A9` → `é`).
fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(v) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn is_stripped_name(local: &str) -> bool {
    let n = local;
    n.starts_with("data-") || matches!(n, "about" | "typeof" | "rel" | "style" | "resource")
}

/// Converts a relative or protocol-relative URL into an absolute one.
/// `is_link` distinguishes `#fragment` handling (anchors resolve against
/// the article URL) from images.
fn absolutize(raw: &str, ctx: &CleanContext, is_link: bool) -> Option<String> {
    absolutize_raw(raw.trim(), ctx, is_link).map(|u| strip_tracking(&u))
}

fn absolutize_raw(raw: &str, ctx: &CleanContext, is_link: bool) -> Option<String> {
    if raw.is_empty() || raw.starts_with("mailto:") || raw.contains("://") {
        return Some(raw.to_string());
    }
    if raw.starts_with("//") {
        return Some(format!("https:{raw}"));
    }
    if raw.starts_with('/') {
        return Some(format!("{}{}", ctx.article.wiki_root, raw));
    }
    if raw.starts_with("./") {
        // `./Foo` is relative to the /wiki/ directory on Wikipedia.
        let path = raw.trim_start_matches("./");
        return Some(format!("{}/wiki/{path}", ctx.article.wiki_root));
    }
    if is_link && raw.starts_with('#') {
        return Some(format!("{}{}", ctx.article.article_url, raw));
    }
    if is_link && !raw.contains('/') {
        // Bare `Foo` links are rare; treat like ./Foo.
        return Some(format!("{}/wiki/{raw}", ctx.article.wiki_root));
    }
    Some(raw.to_string())
}

/// Removes `utm_*` tracking parameters Wikipedia appends to thumbnail URLs.
fn strip_tracking(url: &str) -> String {
    let Ok(mut parsed) = ::url::Url::parse(url) else {
        return url.to_string();
    };
    let pairs: Vec<(String, String)> = parsed
        .query_pairs()
        .filter(|(k, _)| !k.starts_with("utm_"))
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();
    if pairs.is_empty() {
        parsed.set_query(None);
    } else {
        parsed.query_pairs_mut().clear().extend_pairs(pairs);
    }
    parsed.to_string()
}

/// Finds an attribute's value by local name.
fn attr_value(attrs: &Attrs, name: &str) -> Option<String> {
    attrs
        .borrow()
        .iter()
        .find(|a| a.name.local.as_ref() == name)
        .map(|a| a.value.to_string())
}

/// Removes the whole trailing references section — heading plus every
/// sibling until the next heading of equal or higher level — when
/// `--include-references` is off. Matches the heading texts Wikipedia
/// uses for citation sections.
fn drop_references_sections(node: &Rc<Node>) {
    let children = node.children.take();
    let mut kept: Vec<Rc<Node>> = Vec::with_capacity(children.len());
    let mut skipping: Option<u8> = None;
    for child in children {
        if let Some(level) = heading_level(&child) {
            if is_references_heading(&child) {
                skipping = Some(level);
                continue;
            }
            if skipping.is_some_and(|l| level <= l) {
                skipping = None;
            }
        }
        if skipping.is_none() {
            drop_references_sections(&child);
            kept.push(child);
        }
    }
    *node.children.borrow_mut() = kept;
}

fn heading_level(node: &Rc<Node>) -> Option<u8> {
    let NodeData::Element { name, .. } = &node.data else {
        return None;
    };
    let tag = name.local.as_ref();
    if tag.len() == 2 && tag.starts_with('h') && tag.as_bytes()[1].is_ascii_digit() {
        tag.as_bytes()[1].checked_sub(b'0')
    } else {
        None
    }
}

fn is_references_heading(node: &Rc<Node>) -> bool {
    let text = all_text(node).trim().to_string();
    ["references", "notes", "footnotes", "citations"]
        .iter()
        .any(|t| text.eq_ignore_ascii_case(t))
}
