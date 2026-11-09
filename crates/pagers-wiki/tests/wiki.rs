//! Integration tests over local HTML fixtures — no network access.

use pagers_wiki::url::WikiArticle;
use pagers_wiki::{convert_html, ConvertOptions};

const RUST_URL: &str = "https://en.wikipedia.org/wiki/Rust_(programming_language)";

fn rust_article() -> WikiArticle {
    WikiArticle::parse(RUST_URL).unwrap()
}

fn rust_html() -> String {
    std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/rust.html"
    ))
    .unwrap()
}

fn convert(fragment_source: &str, opts: &ConvertOptions) -> pagers_wiki::metadata::Article {
    let article = rust_article();
    convert_html(&article, fragment_source, None, opts).unwrap()
}

#[test]
fn converts_full_article() {
    let article = convert(&rust_html(), &ConvertOptions::default());
    assert!(article
        .markdown
        .starts_with("# Rust (programming language)"));
    assert!(article.markdown.contains("## History"));
    assert!(article.markdown.contains("## Syntax and features"));
    // Site chrome must not leak into the output.
    assert!(!article.markdown.contains("Vector skin"));
    assert!(!article.markdown.contains("data-mw"));
    assert!(!article.markdown.contains("utm_"));
}

#[test]
fn links_become_markdown_files_with_source_url() {
    let article = convert(&rust_html(), &ConvertOptions::default());
    assert!(article.markdown.contains(
        "(general_purpose_programming_language.md \"https://en.wikipedia.org/wiki/General-purpose_programming_language\")"
    ));
}

#[test]
fn wiki_links_mode_keeps_absolute_urls() {
    let article = convert(
        &rust_html(),
        &ConvertOptions {
            local_links: false,
            ..Default::default()
        },
    );
    assert!(article
        .markdown
        .contains("(https://en.wikipedia.org/wiki/General-purpose_programming_language"));
}

#[test]
fn strips_references_by_default() {
    let article = convert(&rust_html(), &ConvertOptions::default());
    assert!(!article.markdown.contains("## Notes"));
    assert!(!article.markdown.contains("mw-references"));
}

#[test]
fn keeps_references_when_requested() {
    let article = convert(
        &rust_html(),
        &ConvertOptions {
            include_references: true,
            ..Default::default()
        },
    );
    assert!(article.markdown.contains("## Notes"));
    assert!(article.markdown.contains("cite_note"));
}

#[test]
fn converts_tables_to_gfm() {
    let html = r#"<html><body class="mw-parser-output"><table class="wikitable"><tbody>
        <tr><th>Name</th><th>Value</th></tr>
        <tr><td>alpha</td><td>1</td></tr>
        <tr><td>beta</td><td>2</td></tr></tbody></table></body></html>"#;
    let article = convert(html, &ConvertOptions::default());
    assert!(article.markdown.contains("| Name | Value |"));
    assert!(article.markdown.contains("| --- | --- |"));
    assert!(article.markdown.contains("| alpha | 1 |"));
}

#[test]
fn preserves_nested_lists() {
    let html = r#"<html><body class="mw-parser-output"><ul>
        <li>one<ul><li>nested</li></ul></li>
        <li>two</li></ul></body></html>"#;
    let article = convert(html, &ConvertOptions::default());
    assert!(article.markdown.contains("-   one"));
    assert!(article.markdown.contains("nested"));
}

#[test]
fn converts_math_to_latex() {
    let html = r#"<html><body class="mw-parser-output"><p>Energy 
        <span class="mwe-math-element" typeof="mw:Extension/math">
        <math><semantics><annotation encoding="application/x-tex">E=mc^2</annotation></semantics></math>
        </span> here.</p></body></html>"#;
    let article = convert(html, &ConvertOptions::default());
    assert!(article.markdown.contains("$E=mc^2$"));
}

#[test]
fn preserves_code_blocks() {
    let html = r#"<html><body class="mw-parser-output"><pre><code class="language-rust">fn main() {}</code></pre></body></html>"#;
    let article = convert(html, &ConvertOptions::default());
    assert!(article.markdown.contains("fn main() {}"));
}

#[test]
fn removes_scripts_and_edit_links() {
    let html = r#"<html><body class="mw-parser-output"><p>Text</p>
        <span class="mw-editsection"><a href="./Foo?action=edit">edit</a></span>
        <script>alert(1)</script></body></html>"#;
    let article = convert(html, &ConvertOptions::default());
    assert!(!article.markdown.contains("edit"));
    assert!(!article.markdown.contains("alert"));
}

#[test]
fn errors_on_empty_content() {
    let html = "<html><body class=\"mw-parser-output\"></body></html>";
    let article = rust_article();
    let err = convert_html(&article, html, None, &ConvertOptions::default()).unwrap_err();
    assert!(matches!(
        err,
        pagers_wiki::error::WikiError::EmptyContent(_)
    ));
}

#[test]
fn non_english_titles_survive() {
    let article = WikiArticle::parse("https://de.wikipedia.org/wiki/Programmiersprache").unwrap();
    assert_eq!(article.language, "de");
    assert_eq!(article.title, "Programmiersprache");
}
