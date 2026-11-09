//! Site meta files generated on every build: `robots.txt`, `sitemap.xml`,
//! `humans.txt`, favicons, `site.webmanifest`, security/identity stubs,
//! `llms.txt` and `ai.txt`. A same-named file in the content directory
//! always wins — authors can ship their own versions as regular assets.

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use pagers_site::config::Config;
use pagers_site::site::{Content, Page};
use pagers_theme::theme;

/// One sitemap entry: absolute `<loc>` plus the source file for `lastmod`.
struct Entry {
    loc: String,
    source: Option<String>,
}

/// Writes every meta file not already provided by the content directory.
/// `include_landing` adds `/` for generated landing pages (no source file).
pub fn write_meta_files(
    output_dir: &str,
    content_dir: &str,
    config: &Config,
    pages: &[Page],
    include_landing: bool,
    content: &Content,
) -> Result<usize, String> {
    let mut written = 0;
    for (name, data) in meta_files(content_dir, config, pages, include_landing)? {
        if content.assets.iter().any(|(s, _)| s == &name) {
            continue;
        }
        super::write_file(output_dir, &name, &data)?;
        written += 1;
    }
    Ok(written)
}

/// (path, bytes) for every generated meta file.
fn meta_files(
    content_dir: &str,
    config: &Config,
    pages: &[Page],
    include_landing: bool,
) -> Result<Vec<(String, Vec<u8>)>, String> {
    Ok(vec![
        ("robots.txt".into(), robots_txt(config).into_bytes()),
        (
            "sitemap.xml".into(),
            sitemap_xml(content_dir, config, pages, include_landing).into_bytes(),
        ),
        ("humans.txt".into(), humans_txt(config).into_bytes()),
        ("favicon.ico".into(), favicon_ico()),
        ("site.webmanifest".into(), webmanifest(config).into_bytes()),
        (
            ".well-known/security.txt".into(),
            security_txt(config).into_bytes(),
        ),
        (
            "browserconfig.xml".into(),
            browserconfig(config).into_bytes(),
        ),
        ("opensearch.xml".into(), opensearch(config).into_bytes()),
        ("assetlinks.json".into(), b"[]\n".to_vec()),
        (
            "apple-app-site-association".into(),
            br#"{"applinks":{"apps":[],"details":[]},"webcredentials":{"apps":[]}}"#.to_vec(),
        ),
        ("keybase.txt".into(), keybase_txt(config).into_bytes()),
        ("pgpkey.txt".into(), pgpkey_txt(config).into_bytes()),
        (
            "llms.txt".into(),
            llms_txt(config, pages, include_landing).into_bytes(),
        ),
        ("ai.txt".into(), ai_txt(config).into_bytes()),
    ])
}

fn robots_txt(config: &Config) -> String {
    let mut out = String::from("User-agent: *\nAllow: /\n");
    if !config.url.is_empty() {
        out.push_str(&format!("\nSitemap: {}/sitemap.xml\n", config.url));
    }
    out
}

fn humans_txt(config: &Config) -> String {
    let site = if config.url.is_empty() {
        "/"
    } else {
        config.url.as_str()
    };
    format!(
        "/* TEAM */\nProject: {}\nSite: {}\n\n/* SITE */\nDoctype: HTML5\nLanguage: {}\n",
        config.title, site, config.language
    )
}

fn sitemap_xml(
    content_dir: &str,
    config: &Config,
    pages: &[Page],
    include_landing: bool,
) -> String {
    // `url` already includes the base path, so page URLs append raw; without
    // it, locs are base-path-prefixed paths relative to the host root.
    let page_loc = |page_url: &str| {
        let loc = if config.url.is_empty() {
            config.url(page_url)
        } else {
            format!("{}{}", config.url, page_url)
        };
        xml_escape(&loc)
    };
    let mut entries: Vec<Entry> = pages
        .iter()
        .map(|p| Entry {
            loc: page_loc(&p.url),
            source: Some(p.source.clone()),
        })
        .collect();
    if include_landing {
        entries.push(Entry {
            loc: page_loc("/"),
            source: None,
        });
    }
    entries.sort_by(|a, b| a.loc.cmp(&b.loc));
    entries.dedup_by(|a, b| a.loc == b.loc);

    let mut out = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );
    for entry in &entries {
        out.push_str("  <url>\n");
        out.push_str(&format!("    <loc>{}</loc>\n", entry.loc));
        if let Some(lastmod) = entry
            .source
            .as_deref()
            .and_then(|src| file_ymd(content_dir, src))
        {
            out.push_str(&format!("    <lastmod>{lastmod}</lastmod>\n"));
        }
        out.push_str("  </url>\n");
    }
    out.push_str("</urlset>\n");
    out
}

/// `favicon.ico` — an ICO container wrapping the embedded 32px PNG
/// (PNG-compressed icons are valid for all modern browsers).
fn favicon_ico() -> Vec<u8> {
    let png = theme::FAVICON_32_PNG;
    let mut out = Vec::with_capacity(22 + png.len());
    out.extend_from_slice(&[0, 0, 1, 0]); // reserved, type=icon
    out.extend_from_slice(&1u16.to_le_bytes()); // image count
    out.push(32); // width
    out.push(32); // height
    out.extend_from_slice(&[0, 0]); // palette, reserved
    out.extend_from_slice(&1u16.to_le_bytes()); // planes
    out.extend_from_slice(&32u16.to_le_bytes()); // bpp
    out.extend_from_slice(&(png.len() as u32).to_le_bytes());
    out.extend_from_slice(&22u32.to_le_bytes()); // data offset
    out.extend_from_slice(png);
    out
}

fn webmanifest(config: &Config) -> String {
    // `url` includes the base path already; append raw asset paths to it.
    let abs = |path: &str| {
        if config.url.is_empty() {
            config.url(path)
        } else {
            format!("{}{}", config.url, path)
        }
    };
    let icon = |name: &str, sizes: &str| {
        format!(
            "{{\"src\":\"{}\",\"sizes\":\"{}\",\"type\":\"image/png\"}}",
            xml_escape(&abs(&format!("/assets/{name}"))),
            sizes
        )
    };
    let root = abs("/");
    format!(
        "{{\n  \"name\": {},\n  \"short_name\": {},\n  \"lang\": {},\n  \"start_url\": {},\n  \"scope\": {},\n  \"display\": \"browser\",\n  \"background_color\": \"#ffffff\",\n  \"theme_color\": \"#ffffff\",\n  \"icons\": [\n    {},\n    {},\n    {},\n    {}\n  ]\n}}\n",
        json_string(&config.title),
        json_string(&config.title),
        json_string(&config.language),
        json_string(&root),
        json_string(&root),
        icon("favicon-32.png", "32x32"),
        icon("favicon-64.png", "64x64"),
        icon("logo.png", "512x512"),
        icon("apple-touch-icon.png", "180x180"),
    )
}

/// RFC 9116 security contact file. `Expires` is one year out.
fn security_txt(config: &Config) -> String {
    let contact = config
        .contact
        .clone()
        .or_else(|| (!config.url.is_empty()).then(|| config.url.clone()))
        .unwrap_or_else(|| "/".to_string());
    let expires = ymd_from_secs(now_secs() + 365 * 86_400);
    let mut out = format!("Contact: {contact}\nExpires: {expires}\n");
    if !config.language.is_empty() {
        out.push_str(&format!("Preferred-Languages: {}\n", config.language));
    }
    if !config.url.is_empty() {
        out.push_str(&format!(
            "Canonical: {}/.well-known/security.txt\n",
            config.url
        ));
    }
    out
}

fn browserconfig(config: &Config) -> String {
    let src = |name: &str| config.url(&format!("/assets/{name}"));
    format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
         <browserconfig>\n  <msapplication>\n    <tile>\n      <square70x70logo src=\"{}\"/>\n      <square150x150logo src=\"{}\"/>\n      <TileColor>#ffffff</TileColor>\n    </tile>\n  </msapplication>\n</browserconfig>\n",
        xml_escape(&src("favicon-32.png")),
        xml_escape(&src("logo.png")),
    )
}

/// OpenSearch descriptor. `pagers` has no built-in search yet — the template
/// points at the site root with `{searchTerms}` so browsers can still offer
/// "search this site" once a search endpoint exists.
fn opensearch(config: &Config) -> String {
    let root = if config.url.is_empty() {
        "/".to_string()
    } else {
        config.url.clone()
    };
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <OpenSearchDescription xmlns=\"http://a9.com/-/spec/opensearch/1.1/\">\n  <ShortName>{}</ShortName>\n  <Description>Search {}</Description>\n  <InputEncoding>UTF-8</InputEncoding>\n  <Url type=\"text/html\" method=\"get\" template=\"{}?q={{searchTerms}}\"/>\n</OpenSearchDescription>\n",
        xml_escape(&config.title),
        xml_escape(&config.title),
        xml_escape(&root),
    )
}

/// `keybase.txt` — Keybase verification stub. Set `keybase:` (username) and
/// `pgp:` (fingerprint) in site config, or replace the file entirely.
fn keybase_txt(config: &Config) -> String {
    let user = config.keybase.as_deref().unwrap_or("<username>");
    let mut out = format!(
        "# Keybase proof for {}\n# Verify at https://keybase.io/{}\n",
        config.title, user
    );
    if let Some(pgp) = &config.pgp {
        out.push_str(&format!("PGP fingerprint: {pgp}\n"));
    }
    out.push_str("# Replace this file with a signed keybase.txt, or delete it.\n");
    out
}

/// `pgpkey.txt` — ASCII-armored public key placeholder. Drop your real
/// armored key at this path (or in the content dir) to activate it.
fn pgpkey_txt(config: &Config) -> String {
    let mut out = format!("# PGP public key for {}\n", config.title);
    if let Some(pgp) = &config.pgp {
        out.push_str(&format!("# Fingerprint: {pgp}\n"));
    }
    out.push_str("# Replace with your ASCII-armored public key block.\n");
    out
}

/// `llms.txt` — llmstxt.org-style index of the site for LLM agents:
/// title, one-line description, then pages grouped by top-level section.
fn llms_txt(config: &Config, pages: &[Page], include_landing: bool) -> String {
    let mut out = format!("# {}\n\n> {}\n\n", config.title, config.title);
    if !config.url.is_empty() {
        out.push_str(&format!("Site: {}\n\n", config.url));
    }

    let page_url = |u: &str| {
        if config.url.is_empty() {
            config.url(u)
        } else {
            format!("{}{}", config.url, u)
        }
    };

    // Group pages by their first path segment; root-level pages get "Pages".
    let mut groups: Vec<(String, Vec<&Page>)> = Vec::new();
    for page in pages {
        let segment = page
            .url
            .trim_matches('/')
            .split('/')
            .next()
            .filter(|s| !s.is_empty())
            .unwrap_or("pages")
            .to_string();
        match groups.iter_mut().find(|(name, _)| *name == segment) {
            Some((_, list)) => list.push(page),
            None => groups.push((segment, vec![page])),
        }
    }
    groups.sort_by(|a, b| a.0.cmp(&b.0));

    if include_landing {
        out.push_str(&format!("## Overview\n\n- [Home]({})\n\n", page_url("/")));
    }
    for (name, list) in groups {
        out.push_str(&format!("## {}\n\n", title_case(&name)));
        for page in list {
            let label = if page.label.is_some() || !page.title.is_empty() {
                page.title.clone()
            } else {
                name.clone()
            };
            out.push_str(&format!("- [{}]({})\n", label, page_url(&page.url)));
        }
        out.push('\n');
    }
    out
}

/// `ai.txt` — plain-language crawler/AI policy pointer. Not a formal
/// standard, but a stable place to declare preferences alongside robots.txt.
fn ai_txt(config: &Config) -> String {
    format!(
        "# AI and crawler policy — {}\n\n\
         This site permits automated access for documentation, search and\n\
         assistant use. See robots.txt for crawler rules and sitemap.xml for\n\
         the page index; llms.txt provides an LLM-friendly outline.\n\n\
         Site: {}\n",
        config.title,
        if config.url.is_empty() {
            "/".to_string()
        } else {
            config.url.clone()
        },
    )
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// `YYYY-MM-DD` from a unix timestamp (civil-from-days algorithm).
fn ymd_from_secs(secs: u64) -> String {
    let z = secs / 86_400 + 719_468;
    let era = z / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

/// `YYYY-MM-DD` from a file's mtime, for sitemap `lastmod`.
fn file_ymd(content_dir: &str, source: &str) -> Option<String> {
    let modified = std::fs::metadata(Path::new(content_dir).join(source))
        .and_then(|m| m.modified())
        .ok()?;
    Some(ymd_from_secs(
        modified.duration_since(UNIX_EPOCH).ok()?.as_secs(),
    ))
}

fn title_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> Config {
        Config {
            title: "Docs".into(),
            language: "en".into(),
            base_path: "/docs".into(),
            theme: "light".into(),
            footer: None,
            url: "https://example.com/docs".into(),
            contact: Some("mailto:hi@example.com".into()),
            keybase: Some("alice".into()),
            pgp: Some("ABCD 1234".into()),
        }
    }

    fn page(url: &str) -> Page {
        Page {
            source: "a.md".into(),
            url: url.into(),
            output: "a/index.html".into(),
            title: "A".into(),
            title_from_heading: false,
            description: None,
            label: None,
            order: None,
            draft: false,
            dir: String::new(),
            content: String::new(),
        }
    }

    #[test]
    fn robots_includes_sitemap_only_with_url() {
        let mut c = config();
        c.url = String::new();
        assert!(!robots_txt(&c).contains("Sitemap:"));
        c.url = "https://example.com/docs".into();
        assert!(robots_txt(&c).contains("Sitemap: https://example.com/docs/sitemap.xml"));
    }

    #[test]
    fn sitemap_uses_site_url_including_base_path() {
        let xml = sitemap_xml("/nonexistent", &config(), &[page("/a/")], false);
        assert!(xml.contains("<loc>https://example.com/docs/a/</loc>"));
        assert!(xml.contains("</urlset>"));
    }

    #[test]
    fn sitemap_without_site_url_is_base_path_relative() {
        let mut c = config();
        c.url = String::new();
        let xml = sitemap_xml("/nonexistent", &c, &[page("/a/")], true);
        assert!(xml.contains("<loc>/docs/a/</loc>"));
        assert!(xml.contains("<loc>/docs/</loc>"));
    }

    #[test]
    fn favicon_ico_wraps_png() {
        let ico = favicon_ico();
        assert_eq!(&ico[0..4], &[0, 0, 1, 0]);
        assert_eq!(ico[6], 32);
        assert!(ico.windows(8).any(|w| w == b"\x89PNG\r\n\x1a\n"));
    }

    #[test]
    fn webmanifest_is_valid_json_shape() {
        let m = webmanifest(&config());
        assert!(m.contains("\"name\": \"Docs\""));
        assert!(m.contains("https://example.com/docs/assets/logo.png"));
    }

    #[test]
    fn security_txt_has_contact_and_expires() {
        let s = security_txt(&config());
        assert!(s.contains("Contact: mailto:hi@example.com"));
        assert!(s.contains("Expires: 20"));
    }

    #[test]
    fn llms_txt_groups_pages() {
        let llms = llms_txt(
            &config(),
            &[page("/guide/install/"), page("/index/")],
            false,
        );
        assert!(llms.contains("# Docs"));
        assert!(llms.contains("## Guide"));
        assert!(llms.contains("https://example.com/docs/guide/install/"));
    }

    #[test]
    fn xml_escape_covers_markup_chars() {
        assert_eq!(xml_escape("a&b<c>"), "a&amp;b&lt;c&gt;");
    }
}
