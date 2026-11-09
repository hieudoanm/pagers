//! HTML assembly: the page template plus the recursive navigation and
//! outline markup it embeds. Templates use `{{TOKEN}}` placeholders replaced
//! in a single pass, so rendered Markdown can never be reinterpreted as a
//! token.

use std::collections::HashMap;
use std::fmt::Write;

use crate::theme;
use pagers_markdown::Heading;
use pagers_site::config::Config;
use pagers_site::nav::{Group, Nav};
use pagers_site::site::Page;

/// Everything one page render needs.
pub struct View<'a> {
    pub config: &'a Config,
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub body_html: &'a str,
    pub outline: &'a [Heading],
    pub nav: Option<&'a Nav>,
    pub pages: &'a [Page],
    /// The current page's URL, for marking the active nav entry.
    pub current_url: &'a str,
    /// True for the generated landing page, whose body is the nav itself.
    pub is_landing: bool,
}

/// Renders a complete HTML page.
pub fn render_page(view: &View) -> String {
    let mut vars: HashMap<&str, String> = HashMap::new();
    let config = view.config;
    let asset = |name: &str| config.url(&format!("/{}/{}", theme::ASSET_DIR, name));

    vars.insert("LANG", escape(&config.language));
    vars.insert("THEME", escape(&config.theme));
    vars.insert(
        "TITLE",
        if view.title.is_empty() {
            escape(&config.title)
        } else {
            format!("{} · {}", escape(view.title), escape(&config.title))
        },
    );
    vars.insert(
        "DESCRIPTION",
        view.description
            .map(|d| {
                format!(
                    "<meta name=\"description\" content=\"{}\" />\n    ",
                    escape(d)
                )
            })
            .unwrap_or_default(),
    );
    vars.insert("ASSET_MD", escape(&asset("md.css")));
    vars.insert("ASSET_CSS", escape(&asset("styles.css")));
    vars.insert("ASSET_JS", escape(&asset("script.js")));
    vars.insert("ASSET_FAVICON", escape(&asset("favicon.svg")));
    vars.insert("ASSET_FAVICON_PNG", escape(&asset("favicon-32.png")));
    vars.insert("ASSET_APPLE", escape(&asset("apple-touch-icon.png")));
    vars.insert("FAVICON_ICO_URL", escape(&config.url("/favicon.ico")));
    vars.insert("MANIFEST_URL", escape(&config.url("/site.webmanifest")));
    vars.insert(
        "THEME_COLOR",
        (if config.theme == "dark" {
            "#111827"
        } else {
            "#ffffff"
        })
        .to_string(),
    );
    vars.insert("HOME_URL", escape(&config.url("/")));
    vars.insert("GRAPH_URL", escape(&config.url("/graph.html")));
    vars.insert("SITE_TITLE", escape(&config.title));
    vars.insert("PAGE_TITLE", escape(view.title));
    vars.insert(
        "LEDE",
        view.description
            .map(|d| format!("<p class=\"markdown__lede\">{}</p>\n          ", escape(d)))
            .unwrap_or_default(),
    );
    vars.insert(
        "MD_DARK",
        if config.theme == "dark" {
            " md-dark"
        } else {
            ""
        }
        .to_string(),
    );
    vars.insert(
        "LAYOUT_WIDE",
        if view.nav.is_none_or(|n| n.is_empty()) {
            " layout--wide"
        } else {
            ""
        }
        .to_string(),
    );
    vars.insert(
        "FOOTER",
        config
            .footer
            .as_deref()
            .map(|f| format!(" · {}", escape(f)))
            .unwrap_or_default(),
    );

    // Structural values are substituted last; they contain trusted markup.
    let sidebar = view
        .nav
        .filter(|n| !n.is_empty())
        .map(|n| sidebar_html(n, view.pages, config, view.current_url))
        .unwrap_or_default();
    let nav_toggle = if view.nav.is_some_and(|n| !n.is_empty()) {
        "<button class=\"header__toggle\" type=\"button\" aria-expanded=\"false\" aria-controls=\"sidebar\" data-nav-toggle><span class=\"header__toggle-icon\" aria-hidden=\"true\"></span><span class=\"visually-hidden\">Menu</span></button>"
            .to_string()
    } else {
        String::new()
    };
    vars.insert("NAV_TOGGLE", nav_toggle);
    vars.insert("SIDEBAR", sidebar);

    let body = if view.is_landing {
        landing_body(view.nav, view.pages, config)
    } else {
        view.body_html.to_string()
    };
    vars.insert("BODY", body);
    vars.insert("OUTLINE", outline_html(view.outline));

    substitute(theme::PAGE_TEMPLATE, &vars)
}

fn sidebar_html(nav: &Nav, pages: &[Page], config: &Config, current: &str) -> String {
    let mut out = String::from("<nav class=\"sidebar\" id=\"sidebar\" aria-label=\"Site\">");
    write_nav_list(&mut out, &nav_items(nav, pages, config, current), 0);
    out.push_str("</nav>\n      ");
    out
}

/// One navigation entry: a link, or a heading/group over children.
struct NavItem {
    title: String,
    url: Option<String>,
    current: bool,
    group: bool,
    children: Vec<NavItem>,
}

/// Flattens the nav tree into renderable items: root pages first, then
/// top-level groups with their pages and nested groups as children.
fn nav_items(nav: &Nav, pages: &[Page], config: &Config, current: &str) -> Vec<NavItem> {
    let mut items: Vec<NavItem> = nav
        .pages
        .iter()
        .map(|&i| leaf_item(&pages[i], config, current))
        .collect();
    items.extend(
        nav.groups
            .iter()
            .map(|g| group_item(g, pages, config, current)),
    );
    items
}

fn leaf_item(page: &Page, config: &Config, current: &str) -> NavItem {
    NavItem {
        title: page.nav_title().to_string(),
        url: Some(config.url(&page.url)),
        current: page.url == current,
        group: false,
        children: Vec::new(),
    }
}

fn group_item(group: &Group, pages: &[Page], config: &Config, current: &str) -> NavItem {
    let mut children: Vec<NavItem> = group
        .pages
        .iter()
        .map(|&i| leaf_item(&pages[i], config, current))
        .collect();
    children.extend(
        group
            .groups
            .iter()
            .map(|g| group_item(g, pages, config, current)),
    );
    NavItem {
        title: group.title.clone(),
        url: group.url.as_ref().map(|u| config.url(u)),
        current: group.url.as_deref() == Some(current),
        group: true,
        children,
    }
}

fn write_nav_list(out: &mut String, items: &[NavItem], depth: usize) {
    if items.is_empty() {
        return;
    }
    let class = if depth == 0 { "nav" } else { "nav nav--nested" };
    let _ = write!(out, "<ul class=\"{class}\">");
    for item in items {
        let _ = write!(
            out,
            "<li class=\"nav__item{}\">",
            if item.group { " nav__item--group" } else { "" }
        );
        match &item.url {
            Some(url) => {
                let _ = write!(
                    out,
                    "<a class=\"nav__link{}\" href=\"{}\"{}>{}</a>",
                    if item.current {
                        " nav__link--current"
                    } else {
                        ""
                    },
                    escape(url),
                    if item.current {
                        " aria-current=\"page\""
                    } else {
                        ""
                    },
                    escape(&item.title)
                );
            }
            None => {
                let _ = write!(
                    out,
                    "<span class=\"nav__heading\">{}</span>",
                    escape(&item.title)
                );
            }
        }
        write_nav_list(out, &item.children, depth + 1);
        out.push_str("</li>");
    }
    out.push_str("</ul>");
}

/// The generated landing page's body: the navigation itself, since a site
/// with no `index.md` still needs a front door.
fn landing_body(nav: Option<&Nav>, pages: &[Page], config: &Config) -> String {
    let mut out = String::from(
        "<p class=\"markdown__lede\">Everything in this site, in order.</p>\n<nav class=\"landing\" aria-label=\"Pages\">",
    );
    if let Some(nav) = nav {
        write_nav_list(&mut out, &nav_items(nav, pages, config, "/"), 0);
    }
    out.push_str("</nav>");
    out
}

/// The "On this page" aside; level-1 headings are the page title already.
fn outline_html(outline: &[Heading]) -> String {
    if !outline.iter().any(|h| h.level > 1) {
        return String::new();
    }
    let mut out = String::from(
        "<aside class=\"outline\" aria-label=\"On this page\"><h2 class=\"outline__title\">On this page</h2><ul class=\"outline__list\">",
    );
    for h in outline.iter().filter(|h| h.level > 1) {
        let _ = write!(
            out,
            "<li class=\"outline__item outline__item--{}\"><a class=\"outline__link\" href=\"#{}\">{}</a></li>",
            h.level,
            escape(&h.id),
            escape(&h.text)
        );
    }
    out.push_str("</ul></aside>");
    out
}

/// Single-pass `{{TOKEN}}` substitution.
fn substitute(template: &str, vars: &HashMap<&str, String>) -> String {
    let mut out = String::with_capacity(template.len() + 4096);
    let mut rest = template;
    while let Some(start) = rest.find("{{") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        match after.find("}}") {
            Some(end) => {
                let token = &after[..end];
                out.push_str(vars.get(token).map(String::as_str).unwrap_or(""));
                rest = &after[end + 2..];
            }
            None => {
                out.push_str(&rest[start..]);
                return out;
            }
        }
    }
    out.push_str(rest);
    out
}

/// Escapes text for HTML content and double-quoted attribute values.
pub fn escape(text: &str) -> String {
    pagers_site::util::escape_html(text)
}
