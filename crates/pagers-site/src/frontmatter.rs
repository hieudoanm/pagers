//! Frontmatter parsing: a leading `---` delimited block of flat `key: value`
//! pairs. Every field is optional; unknown keys are ignored so authors can
//! carry extra metadata without breaking the build.

/// Page metadata read from a leading YAML-style block.
#[derive(Debug, Default, Clone)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub description: Option<String>,
    pub order: Option<i64>,
    pub draft: bool,
    pub label: Option<String>,
    // Site-wide fields, only honoured on the index page.
    pub language: Option<String>,
    pub base_path: Option<String>,
    pub theme: Option<String>,
    pub footer: Option<String>,
    pub url: Option<String>,
    pub contact: Option<String>,
    pub keybase: Option<String>,
    pub pgp: Option<String>,
}

impl Frontmatter {
    /// `title`, falling back to a derived value when unset.
    pub fn title_or(&self, fallback: &str) -> String {
        self.title.clone().unwrap_or_else(|| fallback.to_string())
    }

    /// Whether the block carries any site-wide field.
    pub fn has_site_fields(&self) -> bool {
        self.language.is_some()
            || self.base_path.is_some()
            || self.theme.is_some()
            || self.footer.is_some()
            || self.url.is_some()
            || self.contact.is_some()
            || self.keybase.is_some()
            || self.pgp.is_some()
    }
}

/// Splits `src` into frontmatter and body. Input without a complete fence is
/// returned unchanged with `None`, matching the convention that an unclosed
/// `---` is a thematic break, not metadata.
pub fn split(src: &str) -> Result<(Option<Frontmatter>, &str), String> {
    let Some(after_open) = src.strip_prefix("---") else {
        return Ok((None, src));
    };
    // The opening fence must end the line.
    let after_open = after_open
        .strip_prefix('\n')
        .or_else(|| after_open.strip_prefix("\r\n"))
        .unwrap_or(after_open);
    let Some(close) = find_closing_fence(after_open) else {
        return Ok((None, src));
    };
    let (block, body) = after_open.split_at(close);
    let fm = parse_block(block)?;
    Ok((Some(fm), body.trim_start_matches(['\n', '\r'])))
}

/// Finds the byte offset just past the closing `---` fence line.
fn find_closing_fence(src: &str) -> Option<usize> {
    let mut offset = 0;
    for line in src.split_inclusive('\n') {
        let trimmed = line.trim_end_matches(['\n', '\r']);
        if trimmed == "---" {
            return Some(offset + line.len());
        }
        offset += line.len();
    }
    None
}

/// Parses a flat `key: value` block. Lines without a colon and blank lines
/// are skipped; values may be single- or double-quoted.
pub fn parse_block(block: &str) -> Result<Frontmatter, String> {
    let mut fm = Frontmatter::default();
    for line in block.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim().to_lowercase();
        let value = unquote(value.trim());
        match key.as_str() {
            "title" => fm.title = non_empty(&value),
            "description" => fm.description = non_empty(&value),
            "order" => fm.order = value.parse().ok(),
            "draft" => fm.draft = matches!(value.to_lowercase().as_str(), "true" | "yes" | "1"),
            "label" => fm.label = non_empty(&value),
            "language" => fm.language = non_empty(&value),
            "basepath" | "base_path" => fm.base_path = non_empty(&value),
            "theme" => fm.theme = non_empty(&value),
            "footer" => fm.footer = non_empty(&value),
            "url" | "site_url" | "siteurl" => fm.url = non_empty(&value),
            "contact" => fm.contact = non_empty(&value),
            "keybase" => fm.keybase = non_empty(&value),
            "pgp" | "pgpkey" | "pgp_fingerprint" => fm.pgp = non_empty(&value),
            _ => {}
        }
    }
    Ok(fm)
}

fn non_empty(value: &str) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn unquote(value: &str) -> String {
    let bytes = value.as_bytes();
    if bytes.len() >= 2
        && ((bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\''))
    {
        return value[1..value.len() - 1].to_string();
    }
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_flat_frontmatter() {
        let src = "---\ntitle: Hello\norder: 2\ndraft: true\n---\n\n# Body\n";
        let (fm, body) = split(src).unwrap();
        let fm = fm.unwrap();
        assert_eq!(fm.title.as_deref(), Some("Hello"));
        assert_eq!(fm.order, Some(2));
        assert!(fm.draft);
        assert_eq!(body, "# Body\n");
    }

    #[test]
    fn returns_source_when_no_fence() {
        let (fm, body) = split("# Just markdown\n").unwrap();
        assert!(fm.is_none());
        assert_eq!(body, "# Just markdown\n");
    }

    #[test]
    fn unclosed_fence_is_not_frontmatter() {
        let src = "---\ntitle: nope\n";
        let (fm, body) = split(src).unwrap();
        assert!(fm.is_none());
        assert_eq!(body, src);
    }

    #[test]
    fn strips_quotes() {
        assert_eq!(unquote("\"hi there\""), "hi there");
        assert_eq!(unquote("'hi'"), "hi");
        assert_eq!(unquote("plain"), "plain");
    }
}
