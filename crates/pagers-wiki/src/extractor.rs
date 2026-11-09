//! Article extraction: locates the main content element in the fetched
//! document so navigation, sidebars and page chrome never reach the
//! converter. Works on both Parsoid output (`body.mw-parser-output`) and
//! legacy parser output (`div.mw-parser-output`).

use scraper::{Html, Selector};

use crate::error::WikiError;

/// Extracts the inner HTML of the article body.
///
/// `title` is only used for error messages. Returns an error when no
/// plausible article container exists or it is empty.
pub fn extract_article_html(document_html: &str, title: &str) -> Result<String, WikiError> {
    let document = Html::parse_document(document_html);
    let article = select_article(&document).ok_or_else(|| {
        WikiError::Parse(format!(
            "no .mw-parser-output or <body> found for {title:?}"
        ))
    })?;
    let inner = article.inner_html();
    if inner.trim().is_empty() {
        return Err(WikiError::EmptyContent(title.to_string()));
    }
    Ok(inner)
}

/// Finds the article container: `.mw-parser-output` first (Parsoid puts the
/// class on `<body>` itself), then any `<body>` as a last resort.
fn select_article<'a>(document: &'a Html) -> Option<scraper::ElementRef<'a>> {
    let parser_output = Selector::parse(".mw-parser-output").ok()?;
    if let Some(el) = document.select(&parser_output).next() {
        return Some(el);
    }
    let body = Selector::parse("body").ok()?;
    document.select(&body).next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_parsoid_body_content() {
        let html =
            r#"<html><body class="mw-parser-output"><p>Hello <b>world</b></p></body></html>"#;
        let out = extract_article_html(html, "Test").unwrap();
        assert!(out.contains("Hello <b>world</b>"));
    }

    #[test]
    fn extracts_legacy_parser_output_div() {
        let html =
            r#"<html><body><div class="mw-parser-output"><p>Content</p></div></body></html>"#;
        let out = extract_article_html(html, "Test").unwrap();
        assert_eq!(out.trim(), "<p>Content</p>");
    }

    #[test]
    fn errors_on_missing_content() {
        let html = "<html><body></body></html>";
        assert!(matches!(
            extract_article_html(html, "Empty"),
            Err(WikiError::EmptyContent(_))
        ));
    }
}
