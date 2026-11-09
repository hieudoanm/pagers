//! MediaWiki REST API client: fetches the rendered article HTML and page
//! metadata with retries, timeout and a descriptive User-Agent.

use std::time::Duration;

use serde::Deserialize;

use crate::error::WikiError;
use crate::url::WikiArticle;

const USER_AGENT: &str = concat!("pagers/", env!("CARGO_PKG_VERSION"), " (wiki-to-markdown)");
const MAX_ATTEMPTS: u32 = 3;
const TIMEOUT: Duration = Duration::from_secs(30);

/// Article metadata returned by the REST `page/{title}` endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct PageInfo {
    pub title: String,
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub latest: Option<LatestRevision>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LatestRevision {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub timestamp: Option<String>,
}

/// A reusable HTTP client tuned for Wikipedia requests.
pub struct WikiClient {
    http: reqwest::blocking::Client,
}

impl WikiClient {
    pub fn new() -> Result<Self, WikiError> {
        let http = reqwest::blocking::Client::builder()
            .user_agent(USER_AGENT)
            .timeout(TIMEOUT)
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()
            .map_err(|e| WikiError::Network(e.to_string()))?;
        Ok(Self { http })
    }

    /// Fetches the rendered article HTML (`page/{title}/html`).
    pub fn fetch_html(&self, article: &WikiArticle) -> Result<String, WikiError> {
        let url = format!("{}{}/html", article.api_base, article.api_title());
        self.get_text(&url)
    }

    /// Fetches page metadata (`page/{title}`); failures degrade to `None`
    /// rather than failing the conversion.
    pub fn fetch_info(&self, article: &WikiArticle) -> Option<PageInfo> {
        let url = format!("{}{}", article.api_base, article.api_title());
        let response = self.try_get(&url).ok()??;
        serde_json::from_str(&response).ok()
    }

    fn get_text(&self, url: &str) -> Result<String, WikiError> {
        let mut attempt = 0;
        let mut saw_rate_limit = false;
        loop {
            attempt += 1;
            match self.try_get(url) {
                Ok(Some(body)) => return Ok(body),
                Ok(None) => saw_rate_limit = true,
                Err(e) => {
                    if attempt >= MAX_ATTEMPTS || !e.is_retryable() {
                        return Err(e);
                    }
                }
            }
            if attempt >= MAX_ATTEMPTS {
                return Err(if saw_rate_limit {
                    WikiError::RateLimited {
                        url: url.to_string(),
                    }
                } else {
                    WikiError::Network(format!("gave up after {MAX_ATTEMPTS} attempts"))
                });
            }
            std::thread::sleep(Duration::from_millis(400 * attempt as u64));
        }
    }

    /// One GET attempt. `Ok(None)` means a retryable status; `Err` is final.
    fn try_get(&self, url: &str) -> Result<Option<String>, WikiError> {
        let response = self
            .http
            .get(url)
            .send()
            .map_err(|e| WikiError::Network(format!("{url}: {e}")))?;
        let status = response.status();
        match status.as_u16() {
            200..=299 => response
                .text()
                .map(Some)
                .map_err(|e| WikiError::Network(format!("read {url}: {e}"))),
            404 => Err(WikiError::NotFound(url.to_string())),
            429 => Ok(None),
            403 => Err(WikiError::Http {
                status: 403,
                url: url.to_string(),
            }),
            s if s >= 500 => Err(WikiError::Http {
                status: s,
                url: url.to_string(),
            }),
            s => Err(WikiError::Http {
                status: s,
                url: url.to_string(),
            }),
        }
    }
}

trait Retryable {
    fn is_retryable(&self) -> bool;
}

impl Retryable for WikiError {
    fn is_retryable(&self) -> bool {
        match self {
            WikiError::Network(_) => true,
            WikiError::Http { status, .. } => *status >= 500,
            _ => false,
        }
    }
}
