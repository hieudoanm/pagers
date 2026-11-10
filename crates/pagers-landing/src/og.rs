//! Open Graph card: 1200×630 SVG generated from the page theme.
//! Port of `og.go`, `ogpalette.go`, `ogtext.go`, `oglayout.go`.

use crate::color;
use crate::color::Tokens;
use crate::config::Config;
use std::path::{Path, PathBuf};

// ---- geometry constants (ogtext.go / oglayout.go) ----
const OG_WIDTH: i32 = 1200;
const OG_HEIGHT: i32 = 630;
const OG_RAIL: i32 = 12;
const OG_PAD: i32 = 96;
const OG_EDGE: i32 = OG_WIDTH - OG_PAD;
const OG_HEAD_Y: i32 = 84;
const OG_HEAD_TILE: i32 = 56;
const OG_HEAD_RADIUS: i32 = 16;
const OG_NAME_SIZE: i32 = 26;
const OG_HOST_SIZE: i32 = 22;
const OG_NAME_STEP: i32 = 20;
const OG_RULE_Y: i32 = 176;
const OG_GUTTER: i32 = 48;
const OG_HEAD_LIFT: i32 = 37;
const OG_HEAD_NUDGE: i32 = 35;
const OG_MONO_SIZE: i32 = 26;
const OG_MONO_LIFT: i32 = 15;
const OG_COLUMN: i32 = 748 - OG_PAD - OG_GUTTER;
const OG_TEXT_TOP: i32 = 202;
const OG_TEXT_BOTTOM: i32 = 584;
const OG_KICKER_SIZE: i32 = 17;
const OG_KICKER_STEP: i32 = 30;
const OG_KICKER_TRACK: f64 = 0.16;
const OG_TITLE_SIZE: i32 = 58;
const OG_TITLE_STEP: i32 = 62;
const OG_TITLE_LINES: usize = 3;
const OG_DESC_SIZE: i32 = 25;
const OG_DESC_STEP: i32 = 35;
const OG_DESC_LINES: usize = 3;
const OG_GAP: i32 = 52;
const OG_KICKER_CAP: i32 = 12;
const OG_ASCENDER: i32 = 45;
const OG_TITLE_DESC: i32 = 14;
const OG_DESC_DEPTH: i32 = 7;
const OG_PANEL_X: i32 = 748;
const OG_PANEL_Y: i32 = OG_TEXT_TOP;
const OG_PANEL_W: i32 = OG_EDGE - OG_PANEL_X;
const OG_PANEL_H: i32 = OG_TEXT_BOTTOM - OG_TEXT_TOP;
const OG_PANEL_RADIUS: i32 = 24;
const OG_PANEL_PAD: i32 = 30;
const OG_PANEL_TILE: i32 = 84;
const OG_PANEL_TILE_R: i32 = 24;
const OG_PANEL_MONOGRAM: i32 = 42;
const OG_PANEL_GAP: i32 = 26;
const OG_PILL_H: i32 = 44;
const OG_PILL_GAP: i32 = 10;
const OG_PILL_SIZE: i32 = 19;
const OG_PILL_PAD_X: i32 = 20;
const OG_PILL_DOT: i32 = 8;
const OG_PILL_TEXT_GAP: i32 = 12;
const OG_PILL_LIFT: i32 = 7;
const OG_PILL_MAX: usize = 4;
const OG_PILL_FRAME: i32 = 2 * OG_PILL_PAD_X + OG_PILL_DOT + OG_PILL_TEXT_GAP;
const OG_MIN_CONTRAST: f64 = 4.5;
const OG_INK: &str = "#181d25";

// ---- view structs consumed by templates/og_card.html ----

#[derive(Debug, Clone)]
pub struct OgFrame {
    pub width: i32,
    pub height: i32,
    pub pad: i32,
    pub edge: i32,
    pub rail: i32,
    pub head_y: i32,
    pub tile: i32,
    pub tile_radius: i32,
    pub name_x: i32,
    pub monogram_x: i32,
    pub monogram_size: i32,
    pub baseline: i32,
    pub host_baseline: i32,
    pub rule_y: i32,
}

#[derive(Debug, Clone)]
pub struct OgType {
    pub name: i32,
    pub host: i32,
    pub kicker: i32,
    pub kicker_step: i32,
    pub kicker_y: i32,
    pub title: i32,
    pub title_step: i32,
    pub title_y: i32,
    pub description: i32,
    pub desc_step: i32,
    pub description_y: i32,
    pub column: i32,
}

#[derive(Debug, Clone)]
pub struct OgPanel {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub radius: i32,
    pub tile: i32,
    pub tile_x: i32,
    pub tile_y: i32,
    pub tile_radius: i32,
    pub monogram: i32,
    pub monogram_x: i32,
    pub monogram_y: i32,
    pub pill_size: i32,
}

#[derive(Debug, Clone)]
pub struct OgPill {
    pub label: String,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub dot_x: i32,
    pub dot_y: i32,
    pub dot_r: i32,
    pub text_x: i32,
    pub text_y: i32,
    pub radius: i32,
}

#[derive(Debug, Clone)]
pub struct OgColor {
    pub base: String,
    pub base_alt: String,
    pub glow: String,
    pub accent: String,
    pub accent_deep: String,
    pub accent_ink: String,
    pub accent_on_dark: String,
    pub ink: String,
    pub muted: String,
    pub hairline: String,
    pub panel: String,
    pub panel_line: String,
    pub pill: String,
}

/// One wrapped line of card copy with its baseline.
#[derive(Debug, Clone)]
pub struct OgRun {
    pub text: String,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct OgCard {
    pub frame: OgFrame,
    pub og_type: OgType,
    pub panel: OgPanel,
    pub site: String,
    pub monogram: String,
    pub kicker: String,
    pub host: String,
    pub title: String,
    pub lines: Vec<OgRun>,
    pub desc: String,
    pub desc_runs: Vec<OgRun>,
    pub tags: Vec<String>,
    pub pills: Vec<OgPill>,
    pub colors: OgColor,
}

/// `og/og.svg` beside the built page.
pub fn og_card_path(output: &Path) -> PathBuf {
    output
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("og")
        .join("og.svg")
}

/// Render the card SVG for `cfg`.
pub fn render_og(cfg: &Config) -> Result<String, crate::build::BuildError> {
    let card = og_card(cfg)?;
    use askama::Template;
    Ok(crate::build::OgCardTemplate { card: &card }.render()?)
}

fn og_card(cfg: &Config) -> Result<OgCard, crate::build::BuildError> {
    let tokens = color::tokens(&cfg.theme).map_err(crate::build::BuildError::Color)?;
    let colors = og_colors(&tokens)?;
    let og = cfg.social();
    let title = og.title.trim().to_string();
    let description = og_trim_description(&og.description);
    let mut card = OgCard {
        frame: og_frame(),
        og_type: og_scale(),
        panel: og_panel_box(),
        site: cfg.site.name.clone(),
        monogram: og_monogram(&cfg.site.name),
        kicker: og_kicker(&og.kicker),
        host: og_host(&og.url),
        title: title.clone(),
        lines: og_wrap(
            &title,
            OG_TITLE_SIZE as f64,
            OG_COLUMN as f64,
            OG_TITLE_LINES,
        )
        .into_iter()
        .map(|text| OgRun { text, y: 0 })
        .collect(),
        desc: description.clone(),
        desc_runs: og_wrap(
            &description,
            OG_DESC_SIZE as f64,
            OG_COLUMN as f64,
            OG_DESC_LINES,
        )
        .into_iter()
        .map(|text| OgRun { text, y: 0 })
        .collect(),
        tags: og.tags.clone(),
        pills: Vec::new(),
        colors,
    };
    og_center_text(&mut card);
    og_pills(&mut card);
    Ok(card)
}

fn og_colors(tokens: &Tokens) -> Result<OgColor, crate::build::BuildError> {
    let base = tokens["base-100"].clone();
    let accent = tokens["primary"].clone();
    let glow = color::blend(&accent, &tokens["secondary"], 0.5)
        .map_err(crate::build::BuildError::Color)?;
    let readable = color::readable_on(&accent, &base, OG_MIN_CONTRAST)
        .map_err(crate::build::BuildError::Color)?;
    let muted = color::readable_on(&tokens["neutral"], &base, OG_MIN_CONTRAST)
        .map_err(crate::build::BuildError::Color)?;
    let on_accent = og_ink_on(&[&accent, &tokens["primary-dark"]])?;
    Ok(OgColor {
        base,
        base_alt: tokens["base-200"].clone(),
        glow,
        accent: accent.clone(),
        accent_deep: tokens["primary-dark"].clone(),
        accent_ink: readable,
        accent_on_dark: on_accent,
        ink: tokens["base-content"].clone(),
        muted,
        hairline: tokens["neutral-faint"].clone(),
        panel: tokens["primary-soft"].clone(),
        panel_line: tokens["border-soft"].clone(),
        pill: tokens["base-100"].clone(),
    })
}

/// `og_ink_on` — whichever ink reads better on the accent gradient ends.
fn og_ink_on(gradient: &[&str]) -> Result<String, crate::build::BuildError> {
    let worst = |ink: &str| -> Result<f64, crate::build::BuildError> {
        let mut lowest = f64::MAX;
        for bg in gradient {
            let ratio = color::contrast_ratio(ink, bg).map_err(crate::build::BuildError::Color)?;
            lowest = lowest.min(ratio);
        }
        Ok(lowest)
    };
    let on_white = worst("#ffffff")?;
    let on_ink = worst(OG_INK)?;
    Ok(if on_ink > on_white {
        OG_INK.into()
    } else {
        "#ffffff".into()
    })
}

fn og_kicker(kicker: &str) -> String {
    og_clamp_track(
        &kicker.trim().to_uppercase(),
        OG_KICKER_SIZE as f64,
        OG_KICKER_TRACK,
        OG_COLUMN as f64,
    )
}

fn og_trim_description(description: &str) -> String {
    description.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn og_host(raw_url: &str) -> String {
    let raw = raw_url.trim();
    if raw.is_empty() {
        return String::new();
    }
    // url::Url needs absolute; fall back to the raw value like Go's url.Parse
    // with empty Host.
    if let Ok(u) = url_parse_host(raw) {
        if !u.is_empty() {
            return u.trim_start_matches("www.").to_string();
        }
    }
    raw.to_string()
}

fn url_parse_host(raw: &str) -> Result<String, ()> {
    // Minimal host extraction: scheme://host — avoids a url crate dep.
    let rest = raw.split("://").nth(1).ok_or(())?;
    let host = rest.split('/').next().ok_or(())?;
    if host.is_empty() {
        return Err(());
    }
    Ok(host.to_string())
}

// ---- layout ----

fn og_frame() -> OgFrame {
    OgFrame {
        width: OG_WIDTH,
        height: OG_HEIGHT,
        pad: OG_PAD,
        edge: OG_EDGE,
        rail: OG_RAIL,
        head_y: OG_HEAD_Y,
        tile: OG_HEAD_TILE,
        tile_radius: OG_HEAD_RADIUS,
        name_x: OG_PAD + OG_HEAD_TILE + OG_NAME_STEP,
        monogram_x: OG_PAD + OG_HEAD_TILE / 2,
        monogram_size: OG_MONO_SIZE,
        baseline: OG_HEAD_Y + OG_HEAD_LIFT,
        host_baseline: OG_HEAD_Y + OG_HEAD_NUDGE,
        rule_y: OG_RULE_Y,
    }
}

fn og_scale() -> OgType {
    OgType {
        name: OG_NAME_SIZE,
        host: OG_HOST_SIZE,
        kicker: OG_KICKER_SIZE,
        kicker_step: OG_KICKER_STEP,
        kicker_y: 0,
        title: OG_TITLE_SIZE,
        title_step: OG_TITLE_STEP,
        title_y: 0,
        description: OG_DESC_SIZE,
        desc_step: OG_DESC_STEP,
        description_y: 0,
        column: OG_COLUMN,
    }
}

fn og_panel_box() -> OgPanel {
    let tile_x = OG_PANEL_X + OG_PANEL_PAD;
    OgPanel {
        x: OG_PANEL_X,
        y: OG_PANEL_Y,
        w: OG_PANEL_W,
        h: OG_PANEL_H,
        radius: OG_PANEL_RADIUS,
        tile: OG_PANEL_TILE,
        tile_x,
        tile_y: 0,
        tile_radius: OG_PANEL_TILE_R,
        monogram: OG_PANEL_MONOGRAM,
        monogram_x: tile_x + OG_PANEL_TILE / 2,
        monogram_y: 0,
        pill_size: OG_PILL_SIZE,
    }
}

fn og_center_text(card: &mut OgCard) {
    let mut height = 0;
    if !card.kicker.is_empty() {
        height += OG_KICKER_CAP + OG_KICKER_STEP;
    }
    height += (card.lines.len() as i32 - 1) * OG_TITLE_STEP + OG_ASCENDER;
    if !card.desc_runs.is_empty() {
        height += OG_GAP + (card.desc_runs.len() as i32 - 1) * OG_DESC_STEP + OG_DESC_DEPTH;
    } else {
        height += OG_TITLE_DESC;
    }
    let mut top = OG_TEXT_TOP + (OG_TEXT_BOTTOM - OG_TEXT_TOP - height) / 2;
    if !card.kicker.is_empty() {
        card.og_type.kicker_y = top + OG_KICKER_CAP;
        top += OG_KICKER_CAP + OG_KICKER_STEP;
    }
    card.og_type.title_y = top + OG_ASCENDER;
    if !card.desc_runs.is_empty() {
        card.og_type.description_y =
            card.og_type.title_y + (card.lines.len() as i32 - 1) * OG_TITLE_STEP + OG_GAP;
    }
    let title_y = card.og_type.title_y;
    let desc_y = card.og_type.description_y;
    for (i, run) in card.lines.iter_mut().enumerate() {
        run.y = title_y + i as i32 * OG_TITLE_STEP;
    }
    for (i, run) in card.desc_runs.iter_mut().enumerate() {
        run.y = desc_y + i as i32 * OG_DESC_STEP;
    }
}

fn og_pills(card: &mut OgCard) {
    let inner = (OG_PANEL_W - 2 * OG_PANEL_PAD) as f64;
    let tags: Vec<String> = card.tags.iter().take(OG_PILL_MAX).cloned().collect();
    let mut stack = OG_PANEL_TILE + OG_PANEL_GAP;
    if !tags.is_empty() {
        stack += tags.len() as i32 * OG_PILL_H + (tags.len() as i32 - 1) * OG_PILL_GAP;
    }
    let top = OG_PANEL_Y + (OG_PANEL_H - stack) / 2;
    card.panel.tile_y = top;
    card.panel.monogram_y = top + OG_PANEL_TILE / 2 + OG_MONO_LIFT;
    for (i, tag) in tags.iter().enumerate() {
        let y = top + OG_PANEL_TILE + OG_PANEL_GAP + i as i32 * (OG_PILL_H + OG_PILL_GAP);
        og_pill(card, tag, y, inner);
    }
}

fn og_pill(card: &mut OgCard, tag: &str, y: i32, inner: f64) {
    let label = og_clamp(
        tag.trim(),
        OG_PILL_SIZE as f64,
        inner - OG_PILL_FRAME as f64,
    );
    card.pills.push(OgPill {
        label: label.clone(),
        x: OG_PANEL_X + OG_PANEL_PAD,
        y,
        w: og_text_width(&label, OG_PILL_SIZE as f64) as i32 + OG_PILL_FRAME,
        h: OG_PILL_H,
        dot_x: OG_PANEL_X + OG_PANEL_PAD + OG_PILL_PAD_X,
        dot_y: y + OG_PILL_H / 2,
        dot_r: OG_PILL_DOT / 2,
        text_x: OG_PANEL_X + OG_PANEL_PAD + OG_PILL_PAD_X + OG_PILL_DOT + OG_PILL_TEXT_GAP,
        text_y: y + OG_PILL_H / 2 + OG_PILL_LIFT,
        radius: OG_PILL_H / 2,
    });
}

// ---- text measurement (ogtext.go) ----

fn og_rune_width(r: char, size: f64) -> f64 {
    match r {
        ' ' => 0.28 * size,
        'i' | 'l' | 'j' | 't' | 'I' | 'f' | '.' | ',' | ':' | ';' | '\'' | '!' | '|' | '('
        | ')' | '[' | ']' | '{' | '}' | '-' | '`' => 0.32 * size,
        'm' | 'w' | 'M' | 'W' | '@' | '%' => 0.92 * size,
        c if c.is_uppercase() => 0.70 * size,
        _ => 0.55 * size,
    }
}

fn og_text_width(s: &str, size: f64) -> f64 {
    s.chars().map(|r| og_rune_width(r, size)).sum()
}

fn og_track_width(s: &str, size: f64, tracking: f64) -> f64 {
    og_text_width(s, size) + s.chars().count() as f64 * tracking * size
}

fn og_clamp_track(s: &str, size: f64, tracking: f64, max_width: f64) -> String {
    if og_track_width(s, size, tracking) <= max_width {
        return s.to_string();
    }
    let mut s = s.to_string();
    while !s.is_empty() && og_track_width(&(s.clone() + "…"), size, tracking) > max_width {
        s = s
            .trim_end()
            .chars()
            .take(s.trim_end().chars().count() - 1)
            .collect();
    }
    s + "…"
}

fn og_wrap(s: &str, size: f64, max_width: f64, max_lines: usize) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut used = String::new();
    for word in s.split_whitespace() {
        let candidate = if used.is_empty() {
            word.to_string()
        } else {
            format!("{used} {word}")
        };
        if !used.is_empty() && og_text_width(&candidate, size) > max_width {
            lines.push(std::mem::take(&mut used));
            used = word.to_string();
            continue;
        }
        used = candidate;
    }
    if !used.is_empty() {
        lines.push(used);
    }
    if lines.len() <= max_lines {
        return lines;
    }
    lines.truncate(max_lines);
    let last = lines.pop().unwrap_or_default();
    lines.push(og_ellipsis(&last, size, max_width));
    lines
}

fn og_clamp(s: &str, size: f64, max_width: f64) -> String {
    if og_text_width(s, size) <= max_width {
        s.to_string()
    } else {
        og_ellipsis(s, size, max_width)
    }
}

fn og_ellipsis(line: &str, size: f64, max_width: f64) -> String {
    let mut line = line.to_string();
    while og_text_width(&(line.clone() + "…"), size) > max_width && !line.is_empty() {
        line = line
            .trim_end()
            .chars()
            .take(line.trim_end().chars().count() - 1)
            .collect();
    }
    line + "…"
}

/// `og_monogram` — one/two-letter mark from the site name.
fn og_monogram(name: &str) -> String {
    let words: Vec<&str> = name
        .split(|c: char| !(c.is_alphanumeric()))
        .filter(|w| !w.is_empty())
        .collect();
    if words.is_empty() {
        return String::new();
    }
    let letter = |word: &str| -> String {
        word.chars()
            .find(|c| c.is_alphanumeric())
            .map(|c| c.to_uppercase().next().unwrap_or(c))
            .map(|c| c.to_string())
            .unwrap_or_default()
    };
    if words.len() > 1 {
        return letter(words[0]) + &letter(words[1]);
    }
    let first = letter(words[0]);
    if !first.is_empty() && words[0].chars().count() > 1 {
        // first letter of the word + first letter of the rest
        let rest: String = words[0].chars().skip(1).collect();
        return first + &letter(&rest);
    }
    first
}
