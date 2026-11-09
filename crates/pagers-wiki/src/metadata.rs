//! Article metadata and the structured result type shared by `convert`
//! and `info`.

use serde::Serialize;
use serde_json::{json, Value};

use crate::client::PageInfo;
use crate::url::WikiArticle;

/// Everything known about the source article.
#[derive(Debug, Clone, Serialize)]
pub struct ArticleMetadata {
    pub title: String,
    pub url: String,
    pub language: String,
    /// MediaWiki revision ID of the fetched version, when the API reports it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<u64>,
    /// Revision timestamp (ISO 8601), when reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_timestamp: Option<String>,
}

impl ArticleMetadata {
    /// Suggested Markdown filename derived from the article title:
    /// `Rust (programming language)` → `rust_programming_language.md`.
    pub fn file_name(&self) -> String {
        format!("{}.md", file_slug(&self.title))
    }

    /// Human-readable metadata block for `wiki info`.
    pub fn info_text(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("Title:    {}\n", self.title));
        out.push_str(&format!("URL:      {}\n", self.url));
        out.push_str(&format!("Language: {}\n", self.language));
        if let Some(id) = self.revision_id {
            out.push_str(&format!("Revision: {id}\n"));
        }
        if let Some(ts) = &self.revision_timestamp {
            out.push_str(&format!("Modified: {ts}\n"));
        }
        out
    }

    /// Builds metadata from the parsed URL and optional REST page info.
    /// Never invents values: missing API fields stay `None`.
    pub fn from_parts(article: &WikiArticle, info: Option<&PageInfo>) -> Self {
        let (revision_id, revision_timestamp) = info
            .and_then(|i| i.latest.as_ref())
            .map(|l| (l.id, l.timestamp.clone()))
            .unwrap_or((None, None));
        Self {
            title: info
                .map(|i| i.title.clone())
                .filter(|t| !t.is_empty())
                .unwrap_or_else(|| article.title.clone()),
            url: article.article_url.clone(),
            language: article.language.clone(),
            revision_id,
            revision_timestamp,
        }
    }
}

/// Slugifies an article title for use as a Markdown filename:
/// lowercase, non-alphanumeric runs collapse to `_`, unicode letters kept.
pub fn file_slug(title: &str) -> String {
    let mut slug = String::with_capacity(title.len());
    let mut last_sep = true; // trim leading separators
    for ch in title.chars().flat_map(char::to_lowercase) {
        if ch.is_alphanumeric() {
            slug.push(ch);
            last_sep = false;
        } else if !last_sep {
            slug.push('_');
            last_sep = true;
        }
    }
    while slug.ends_with('_') {
        slug.pop();
    }
    if slug.is_empty() {
        slug.push_str("article");
    }
    slug
}

/// The result of a successful conversion.
#[derive(Debug)]
pub struct Article {
    pub metadata: ArticleMetadata,
    pub markdown: String,
}

impl Article {
    /// Word count over the Markdown body (whitespace-separated tokens).
    pub fn word_count(&self) -> usize {
        self.markdown.split_whitespace().count()
    }

    /// Serializes to the `--format json` document shape.
    pub fn to_json(&self) -> Value {
        json!({
            "title": self.metadata.title,
            "url": self.metadata.url,
            "language": self.metadata.language,
            "revision_id": self.metadata.revision_id,
            "revision_timestamp": self.metadata.revision_timestamp,
            "markdown": self.markdown,
            "file_name": self.metadata.file_name(),
            "metadata": {
                "word_count": self.word_count(),
                "character_count": self.markdown.chars().count(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn meta(title: &str) -> ArticleMetadata {
        ArticleMetadata {
            title: title.to_string(),
            url: String::new(),
            language: "en".to_string(),
            revision_id: None,
            revision_timestamp: None,
        }
    }

    #[test]
    fn file_name_slugifies_title() {
        assert_eq!(
            meta("Rust (programming language)").file_name(),
            "rust_programming_language.md"
        );
        assert_eq!(meta("Café").file_name(), "café.md");
        assert_eq!(meta("  --Weird__Title!! ").file_name(), "weird_title.md");
        assert_eq!(meta("").file_name(), "article.md");
    }
}
