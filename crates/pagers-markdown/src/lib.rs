//! Markdown rendering: one pass over the pulldown-cmark event stream that
//! injects heading ids, rewrites link destinations through the site resolver,
//! turns GFM alert blockquotes into the `.markdown-alert` markup `md.css`
//! styles, lifts a leading level-1 heading into the page title, and collects
//! the heading outline plus page-to-page edges for the graph.

use std::collections::HashMap;

use pulldown_cmark::html;
use pulldown_cmark::{BlockQuoteKind, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use pagers_site::resolve::{Resolution, Resolver};

/// One entry in a page's table of contents.
#[derive(Debug, Clone)]
pub struct Heading {
    pub level: u8,
    pub id: String,
    pub text: String,
}

/// A page-to-page link found while rendering: (source page, target page),
/// both as content-root-relative paths like `guide/install.md`.
pub type Edge = (String, String);

/// The result of rendering one page's Markdown body.
pub struct Rendered {
    pub html: String,
    pub outline: Vec<Heading>,
    /// Text of a leading level-1 heading, when the document opens with one.
    pub title: Option<String>,
    pub edges: Vec<Edge>,
}

/// Renders `body` (frontmatter already stripped) to HTML. `source` is the
/// page's path relative to the content root; `resolver` rewrites internal
/// link destinations and is `None` for contexts without site context.
pub fn render(source: &str, body: &str, resolver: Option<&Resolver<'_>>) -> Rendered {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_GFM
        | Options::ENABLE_HEADING_ATTRIBUTES;
    let events: Vec<Event> = Parser::new_ext(body, options).collect();

    // A leading level-1 heading becomes the page title and leaves the body,
    // so the theme header never prints it twice.
    let leading = leading_h1_range(&events);
    let title = leading.as_ref().map(|(_, t)| t.clone());
    let skip_until = leading.map(|(range, _)| range.end);

    let mut out: Vec<Event> = Vec::with_capacity(events.len());
    let mut outline = Vec::new();
    let mut edges = Vec::new();
    let mut slugs: HashMap<String, usize> = HashMap::new();

    for (i, event) in events.iter().enumerate() {
        if skip_until.is_some_and(|end| i < end) {
            continue;
        }
        match event {
            Event::Start(Tag::Heading {
                level,
                id,
                classes,
                attrs,
            }) => {
                let text = heading_text(&events, i + 1);
                let slug = unique_slug(&mut slugs, &text);
                let (final_id, id_field) = match id {
                    Some(existing) => (
                        existing.to_string(),
                        CowStr::Boxed(existing.to_string().into_boxed_str()),
                    ),
                    None => (slug.clone(), CowStr::Boxed(slug.into_boxed_str())),
                };
                outline.push(Heading {
                    level: level_number(*level),
                    id: final_id,
                    text,
                });
                out.push(Event::Start(Tag::Heading {
                    level: *level,
                    id: Some(id_field),
                    classes: classes.clone(),
                    attrs: attrs.clone(),
                }));
            }
            Event::Start(Tag::BlockQuote(kind)) => match kind {
                Some(k) => out.push(Event::Html(CowStr::Boxed(alert_open(*k).into_boxed_str()))),
                None => out.push(event.clone()),
            },
            Event::End(TagEnd::BlockQuote(kind)) => match kind {
                Some(_) => out.push(Event::Html(CowStr::Boxed("</div>\n".into()))),
                None => out.push(event.clone()),
            },
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title: link_title,
                id: link_id,
            }) => {
                let (new_url, edge) = resolve_destination(resolver, dest_url, true);
                if let Some(target) = edge {
                    edges.push((source.to_string(), target));
                }
                out.push(Event::Start(Tag::Link {
                    link_type: *link_type,
                    dest_url: CowStr::Boxed(new_url.into_boxed_str()),
                    title: link_title.clone(),
                    id: link_id.clone(),
                }));
            }
            Event::Start(Tag::Image {
                link_type,
                dest_url,
                title: image_title,
                id: image_id,
            }) => {
                let (new_url, _) = resolve_destination(resolver, dest_url, false);
                out.push(Event::Start(Tag::Image {
                    link_type: *link_type,
                    dest_url: CowStr::Boxed(new_url.into_boxed_str()),
                    title: image_title.clone(),
                    id: image_id.clone(),
                }));
            }
            other => out.push(other.clone()),
        }
    }

    let mut html_out = String::new();
    html::push_html(&mut html_out, out.into_iter());
    Rendered {
        html: html_out,
        outline,
        title,
        edges,
    }
}

fn resolve_destination(
    resolver: Option<&Resolver<'_>>,
    dest: &str,
    is_link: bool,
) -> (String, Option<String>) {
    let Some(resolver) = resolver else {
        return (dest.to_string(), None);
    };
    match resolver.resolve(dest) {
        Resolution::Page { url, source } => {
            let edge = is_link.then_some(source);
            (url, edge)
        }
        Resolution::Asset { url } => (url, None),
        Resolution::Untouched => (dest.to_string(), None),
    }
}

/// Finds the event range of a leading level-1 heading plus its plain-text
/// title. Returns `None` when the document does not open with an `h1`.
fn leading_h1_range(events: &[Event]) -> Option<(std::ops::Range<usize>, String)> {
    if !matches!(
        events.first(),
        Some(Event::Start(Tag::Heading {
            level: HeadingLevel::H1,
            ..
        }))
    ) {
        return None;
    }
    for (i, event) in events.iter().enumerate().skip(1) {
        if matches!(event, Event::End(TagEnd::Heading(_))) {
            return Some(((0..i + 1), heading_text(events, 1)));
        }
    }
    None
}

/// Collects the plain text inside a heading, starting just after its
/// `Start` event.
fn heading_text(events: &[Event], start: usize) -> String {
    let mut text = String::new();
    for event in events.iter().skip(start) {
        match event {
            Event::End(TagEnd::Heading(_)) => break,
            Event::Text(t) | Event::Code(t) => text.push_str(t),
            Event::SoftBreak | Event::HardBreak => text.push(' '),
            _ => {}
        }
    }
    text.trim().to_string()
}

/// Slugifies heading text GitHub-style and dedupes within the document.
fn unique_slug(slugs: &mut HashMap<String, usize>, text: &str) -> String {
    let base = slugify(text);
    let count = slugs.entry(base.clone()).or_insert(0);
    *count += 1;
    if *count == 1 {
        base
    } else {
        format!("{base}-{}", *count - 1)
    }
}

fn slugify(text: &str) -> String {
    let mut out = String::new();
    let mut pending_dash = false;
    for c in text.chars().flat_map(char::to_lowercase) {
        if c.is_alphanumeric() {
            if pending_dash && !out.is_empty() {
                out.push('-');
            }
            pending_dash = false;
            out.push(c);
        } else if c.is_whitespace() || c == '-' || c == '_' {
            pending_dash = true;
        }
    }
    out.trim_matches('-').to_string()
}

fn level_number(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

/// GitHub alert kinds mapped to the `data-markdown-alert` values `md.css`
/// styles, with their visible labels.
fn alert_open(kind: BlockQuoteKind) -> String {
    let (name, label) = match kind {
        BlockQuoteKind::Note => ("note", "Note"),
        BlockQuoteKind::Tip => ("tip", "Tip"),
        BlockQuoteKind::Important => ("important", "Important"),
        BlockQuoteKind::Warning => ("warning", "Warning"),
        BlockQuoteKind::Caution => ("caution", "Caution"),
    };
    format!(
        "<div class=\"markdown-alert\" data-markdown-alert=\"{name}\">\n<p class=\"markdown-alert-title\">{label}</p>\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_heading_with_id() {
        let r = render("a.md", "# Title\n\n## Section One\n\nText.\n", None);
        assert!(r.html.contains("<h2 id=\"section-one\">"));
        assert_eq!(r.title.as_deref(), Some("Title"));
        assert!(!r.html.contains("<h1"));
        assert_eq!(r.outline.len(), 1);
        assert_eq!(r.outline[0].id, "section-one");
    }

    #[test]
    fn dedupes_heading_ids() {
        let r = render("a.md", "## Same\n\n## Same\n", None);
        assert!(r.html.contains("id=\"same\""));
        assert!(r.html.contains("id=\"same-1\""));
    }

    #[test]
    fn renders_alert_blockquote() {
        let r = render("a.md", "> [!NOTE]\n> Remember this.\n", None);
        assert!(r.html.contains("markdown-alert"));
        assert!(r.html.contains("data-markdown-alert=\"note\""));
        assert!(!r.html.contains("[!NOTE]"));
    }

    #[test]
    fn keeps_plain_blockquote() {
        let r = render("a.md", "> Just a quote.\n", None);
        assert!(r.html.contains("<blockquote>"));
        assert!(!r.html.contains("markdown-alert"));
    }

    #[test]
    fn renders_table_and_task_list() {
        let r = render(
            "a.md",
            "| a | b |\n|---|---|\n| 1 | 2 |\n\n- [ ] todo\n- [x] done\n",
            None,
        );
        assert!(r.html.contains("<table>"));
        assert!(r.html.contains("type=\"checkbox\""));
    }
}
