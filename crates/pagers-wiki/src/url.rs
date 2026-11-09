//! Wikipedia URL parsing and validation.
//!
//! Accepts `https://<lang>.wikipedia.org/wiki/<title>` (and `www.`/`wikipedia.org`
//! variants), including percent-encoded titles. Produces the REST API base, the
//! canonical article URL and the MediaWiki page key used by the API.

use url::Url;

use crate::error::WikiError;

/// A validated Wikipedia article reference.
#[derive(Debug, Clone)]
pub struct WikiArticle {
    /// Language subdomain, e.g. `en`.
    pub language: String,
    /// Human-readable title with underscores as spaces, e.g. `Rust (programming language)`.
    pub title: String,
    /// Canonical article URL on the wiki, e.g. `https://en.wikipedia.org/wiki/Rust_(programming_language)`.
    pub article_url: String,
    /// REST API base for this wiki, e.g. `https://en.wikipedia.org/w/rest.php/v1/page/`.
    pub api_base: String,
    /// Wiki root used to absolutize relative links, e.g. `https://en.wikipedia.org`.
    pub wiki_root: String,
}

impl WikiArticle {
    /// Parses and validates a Wikipedia article URL.
    pub fn parse(raw: &str) -> Result<Self, WikiError> {
        let url = Url::parse(raw.trim()).map_err(|_| WikiError::InvalidUrl(raw.to_string()))?;
        if !matches!(url.scheme(), "http" | "https") {
            return Err(WikiError::InvalidUrl(raw.to_string()));
        }
        let host = url
            .host_str()
            .ok_or_else(|| WikiError::InvalidUrl(raw.to_string()))?
            .to_ascii_lowercase();
        let language =
            wiki_language(&host).ok_or_else(|| WikiError::UnsupportedDomain(raw.to_string()))?;

        let segments: Vec<&str> = url.path_segments().map(|s| s.collect()).unwrap_or_default();
        if segments.len() < 2 || segments[0] != "wiki" {
            return Err(WikiError::InvalidUrl(raw.to_string()));
        }
        let key = percent_decode(segments[1]);
        let title = key.replace('_', " ").trim().to_string();
        if title.is_empty() || title.contains('/') {
            return Err(WikiError::InvalidTitle(title));
        }

        let wiki_root = format!("https://{host}");
        let api_base = format!("{wiki_root}/w/rest.php/v1/page/");
        let article_url = format!("{wiki_root}/wiki/{}", key);
        Ok(Self {
            language,
            title,
            article_url,
            api_base,
            wiki_root,
        })
    }

    /// URL-encodes the title for use as a single REST API path segment.
    pub fn api_title(&self) -> String {
        self.title.replace(' ', "_")
    }
}

/// Extracts the language subdomain from a wikipedia.org host.
/// `en.wikipedia.org` → `en`, `en.m.wikipedia.org` → `en`,
/// `wikipedia.org`/`www.wikipedia.org` → `en`.
fn wiki_language(host: &str) -> Option<String> {
    let labels: Vec<&str> = host.split('.').collect();
    if labels.len() < 3 || labels[labels.len() - 2..] != ["wikipedia", "org"] {
        return None;
    }
    match labels[0] {
        "www" | "wikipedia" => None,
        lang => Some(lang.to_string()),
    }
}

/// Percent-decodes a URL path segment (`+` stays literal, as in paths).
fn percent_decode(segment: &str) -> String {
    let bytes = segment.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(v) = u8::from_str_radix(&segment[i + 1..i + 3], 16) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_standard_url() {
        let a = WikiArticle::parse("https://en.wikipedia.org/wiki/Rust_(programming_language)")
            .unwrap();
        assert_eq!(a.language, "en");
        assert_eq!(a.title, "Rust (programming language)");
        assert_eq!(a.api_title(), "Rust_(programming_language)");
        assert_eq!(a.api_base, "https://en.wikipedia.org/w/rest.php/v1/page/");
    }

    #[test]
    fn parses_percent_encoded_title() {
        let a = WikiArticle::parse("https://fr.wikipedia.org/wiki/Caf%C3%A9").unwrap();
        assert_eq!(a.language, "fr");
        assert_eq!(a.title, "Café");
    }

    #[test]
    fn rejects_non_wiki_hosts() {
        assert!(matches!(
            WikiArticle::parse("https://example.com/wiki/Foo"),
            Err(WikiError::UnsupportedDomain(_))
        ));
        assert!(matches!(
            WikiArticle::parse("https://en.wikipedia.org/"),
            Err(WikiError::InvalidUrl(_))
        ));
        assert!(matches!(
            WikiArticle::parse("not a url"),
            Err(WikiError::InvalidUrl(_))
        ));
    }
}
