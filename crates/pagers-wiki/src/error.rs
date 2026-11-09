//! Error type for the wiki pipeline. Every failure mode the converter can
//! hit maps to a variant with an actionable message.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WikiError {
    #[error("invalid Wikipedia URL {0:?}: expected https://<lang>.wikipedia.org/wiki/<title>")]
    InvalidUrl(String),

    #[error("unsupported domain in {0:?}: only *.wikipedia.org article URLs are supported")]
    UnsupportedDomain(String),

    #[error("invalid article title {0:?}: title is empty or contains unsupported characters")]
    InvalidTitle(String),

    #[error("article not found: {0}")]
    NotFound(String),

    #[error("article {0} returned no readable content")]
    EmptyContent(String),

    #[error("Wikipedia request failed after retries: {0}")]
    Network(String),

    #[error("Wikipedia returned HTTP {status} for {url}")]
    Http { status: u16, url: String },

    #[error("Wikipedia rate limited the request for {url}; retry later")]
    RateLimited { url: String },

    #[error("failed to parse article HTML: {0}")]
    Parse(String),

    #[error("Markdown conversion failed: {0}")]
    Conversion(String),

    #[error("failed to write {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
}
