//! YAML loading: strict parse + theme merge. Port of `Load`/`LoadFile`.

use crate::config::{Config, Theme};
use crate::validate;
use std::path::Path;

/// Error loading or validating a landify.yaml.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("read {0}: {1}")]
    Read(String, std::io::Error),
    #[error("parse yaml: {0}")]
    Parse(#[from] serde_yaml::Error),
    #[error("{path} is invalid:\n  - {}", .problems.join("\n  - "))]
    Invalid { path: String, problems: Vec<String> },
}

/// Parse YAML strictly (unknown fields rejected) and merge theme defaults.
pub fn load(data: &[u8]) -> Result<Config, LoadError> {
    let mut cfg: Config = serde_yaml::from_slice(data)?;
    cfg.theme = cfg.theme.merged(&Theme::default());
    Ok(cfg)
}

/// Read and parse the YAML file at `path`.
pub fn load_file(path: &Path) -> Result<Config, LoadError> {
    let data = std::fs::read(path).map_err(|e| LoadError::Read(path.display().to_string(), e))?;
    load(&data)
}

/// Load, apply an optional built-in theme override, and reject invalid configs.
pub fn render_config(path: &Path, theme_name: &str) -> Result<Config, crate::build::BuildError> {
    let mut cfg = load_file(path).map_err(crate::build::BuildError::Load)?;
    if !theme_name.is_empty() {
        let theme = crate::themes::theme_by_name(theme_name).ok_or_else(|| {
            crate::build::BuildError::UnknownTheme(
                theme_name.to_string(),
                crate::themes::theme_names().join(", "),
            )
        })?;
        cfg.theme = theme;
    }
    let errs = validate::errors(&cfg);
    if !errs.is_empty() {
        return Err(crate::build::BuildError::Invalid {
            path: path.display().to_string(),
            problems: errs,
        });
    }
    Ok(cfg)
}

/// Validate a file, returning a combined error listing every problem.
pub fn validate_file(path: &Path) -> Result<(), LoadError> {
    let cfg = load_file(path)?;
    let errs = validate::errors(&cfg);
    if errs.is_empty() {
        Ok(())
    } else {
        Err(LoadError::Invalid {
            path: path.display().to_string(),
            problems: errs,
        })
    }
}
