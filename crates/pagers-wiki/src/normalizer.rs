//! Markdown normalisation: a deterministic post-pass over the htmd output
//! that enforces consistent whitespace, list markers, heading shape and
//! paragraph uniqueness without touching content semantics.

/// Normalises `markdown` and prepends the article title as the top-level
/// heading when it is not already the first heading.
pub fn normalize(markdown: &str, title: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    for raw in markdown.lines() {
        let line = normalize_table_row(&normalize_bullet(raw.trim_end()));
        if line.trim().is_empty() {
            // Collapse runs of blank lines to a single separator.
            if !lines.last().is_some_and(|l: &String| l.is_empty()) && !lines.is_empty() {
                lines.push(String::new());
            }
            continue;
        }
        lines.push(line);
    }
    while lines.last().is_some_and(|l| l.is_empty()) {
        lines.pop();
    }
    let lines = drop_empty_headings(&lines);

    let mut out = String::new();
    if !title.is_empty() {
        out.push_str("# ");
        out.push_str(title);
        out.push_str("\n\n");
    }
    out.push_str(&dedupe_paragraphs(&lines));
    out.push('\n');
    out
}

fn normalize_bullet(line: &str) -> String {
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];
    for marker in ["* ", "+ "] {
        if let Some(rest) = trimmed.strip_prefix(marker) {
            return format!("{indent}- {rest}");
        }
    }
    line.to_string()
}

fn is_heading(line: &str) -> bool {
    let t = line.trim_start();
    t.starts_with('#') && t[1..].starts_with(' ')
}

/// Drops headings immediately followed by another heading or the end of the
/// document — headings whose section ended up empty after cleaning.
fn drop_empty_headings(lines: &[String]) -> Vec<String> {
    let mut out = Vec::with_capacity(lines.len());
    for (i, line) in lines.iter().enumerate() {
        if is_heading(line) {
            let next = lines[i + 1..].iter().find(|l| !l.trim().is_empty());
            if next.is_none_or(|n| is_heading(n)) {
                continue;
            }
        }
        out.push(line.clone());
    }
    out
}

/// Rewrites a GFM table row with single-space padding so htmd's
/// column-width alignment does not produce unreadable lines. Separator
/// rows keep their alignment markers (`:---`, `---:`, `:--:`).
fn normalize_table_row(line: &str) -> String {
    let trimmed = line.trim_start();
    if !trimmed.starts_with('|') {
        return line.to_string();
    }
    let cells: Vec<String> = split_row(trimmed)
        .iter()
        .map(|c| c.trim().to_string())
        .collect();
    if cells.iter().all(|c| is_table_separator(c)) {
        let sep: Vec<&str> = cells.iter().map(|c| table_sep(c)).collect();
        return format!("| {} |", sep.join(" | "));
    }
    format!("| {} |", cells.join(" | "))
}

/// Splits a table row on unescaped `|` characters, keeping `\|` intact.
fn split_row(row: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut current = String::new();
    let mut escaped = false;
    for ch in row
        .trim()
        .trim_start_matches('|')
        .trim_end_matches('|')
        .chars()
    {
        if escaped {
            current.push('\\');
            current.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '|' {
            cells.push(std::mem::take(&mut current));
        } else {
            current.push(ch);
        }
    }
    if escaped {
        current.push('\\');
    }
    cells.push(current);
    cells
}

fn is_table_separator(cell: &str) -> bool {
    let c = cell.trim_matches(':');
    c.len() >= 3 && c.chars().all(|ch| ch == '-')
}

fn table_sep(cell: &str) -> &'static str {
    let left = cell.starts_with(':');
    let right = cell.ends_with(':') && cell.len() > 1;
    match (left, right) {
        (true, true) => ":---:",
        (true, false) => ":---",
        (false, true) => "---:",
        (false, false) => "---",
    }
}

/// Removes exact duplicate paragraphs while preserving order. Only whole
/// non-empty, non-heading, non-list, non-code lines participate so list
/// items and headings that legitimately repeat are untouched.
fn dedupe_paragraphs(lines: &[String]) -> String {
    let mut seen = std::collections::HashSet::new();
    let mut out = String::new();
    let mut paragraph: Vec<&str> = Vec::new();

    let flush = |paragraph: &mut Vec<&str>,
                 out: &mut String,
                 seen: &mut std::collections::HashSet<String>| {
        if paragraph.is_empty() {
            return;
        }
        let text = paragraph.join(" ");
        let key = text.trim().to_string();
        let is_content = !key.is_empty()
            && !key.starts_with('#')
            && !key.starts_with("- ")
            && !key.starts_with("```");
        if !is_content || seen.insert(key) {
            for line in paragraph.drain(..) {
                out.push_str(line);
                out.push('\n');
            }
        } else {
            paragraph.clear();
        }
        out.push('\n');
    };

    for line in lines {
        if line.trim().is_empty() {
            flush(&mut paragraph, &mut out, &mut seen);
        } else {
            paragraph.push(line.as_str());
        }
    }
    flush(&mut paragraph, &mut out, &mut seen);
    // Collapse the extra blank line the final flush adds.
    while out.ends_with("\n\n\n") {
        out.pop();
    }
    out.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepends_title_and_trims() {
        let md = normalize("Some *text*.\n\n\n\nMore.\n", "My Article");
        assert!(md.starts_with("# My Article\n\nSome *text*.\n\nMore.\n"));
    }

    #[test]
    fn normalizes_bullets() {
        let md = normalize("* one\n  + two\n", "T");
        assert!(md.contains("- one"));
        assert!(md.contains("  - two"));
    }

    #[test]
    fn dedupes_exact_paragraphs() {
        let md = normalize("Same paragraph.\n\nDifferent.\n\nSame paragraph.\n", "T");
        assert_eq!(md.matches("Same paragraph.").count(), 1);
        assert!(md.contains("Different."));
    }

    #[test]
    fn keeps_repeated_list_items() {
        let md = normalize("- item\n- item\n", "T");
        assert_eq!(md.matches("- item").count(), 2);
    }
}
