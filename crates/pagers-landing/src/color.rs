//! Color math: `Tokens` expands a Theme into every `:root` custom property,
//! plus the contrast/readability helpers the OG card shares. Port of
//! `internal/landify/color.go`.

use crate::config::Theme;
use std::collections::BTreeMap;

/// Expanded `:root` custom properties, sorted by name.
pub type Tokens = BTreeMap<String, String>;

/// `Tokens` expands a Theme into every `:root` custom property. Only the
/// authored colors appear as inputs; the rest are derived.
pub fn tokens(t: &Theme) -> Result<Tokens, ColorError> {
    let (base_r, base_g, base_b) = parse_hex(&t.base)?;
    let (primary_r, primary_g, primary_b) = parse_hex(&t.primary)?;
    let (secondary_r, secondary_g, secondary_b) = parse_hex(&t.secondary)?;
    let (neutral_r, neutral_g, neutral_b) = parse_hex(&t.neutral)?;
    for color in [&t.info, &t.warning, &t.success, &t.error] {
        parse_hex(color)?;
    }

    let base_lum = luminance(base_r, base_g, base_b);
    let mut base200 = hex(
        mix(base_r, 255, 0.85),
        mix(base_g, 255, 0.85),
        mix(base_b, 255, 0.85),
    );
    let mut base300 = hex(
        mix(base_r, 0, 0.03),
        mix(base_g, 0, 0.03),
        mix(base_b, 0, 0.03),
    );
    if base_lum < 0.35 {
        base200 = hex(
            mix(base_r, 255, 0.10),
            mix(base_g, 255, 0.10),
            mix(base_b, 255, 0.10),
        );
        base300 = hex(
            mix(base_r, 0, 0.08),
            mix(base_g, 0, 0.08),
            mix(base_b, 0, 0.08),
        );
    }

    let mut out = Tokens::new();
    out.insert("base-100".into(), t.base.clone());
    out.insert("base-200".into(), base200);
    out.insert("base-300".into(), base300);
    out.insert(
        "base-content".into(),
        contrasting_text(base_r, base_g, base_b),
    );
    out.insert("primary".into(), t.primary.clone());
    out.insert(
        "primary-dark".into(),
        hex(
            mix(primary_r, 0, 0.15),
            mix(primary_g, 0, 0.15),
            mix(primary_b, 0, 0.15),
        ),
    );
    out.insert(
        "primary-soft".into(),
        hex(
            mix(primary_r, base_r, 0.88),
            mix(primary_g, base_g, 0.88),
            mix(primary_b, base_b, 0.88),
        ),
    );
    out.insert(
        "primary-content".into(),
        contrasting_text(primary_r, primary_g, primary_b),
    );
    out.insert("secondary".into(), t.secondary.clone());
    out.insert(
        "secondary-content".into(),
        contrasting_text(secondary_r, secondary_g, secondary_b),
    );
    out.insert("neutral".into(), t.neutral.clone());
    out.insert(
        "neutral-faint".into(),
        hex(
            mix(neutral_r, base_r, 0.45),
            mix(neutral_g, base_g, 0.45),
            mix(neutral_b, base_b, 0.45),
        ),
    );
    out.insert("border".into(), t.neutral.clone());
    out.insert(
        "border-soft".into(),
        hex(
            mix(neutral_r, base_r, 0.60),
            mix(neutral_g, base_g, 0.60),
            mix(neutral_b, base_b, 0.60),
        ),
    );
    out.insert("info".into(), t.info.clone());
    out.insert("warning".into(), t.warning.clone());
    out.insert("success".into(), t.success.clone());
    out.insert("error".into(), t.error.clone());
    out.insert("radius".into(), t.radius.clone());
    Ok(out)
}

/// Steps `readable_on` walks toward black/white.
const READABLE_STEP: f64 = 0.04;
const READABLE_STEPS: usize = 25;

/// `blend` mixes two "#RRGGBB" colors; strength 0 → a, 1 → b.
pub fn blend(a: &str, b: &str, strength: f64) -> Result<String, ColorError> {
    let (ar, ag, ab) = parse_hex(a)?;
    let (br, bg, bb) = parse_hex(b)?;
    Ok(hex(
        mix(ar, br, strength),
        mix(ag, bg, strength),
        mix(ab, bb, strength),
    ))
}

/// `mix` blends one channel toward target by strength.
fn mix(c: i32, target: i32, strength: f64) -> i32 {
    (f64::from(c) + (f64::from(target) - f64::from(c)) * strength).round() as i32
}

/// WCAG contrast ratio between two colors: 1 same, 21 black-on-white.
pub fn contrast_ratio(a: &str, b: &str) -> Result<f64, ColorError> {
    let (ar, ag, ab) = parse_hex(a)?;
    let (br, bg, bb) = parse_hex(b)?;
    let (mut lighter, mut darker) = (luminance(ar, ag, ab), luminance(br, bg, bb));
    if darker > lighter {
        std::mem::swap(&mut lighter, &mut darker);
    }
    Ok((lighter + 0.05) / (darker + 0.05))
}

/// `readable_on` shifts fg toward black (light bg) or white (dark bg) until
/// it reaches `min` contrast against bg; unchanged when it already does.
pub fn readable_on(fg: &str, bg: &str, min: f64) -> Result<String, ColorError> {
    let (r, g, b) = parse_hex(fg)?;
    let (br, bg_r, bb) = parse_hex(bg)?;
    if let Ok(ratio) = contrast_ratio(fg, bg) {
        if ratio >= min {
            return Ok(fg.to_string());
        }
    }
    let target = if luminance(br, bg_r, bb) >= 0.5 {
        0
    } else {
        255
    };
    for step in 1..=READABLE_STEPS {
        let strength = step as f64 * READABLE_STEP;
        let candidate = hex(
            mix(r, target, strength),
            mix(g, target, strength),
            mix(b, target, strength),
        );
        if let Ok(ratio) = contrast_ratio(&candidate, bg) {
            if ratio >= min {
                return Ok(candidate);
            }
        }
    }
    Ok(hex(
        mix(r, target, 1.0),
        mix(g, target, 1.0),
        mix(b, target, 1.0),
    ))
}

/// Parses "#RRGGBB" (the "#" optional) into RGB channels.
pub fn parse_hex(s: &str) -> Result<(i32, i32, i32), ColorError> {
    let s = s.trim().trim_start_matches('#');
    if s.len() != 6 {
        return Err(ColorError::Parse(format!("parse {s:?}: want #RRGGBB")));
    }
    let n =
        u32::from_str_radix(s, 16).map_err(|e| ColorError::Parse(format!("parse {s:?}: {e}")))?;
    Ok((
        (n >> 16) as i32,
        ((n >> 8) & 0xff) as i32,
        (n & 0xff) as i32,
    ))
}

/// Formats RGB channels as "#rrggbb".
pub fn hex(r: i32, g: i32, b: i32) -> String {
    format!("#{r:02x}{g:02x}{b:02x}")
}

/// `contrasting_text` picks near-black on light backgrounds, white on dark.
pub fn contrasting_text(r: i32, g: i32, b: i32) -> String {
    if luminance(r, g, b) >= 0.5 {
        "#181d25".into()
    } else {
        "#ffffff".into()
    }
}

/// WCAG relative luminance of an sRGB color.
pub fn luminance(r: i32, g: i32, b: i32) -> f64 {
    fn linear(c: i32) -> f64 {
        let v = f64::from(c) / 255.0;
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * linear(r) + 0.7152 * linear(g) + 0.0722 * linear(b)
}

/// Error type for color parsing/derivation.
#[derive(Debug, thiserror::Error)]
pub enum ColorError {
    #[error("{0}")]
    Parse(String),
}
