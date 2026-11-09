//! Site-wide configuration: site fields from the index page's frontmatter,
//! falling back to `pagers.yaml`, then to defaults derived from the directory
//! name. The file uses the same flat `key: value` shape as frontmatter.

use std::path::Path;

use crate::frontmatter::{self, Frontmatter};

pub const CONFIG_FILE: &str = "pagers.yaml";

#[derive(Debug, Clone)]
pub struct Config {
    pub title: String,
    pub language: String,
    pub base_path: String,
    pub theme: String,
    pub footer: Option<String>,
    /// Absolute site root URL (e.g. `https://example.com/docs`), used for
    /// sitemap/robots URLs. Empty when not configured.
    pub url: String,
    /// Security contact for `.well-known/security.txt` (URL or mailto:).
    pub contact: Option<String>,
    /// Keybase username for `keybase.txt`.
    pub keybase: Option<String>,
    /// OpenPGP fingerprint for `keybase.txt` / `pgpkey.txt` stubs.
    pub pgp: Option<String>,
}

impl Config {
    /// Applies the base path to a site-absolute URL (`/guide/` →
    /// `/docs/guide/`). Every generated URL passes through here.
    pub fn url(&self, target: &str) -> String {
        if self.base_path.is_empty() {
            return target.to_string();
        }
        if target == "/" {
            return format!("{}/", self.base_path);
        }
        format!("{}{}", self.base_path, target)
    }
}

/// Loads config for a content directory without a full discovery pass —
/// used by `serve` to learn the base path before serving.
pub fn load_for_dir(content_dir: &str) -> Result<Config, String> {
    let mut index_fm = None;
    for name in ["index.md", "index.markdown", "README.md", "README.markdown"] {
        let path = Path::new(content_dir).join(name);
        if let Ok(raw) = std::fs::read_to_string(&path) {
            index_fm = frontmatter::split(&raw).ok().and_then(|(fm, _)| fm);
            if index_fm.is_some() {
                break;
            }
        }
    }
    load(content_dir, index_fm.as_ref())
}

/// Loads config: index frontmatter site fields win, then `pagers.yaml`, then
/// defaults. `index_fm` is the frontmatter of the root `index.md`/`README.md`.
pub fn load(content_dir: &str, index_fm: Option<&Frontmatter>) -> Result<Config, String> {
    if let Some(fm) = index_fm {
        if fm.has_site_fields() {
            return Ok(from_frontmatter(fm, content_dir));
        }
    }
    let path = Path::new(content_dir).join(CONFIG_FILE);
    match std::fs::read_to_string(&path) {
        Ok(raw) => {
            let fm = frontmatter::parse_block(&raw)?;
            Ok(from_frontmatter(&fm, content_dir))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(defaults(content_dir)),
        Err(e) => Err(format!("read {}: {e}", path.display())),
    }
}

fn from_frontmatter(fm: &Frontmatter, content_dir: &str) -> Config {
    Config {
        title: fm
            .title
            .clone()
            .unwrap_or_else(|| default_title(content_dir)),
        language: fm.language.clone().unwrap_or_else(|| "en".to_string()),
        base_path: normalize_base_path(fm.base_path.as_deref().unwrap_or("")),
        theme: fm.theme.clone().unwrap_or_else(|| "light".to_string()),
        footer: fm.footer.clone(),
        url: fm
            .url
            .clone()
            .unwrap_or_default()
            .trim_end_matches('/')
            .to_string(),
        contact: fm.contact.clone(),
        keybase: fm.keybase.clone(),
        pgp: fm.pgp.clone(),
    }
}

fn defaults(content_dir: &str) -> Config {
    Config {
        title: default_title(content_dir),
        language: "en".to_string(),
        base_path: String::new(),
        theme: "light".to_string(),
        footer: None,
        url: String::new(),
        contact: None,
        keybase: None,
        pgp: None,
    }
}

fn default_title(content_dir: &str) -> String {
    std::fs::canonicalize(content_dir)
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_else(|| "Documentation".to_string())
}

fn normalize_base_path(base: &str) -> String {
    let trimmed = base.trim_matches('/');
    if trimmed.is_empty() {
        String::new()
    } else {
        format!("/{trimmed}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_prefixes_base_path() {
        let c = Config {
            title: "t".into(),
            language: "en".into(),
            base_path: "/docs".into(),
            theme: "light".into(),
            footer: None,
            url: String::new(),
            contact: None,
            keybase: None,
            pgp: None,
        };
        assert_eq!(c.url("/"), "/docs/");
        assert_eq!(c.url("/guide/"), "/docs/guide/");
    }

    #[test]
    fn url_without_base_path_is_identity() {
        let c = defaults("docs");
        assert_eq!(c.url("/guide/"), "/guide/");
    }
}
