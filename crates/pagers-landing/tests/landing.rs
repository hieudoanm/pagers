//! Behaviour tests ported from the Go `internal/landify/*_test.go` suites.

use pagers_landing::build;
use pagers_landing::color;
use pagers_landing::config::{Config, Theme};
use pagers_landing::load;
use pagers_landing::og;
use pagers_landing::placeholder;
use pagers_landing::themes;
use pagers_landing::validate;
use std::io::Write;

fn example(name: &str) -> String {
    format!(
        "{}/examples/example-{name}.yaml",
        env!("CARGO_MANIFEST_DIR")
    )
}

#[test]
fn known_types_has_twelve() {
    assert_eq!(validate::KNOWN_TYPES.len(), 12);
    assert!(validate::is_known_type("product"));
    assert!(!validate::is_known_type("blog"));
}

#[test]
fn normalize_type_defaults_to_product() {
    assert_eq!(validate::normalize_type(""), "product");
    assert_eq!(validate::normalize_type("product"), "product");
    assert_eq!(validate::normalize_type("FAQ"), "FAQ");
}

#[test]
fn every_example_validates() {
    for name in validate::KNOWN_TYPES {
        let cfg = load::load_file(std::path::Path::new(&example(name))).unwrap();
        let errs = validate::errors(&cfg);
        assert!(errs.is_empty(), "{name}: {errs:?}");
    }
}

#[test]
fn unknown_yaml_field_is_rejected() {
    let yaml = b"site:\n  name: X\n  bogus: 1\n";
    assert!(load::load(yaml).is_err());
}

#[test]
fn theme_defaults_merge() {
    let cfg = load::load(b"site:\n  name: X\n").unwrap();
    assert_eq!(cfg.theme.primary, Theme::default().primary);
    assert_eq!(cfg.theme.radius, "10px");
}

#[test]
fn tokens_derive_from_base_colors() {
    let t = Theme::default();
    let tokens = color::tokens(&t).unwrap();
    assert_eq!(tokens["base-100"], t.base);
    assert_eq!(tokens["primary"], t.primary);
    assert_eq!(tokens["radius"], t.radius);
    assert!(tokens.contains_key("base-content"));
    assert!(tokens.contains_key("primary-dark"));
    assert!(tokens.contains_key("primary-soft"));
    assert!(tokens.contains_key("border-soft"));
    for v in tokens.values() {
        if v != &t.radius {
            assert!(v.starts_with('#'), "token value {v} not hex");
        }
    }
}

#[test]
fn invalid_hex_is_an_error() {
    let t = Theme {
        primary: "blue".into(),
        ..Theme::default()
    };
    assert!(color::tokens(&t).is_err());
}

#[test]
fn contrast_ratio_black_white_is_21() {
    let r = color::contrast_ratio("#000000", "#ffffff").unwrap();
    assert!((r - 21.0).abs() < 0.01);
}

#[test]
fn readable_on_lightens_dark_text() {
    // Near-black on white already passes; a mid gray on white must shift.
    let c = color::readable_on("#777777", "#ffffff", 4.5).unwrap();
    let r = color::contrast_ratio(&c, "#ffffff").unwrap();
    assert!(r >= 4.5);
}

#[test]
fn exactly_64_themes() {
    assert_eq!(themes::themes().len(), 64);
    assert_eq!(themes::theme_names().len(), 64);
}

#[test]
fn theme_lookup_is_case_insensitive() {
    let t = themes::theme_by_name("OCEAN").unwrap();
    assert_eq!(t.primary, "#0e7490");
    assert!(themes::theme_by_name("nope").is_none());
}

#[test]
fn placeholder_for_each_type() {
    for name in validate::KNOWN_TYPES {
        let data = placeholder::placeholder(name).unwrap();
        assert!(data.contains("site:"), "{name} scaffold missing site");
        let cfg = load::load(data.as_bytes()).unwrap();
        assert_eq!(validate::normalize_type(&cfg.page_type), *name);
    }
    assert!(placeholder::placeholder("bogus").is_err());
}

#[test]
fn render_all_types() {
    for name in validate::KNOWN_TYPES {
        let cfg = load::load_file(std::path::Path::new(&example(name))).unwrap();
        let html = build::render(&cfg).unwrap();
        assert!(html.contains("<!doctype html>"), "{name} missing doctype");
        assert!(!html.contains("{{"), "{name} has unrendered template tags");
        assert!(html.contains("--primary:"), "{name} missing theme tokens");
    }
}

#[test]
fn unknown_type_falls_back_to_product() {
    let mut cfg = load::load_file(std::path::Path::new(&example("product"))).unwrap();
    cfg.page_type = "bogus".into();
    let html = build::render(&cfg).unwrap();
    assert!(html.contains("demo-media"));
}

#[test]
fn build_file_writes_page_and_og() {
    let dir = std::env::temp_dir().join(format!("pagers-landing-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let yaml = dir.join("landify.yaml");
    let out = dir.join("public/index.html");
    std::fs::write(&yaml, std::fs::read(example("product")).unwrap()).unwrap();
    let result = build::build_file(&yaml, &out, "").unwrap();
    assert!(out.exists());
    assert_eq!(result.page, out);
    // example-product has a site.og block → card written beside the page.
    if let Some(card) = result.card {
        assert!(card.exists());
        assert!(card.ends_with("og/og.svg"));
    }
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn build_file_rejects_invalid_config() {
    let dir = std::env::temp_dir().join(format!("pagers-landing-bad-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let yaml = dir.join("landify.yaml");
    let mut f = std::fs::File::create(&yaml).unwrap();
    writeln!(f, "type: bogus\nsite:\n  name: X\n").unwrap();
    let err = build::build_file(&yaml, &dir.join("index.html"), "").unwrap_err();
    assert!(err.to_string().contains("not supported"));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn theme_flag_overrides_yaml() {
    let cfg = load::render_config(std::path::Path::new(&example("product")), "slate").unwrap();
    assert_eq!(cfg.theme.primary, "#1e293b");
    assert!(load::render_config(std::path::Path::new(&example("product")), "nope").is_err());
}

#[test]
fn og_card_renders_svg() {
    let cfg = load::load_file(std::path::Path::new(&example("product"))).unwrap();
    if cfg.site.open_graph.configured() {
        let svg = og::render_og(&cfg).unwrap();
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("1200"));
    }
}

#[test]
fn social_resolution_falls_back_to_site() {
    let cfg = load::load_file(std::path::Path::new(&example("product"))).unwrap();
    let og_meta = cfg.social();
    // The example sets og.title explicitly; the resolved defaults still apply.
    assert_eq!(og_meta.og_type, "website");
    assert_eq!(og_meta.twitter_card, "summary_large_image");
    assert_eq!(og_meta.site_name, cfg.site.name);
}

#[test]
fn empty_config_produces_validation_errors() {
    let cfg = Config::default();
    let errs = validate::errors(&cfg);
    assert!(errs.iter().any(|e| e.contains("site.name")));
    assert!(errs.iter().any(|e| e.contains("site.nav")));
}

#[test]
fn build_emits_metadata_files() {
    let dir = std::env::temp_dir().join(format!("pagers-landing-meta-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let yaml = dir.join("landify.yaml");
    let out = dir.join("index.html");
    std::fs::write(&yaml, std::fs::read(example("product")).unwrap()).unwrap();
    let result = build::build_file(&yaml, &out, "").unwrap();
    assert_eq!(result.meta.len(), 7);
    for name in [
        "favicon.svg",
        "robots.txt",
        "sitemap.xml",
        "humans.txt",
        "site.webmanifest",
        "llms.txt",
        "styles.css",
    ] {
        assert!(dir.join(name).exists(), "missing {name}");
    }
    let robots = std::fs::read_to_string(dir.join("robots.txt")).unwrap();
    assert!(robots.contains("User-agent: *"));
    let manifest = std::fs::read_to_string(dir.join("site.webmanifest")).unwrap();
    assert!(manifest.contains("\"name\""));
    // Second build must not clobber a customized file.
    std::fs::write(dir.join("robots.txt"), "custom\n").unwrap();
    let result2 = build::build_file(&yaml, &out, "").unwrap();
    assert!(result2.meta.is_empty());
    assert_eq!(
        std::fs::read_to_string(dir.join("robots.txt")).unwrap(),
        "custom\n"
    );
    std::fs::remove_dir_all(&dir).ok();
}
