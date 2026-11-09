//! Emits `PAGERS_BUILD_TIME` (`YYYY.MM.DD.hh.mm.ss`, UTC) so `--version`
//! can report when the binary was compiled.

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    println!("cargo:rustc-env=PAGERS_BUILD_TIME={}", stamp(secs));
    println!("cargo:rerun-if-changed=build.rs");
}

/// Unix seconds → `YYYY.MM.DD.hh.mm.ss` (UTC, civil-from-days algorithm).
fn stamp(secs: u64) -> String {
    let days = secs / 86_400;
    let sod = secs % 86_400;
    let (y, m, d) = civil(days);
    format!(
        "{y:04}.{m:02}.{d:02}.{:02}.{:02}.{:02}",
        sod / 3600,
        sod % 3600 / 60,
        sod % 60
    )
}

fn civil(z: u64) -> (u64, u64, u64) {
    let z = z + 719_468;
    let era = z / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_timestamp() {
        // 2024-01-02 03:04:05 UTC
        assert_eq!(stamp(1_704_164_645), "2024.01.02.03.04.05");
        assert_eq!(stamp(0), "1970.01.01.00.00.00");
    }
}
