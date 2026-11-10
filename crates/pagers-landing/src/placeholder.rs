//! Annotated `landify.yaml` scaffolds per page type — port of
//! `internal/landify/placeholder.go`. Examples are embedded at compile time.

use crate::validate::{is_known_type, normalize_type, KNOWN_TYPES};
use std::path::Path;

fn example(typ: &str) -> Option<&'static str> {
    Some(match typ {
        "app" => include_str!("../examples/example-app.yaml"),
        "docs" => include_str!("../examples/example-docs.yaml"),
        "download" => include_str!("../examples/example-download.yaml"),
        "event" => include_str!("../examples/example-event.yaml"),
        "faq" => include_str!("../examples/example-faq.yaml"),
        "linktree" => include_str!("../examples/example-linktree.yaml"),
        "portfolio" => include_str!("../examples/example-portfolio.yaml"),
        "pricing" => include_str!("../examples/example-pricing.yaml"),
        "product" => include_str!("../examples/example-product.yaml"),
        "status" => include_str!("../examples/example-status.yaml"),
        "team" => include_str!("../examples/example-team.yaml"),
        "waitlist" => include_str!("../examples/example-waitlist.yaml"),
        _ => return None,
    })
}

/// The annotated landify.yaml template for `typ`. Unknown types are rejected;
/// empty maps to the default "product" example.
pub fn placeholder(typ: &str) -> Result<&'static str, String> {
    let typ = normalize_type(typ);
    if !is_known_type(typ) {
        return Err(format!(
            "type {typ:?} is not supported (available: {})",
            KNOWN_TYPES.join(", ")
        ));
    }
    example(typ).ok_or_else(|| format!("no example for type {typ:?}"))
}

/// Write the scaffold for `typ` to `path`.
pub fn write_placeholder(path: &Path, typ: &str) -> Result<(), String> {
    let data = placeholder(typ)?;
    std::fs::write(path, data).map_err(|e| format!("write {}: {e}", path.display()))
}
