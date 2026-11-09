//! Link resolution: Markdown is authored against the content tree
//! (`../reference/cli.md`, `img/diagram.svg`) but published at trailing-slash
//! URLs under an optional base path, so internal destinations are rewritten
//! onto the URLs the build actually emits.

use std::collections::{HashMap, HashSet};

/// What a link destination resolved to.
pub enum Resolution {
    /// A page in the site; carries the published URL and the target's source
    /// path so the build can record a graph edge.
    Page { url: String, source: String },
    /// A non-Markdown asset copied through to the output.
    Asset { url: String },
    /// External, in-page or unresolvable: leave the destination untouched.
    Untouched,
}

/// Resolves destinations for links written inside one directory.
pub struct Resolver<'a> {
    /// Directory of the page holding the link, relative to the content root
    /// (`""` at the root).
    from: &'a str,
    pages: &'a HashMap<String, usize>,
    page_urls: &'a [String],
    page_sources: &'a [String],
    assets: &'a HashSet<String>,
    prefix: &'a dyn Fn(&str) -> String,
}

impl<'a> Resolver<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        from: &'a str,
        pages: &'a HashMap<String, usize>,
        page_urls: &'a [String],
        page_sources: &'a [String],
        assets: &'a HashSet<String>,
        prefix: &'a dyn Fn(&str) -> String,
    ) -> Self {
        Self {
            from,
            pages,
            page_urls,
            page_sources,
            assets,
            prefix,
        }
    }

    pub fn resolve(&self, destination: &str) -> Resolution {
        let (target, fragment) = split_fragment(destination);
        if target.is_empty() || is_external(target) {
            return Resolution::Untouched;
        }
        let source = clean_source(self.from, target);
        if let Some(&idx) = self.page_for(&source) {
            let url = format!("{}{}", (self.prefix)(&self.page_urls[idx]), fragment);
            return Resolution::Page {
                url,
                source: self.page_sources[idx].clone(),
            };
        }
        if self.assets.contains(&source) {
            return Resolution::Asset {
                url: format!("{}{}", (self.prefix)(&format!("/{source}")), fragment),
            };
        }
        Resolution::Untouched
    }

    /// Finds the page a cleaned source path refers to, trying the exact path
    /// then the conventional Markdown spellings (`x` → `x.md`,
    /// `x` → `x/index.md`).
    fn page_for(&self, source: &str) -> Option<&usize> {
        for candidate in [
            source.to_string(),
            format!("{source}.md"),
            format!("{source}.markdown"),
            format!("{source}/index.md"),
            format!("{source}/index.markdown"),
        ] {
            if let Some(idx) = self.pages.get(&candidate) {
                return Some(idx);
            }
        }
        None
    }
}

/// Turns a link destination into a path relative to the content root. A
/// leading `/` resolves from the root; anything else from the linking file's
/// directory.
fn clean_source(from: &str, target: &str) -> String {
    let joined = if let Some(rest) = target.strip_prefix('/') {
        rest.to_string()
    } else if from.is_empty() {
        target.to_string()
    } else {
        format!("{from}/{target}")
    };
    normalize(&joined)
}

/// Resolves `.` and `..` segments without touching the filesystem.
fn normalize(path: &str) -> String {
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

fn split_fragment(raw: &str) -> (&str, &str) {
    match raw.find('#') {
        Some(i) => (&raw[..i], &raw[i..]),
        None => (raw, ""),
    }
}

/// Reports whether a link leaves the site: a URI scheme or a
/// protocol-relative URL. Root-relative paths stay internal.
fn is_external(target: &str) -> bool {
    target.starts_with("//") || has_scheme(target)
}

/// Detects a URI scheme (`https:`, `mailto:`) before any `/`, `?` or `#`.
fn has_scheme(url: &str) -> bool {
    let Some(colon) = url.find(':') else {
        return false;
    };
    if colon == 0 || url[..colon].contains(['/', '?', '#']) {
        return false;
    }
    url[..colon].chars().enumerate().all(|(i, c)| {
        c.is_ascii_alphabetic()
            || (i > 0 && c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_schemes() {
        assert!(has_scheme("https://x.com"));
        assert!(has_scheme("mailto:a@b.c"));
        assert!(!has_scheme("guide/ports:80"));
        assert!(!has_scheme("./rel.md"));
    }

    #[test]
    fn normalizes_dot_segments() {
        assert_eq!(normalize("a/b/../c.md"), "a/c.md");
        assert_eq!(normalize("./x.md"), "x.md");
        assert_eq!(normalize("a/../../b.md"), "b.md");
    }

    #[test]
    fn resolves_relative_and_absolute() {
        assert_eq!(clean_source("guide", "../index.md"), "index.md");
        assert_eq!(clean_source("guide", "/ref/cli.md"), "ref/cli.md");
        assert_eq!(clean_source("", "a.md"), "a.md");
    }
}
