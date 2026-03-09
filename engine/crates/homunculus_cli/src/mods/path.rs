//! `hmcs mod path [mods_dir_path]` — view or update the mods directory.

use homunculus_utils::{config::HomunculusConfig, error::UtilResult};
use std::path::PathBuf;

/// Executes the `mod path` command.
///
/// Without arguments, prints the current `mods_dir`.
/// With a path argument, validates/creates the directory and updates the config.
pub(super) fn cmd_path(mods_dir_path: Option<&str>) -> UtilResult {
    match mods_dir_path {
        None => print_current_path(),
        Some(raw) => update_path(raw),
    }
}

/// Prints the current `mods_dir` config value.
fn print_current_path() -> UtilResult {
    let config = HomunculusConfig::load()?;
    println!("{}", config.mods_dir.display());
    Ok(())
}

/// Resolves the path, creates the directory if needed, and saves the config.
fn update_path(raw: &str) -> UtilResult {
    let resolved = resolve_path(raw);

    std::fs::create_dir_all(&resolved).map_err(|e| {
        anyhow::anyhow!("failed to create directory \"{}\": {e}", resolved.display())
    })?;

    let mut config = HomunculusConfig::load()?;
    config.mods_dir = resolved.clone();
    config.save()?;

    println!("mods_dir updated to: {}", resolved.display());
    Ok(())
}

/// Expands `~` to home directory and resolves relative paths to absolute.
fn resolve_path(raw: &str) -> PathBuf {
    let expanded = if raw.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            home.join(
                raw.strip_prefix("~/")
                    .or(raw.strip_prefix('~'))
                    .unwrap_or(raw),
            )
        } else {
            PathBuf::from(raw)
        }
    } else {
        PathBuf::from(raw)
    };

    // Resolve relative paths to absolute using the current working directory
    if expanded.is_relative() {
        std::env::current_dir()
            .map(|cwd| cwd.join(&expanded))
            .unwrap_or(expanded)
    } else {
        expanded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_resolve_absolute_path() {
        let result = resolve_path("/custom/mods");
        assert_eq!(result, PathBuf::from("/custom/mods"));
    }

    #[test]
    fn test_resolve_tilde_path() {
        let result = resolve_path("~/my-mods");
        let home = dirs::home_dir().unwrap();
        assert_eq!(result, home.join("my-mods"));
    }

    #[test]
    fn test_resolve_tilde_only() {
        let result = resolve_path("~");
        let home = dirs::home_dir().unwrap();
        assert_eq!(result, home);
    }

    #[test]
    fn test_resolve_relative_path() {
        let result = resolve_path("relative/path");
        assert!(result.is_absolute());
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_resolve_windows_path() {
        let result = resolve_path(r"C:\Users\elmpr\mods");
        assert_eq!(result, PathBuf::from(r"C:\Users\elmpr\mods"));
    }
}
