use pagers_wiki::{convert_html, url::WikiArticle, ConvertOptions};

fn main() {
    let html = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/rust.html"
    ))
    .unwrap();
    let article =
        WikiArticle::parse("https://en.wikipedia.org/wiki/Rust_(programming_language)").unwrap();
    let refs = std::env::args().any(|a| a == "--refs");
    let art = convert_html(
        &article,
        &html,
        None,
        &ConvertOptions {
            include_references: refs,
            ..Default::default()
        },
    )
    .unwrap();
    println!("{}", art.markdown);
    eprintln!(
        "--- words: {} chars: {}",
        art.word_count(),
        art.markdown.len()
    );
}
