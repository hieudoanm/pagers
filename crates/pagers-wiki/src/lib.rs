//! pagers-wiki: converts a Wikipedia article into clean, well-structured
//! Markdown. The pipeline is staged so other content sources can reuse the
//! later stages:
//!
//! ```text
//! URL → validate → REST API (HTML + metadata) → extract .mw-parser-output
//!     → clean DOM → htmd → normalise → Markdown / JSON
//! ```

pub mod client;
pub mod converter;
pub mod error;
pub mod extractor;
pub mod metadata;
pub mod normalizer;
pub mod url;

use client::WikiClient;
use converter::CleanContext;
use error::WikiError;
use metadata::{Article, ArticleMetadata};
use url::WikiArticle;

/// Options controlling a conversion.
#[derive(Debug, Clone)]
pub struct ConvertOptions {
    /// Keep the references/citations section and inline reference markers.
    pub include_references: bool,
    /// Rewrite internal `/wiki/` links to `<slug>.md` filenames, keeping the
    /// absolute article URL in the link title (`[t](f.md "https://…")`).
    pub local_links: bool,
}

impl Default for ConvertOptions {
    fn default() -> Self {
        Self {
            include_references: false,
            local_links: true,
        }
    }
}

/// Converts a Wikipedia article URL to Markdown.
pub fn convert(raw_url: &str, options: &ConvertOptions) -> Result<Article, WikiError> {
    let article = WikiArticle::parse(raw_url)?;
    let client = WikiClient::new()?;
    fetch_and_convert(&client, &article, options)
}

/// Fetches article metadata only (no HTML download).
pub fn info(raw_url: &str) -> Result<ArticleMetadata, WikiError> {
    let article = WikiArticle::parse(raw_url)?;
    let client = WikiClient::new()?;
    let page_info = client.fetch_info(&article);
    Ok(ArticleMetadata::from_parts(&article, page_info.as_ref()))
}

/// Shared pipeline used by `convert`: fetch HTML + info, extract, clean,
/// convert, normalise.
fn fetch_and_convert(
    client: &WikiClient,
    article: &WikiArticle,
    options: &ConvertOptions,
) -> Result<Article, WikiError> {
    let html = client.fetch_html(article)?;
    let page_info = client.fetch_info(article);
    convert_html(article, &html, page_info.as_ref(), options)
}

/// Runs extraction → conversion → normalisation on already-fetched HTML.
/// Exposed for tests and for callers that fetched the document themselves.
pub fn convert_html(
    article: &WikiArticle,
    document_html: &str,
    page_info: Option<&client::PageInfo>,
    options: &ConvertOptions,
) -> Result<Article, WikiError> {
    let fragment = extractor::extract_article_html(document_html, &article.title)?;
    let ctx = CleanContext {
        article,
        include_references: options.include_references,
        local_links: options.local_links,
    };
    let markdown = converter::to_markdown(&fragment, &ctx)?;
    let normalized = normalizer::normalize(&markdown, &article.title);
    Ok(Article {
        metadata: ArticleMetadata::from_parts(article, page_info),
        markdown: normalized,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_url() {
        assert!(matches!(
            convert("https://example.com/x", &ConvertOptions::default()),
            Err(WikiError::InvalidUrl(_)) | Err(WikiError::UnsupportedDomain(_))
        ));
    }
}
