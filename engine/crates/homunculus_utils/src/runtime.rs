//! Centralized runtime path resolver for Node.js, pnpm, and tsx.
//!
//! Resolves bundled runtime paths (inside the app bundle) first, falling back
//! to system PATH for development mode. All `node`/`pnpm`/`tsx` invocations
//! across the engine should go through [`RuntimeResolver`].
//!
//! # Bundled layout
//!
//! ```text
//! runtime/
//! ├── node/bin/node(.exe)
//! ├── pnpm/bin/pnpm.cjs
//! └── tsx/node_modules/tsx/dist/esm/index.mjs
//! ```
//!
//! - macOS: `<app>/Contents/Resources/runtime/`
//! - Windows: `<exe_dir>/runtime/`

#[cfg(feature = "bevy")]
use bevy::prelude::Resource;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::process::CommandNoWindow;

/// Minimum supported Node.js major version.
const MIN_NODE_MAJOR: u32 = 22;

/// Centralized runtime path resolver.
///
/// Discovers bundled `node`, `pnpm`, and `tsx` inside the application bundle.
/// Falls back to system PATH when no bundled runtime is found (development mode).
///
/// Registered as a Bevy [`Resource`] when the `bevy` feature is enabled.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Resource))]
pub struct RuntimeResolver {
    bundled_runtime_dir: Option<PathBuf>,
}

impl RuntimeResolver {
    /// Construct from an explicit application executable path.
    ///
    /// Detects the bundled runtime directory based on platform conventions:
    /// - macOS: `<exe>/../Resources/runtime/`
    /// - Windows: `<exe_dir>/runtime/`
    ///
    /// If the expected `node` binary does not exist at the bundled path,
    /// `bundled_runtime_dir` is set to `None` (fallback mode).
    pub fn new(app_exe: &Path) -> Self {
        let runtime_dir = detect_bundled_runtime_dir(app_exe);
        let validated = runtime_dir.filter(|dir| node_binary_path(dir).exists());
        Self {
            bundled_runtime_dir: validated,
        }
    }

    /// Auto-detect from `std::env::current_exe()`.
    ///
    /// Convenience constructor for both engine and CLI contexts.
    /// Falls back to unbundled mode if the executable path cannot be determined.
    pub fn detect() -> Self {
        match std::env::current_exe() {
            Ok(exe) => Self::new(&exe),
            Err(_) => Self {
                bundled_runtime_dir: None,
            },
        }
    }

    /// Whether using the bundled runtime (`true`) or system fallback (`false`).
    pub fn is_bundled(&self) -> bool {
        self.bundled_runtime_dir.is_some()
    }

    /// Path to the `node` binary.
    ///
    /// Bundled: absolute path inside the runtime directory.
    /// Fallback: `"node"` (resolved via system PATH).
    pub fn node(&self) -> PathBuf {
        match &self.bundled_runtime_dir {
            Some(dir) => node_binary_path(dir),
            None => PathBuf::from("node"),
        }
    }

    /// Build a [`Command`] for `node --import tsx <args...>`.
    ///
    /// In bundled mode, tsx is referenced by absolute path so that
    /// `--import` resolves without `node_modules/` lookup.
    /// In fallback mode, uses the bare specifier `tsx` (resolved from
    /// `node_modules/` in the mods directory).
    pub fn node_command_with_tsx(&self) -> Command {
        let mut cmd = Command::new(self.node());
        cmd.arg("--import");
        match &self.bundled_runtime_dir {
            Some(dir) => {
                cmd.arg(tsx_import_path(dir));
            }
            None => {
                cmd.arg("tsx");
            }
        }
        cmd
    }

    /// Build a [`Command`] for pnpm.
    ///
    /// Bundled: `<node> <pnpm.cjs>` (invoked via Node.js).
    /// Fallback: `pnpm` / `pnpm.cmd` (system PATH).
    pub fn pnpm_command(&self) -> Command {
        match &self.bundled_runtime_dir {
            Some(dir) => {
                let mut cmd = Command::new(self.node());
                cmd.arg(pnpm_script_path(dir));
                cmd
            }
            None => {
                #[allow(unused_mut)]
                let mut cmd = Command::new(pnpm_system_program());
                #[cfg(windows)]
                if let Some(path) = crate::process::path_with_node_prepended() {
                    cmd.env("PATH", path);
                }
                cmd
            }
        }
    }

    /// Returns the program and initial arguments for pnpm invocation.
    ///
    /// Use this when you need to construct a `tokio::process::Command` or
    /// other non-std Command type. The returned args should be prepended
    /// before any pnpm subcommand arguments.
    pub fn pnpm_program_and_args(&self) -> (PathBuf, Vec<OsString>) {
        match &self.bundled_runtime_dir {
            Some(dir) => {
                let program = self.node();
                let args = vec![OsString::from(pnpm_script_path(dir))];
                (program, args)
            }
            None => {
                let program = PathBuf::from(pnpm_system_program());
                (program, vec![])
            }
        }
    }

    /// Validate that the resolved `node` binary meets the minimum version
    /// requirement (>= 22).
    ///
    /// Runs `node --version` and parses the output. Returns `Ok(())` if
    /// the version is sufficient, or an error describing the problem.
    pub fn validate_node_version(&self) -> Result<(), RuntimeError> {
        let output = Command::new(self.node())
            .no_window()
            .arg("--version")
            .output()
            .map_err(|e| RuntimeError::NodeNotFound(e.to_string()))?;

        if !output.status.success() {
            return Err(RuntimeError::NodeNotFound(
                "node --version exited with non-zero status".to_string(),
            ));
        }

        let version_str = String::from_utf8_lossy(&output.stdout);
        let major = parse_node_major(version_str.trim())?;

        if major < MIN_NODE_MAJOR {
            return Err(RuntimeError::UnsupportedVersion {
                found: version_str.trim().to_string(),
                minimum: MIN_NODE_MAJOR,
            });
        }

        Ok(())
    }
}

/// Errors from runtime resolution and validation.
#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("Node.js not found: {0}")]
    NodeNotFound(String),
    #[error("Node.js version {found} is below minimum v{minimum}")]
    UnsupportedVersion { found: String, minimum: u32 },
    #[error("failed to parse Node.js version: {0}")]
    VersionParse(String),
}

// ── Platform-specific path helpers ──────────────────────────────────────────

/// Detect the bundled runtime directory relative to the application executable.
fn detect_bundled_runtime_dir(app_exe: &Path) -> Option<PathBuf> {
    let exe_dir = app_exe.parent()?;

    if cfg!(target_os = "macos") {
        // macOS: <app>/Contents/MacOS/<exe> → <app>/Contents/Resources/runtime/
        let resources = exe_dir.parent()?.join("Resources").join("runtime");
        if resources.is_dir() {
            return Some(resources);
        }
    }

    // Windows / fallback: <exe_dir>/runtime/
    let runtime = exe_dir.join("runtime");
    if runtime.is_dir() {
        return Some(runtime);
    }

    None
}

/// Path to the `node` binary inside the bundled runtime directory.
fn node_binary_path(runtime_dir: &Path) -> PathBuf {
    if cfg!(windows) {
        runtime_dir.join("node").join("node.exe")
    } else {
        runtime_dir.join("node").join("bin").join("node")
    }
}

/// Path to the pnpm entry script inside the bundled runtime directory.
fn pnpm_script_path(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("pnpm").join("bin").join("pnpm.cjs")
}

/// Path to the tsx ESM entry point for `--import`.
fn tsx_import_path(runtime_dir: &Path) -> PathBuf {
    runtime_dir
        .join("tsx")
        .join("node_modules")
        .join("tsx")
        .join("dist")
        .join("esm")
        .join("index.mjs")
}

/// Returns the correct program name for pnpm on the current platform.
fn pnpm_system_program() -> &'static str {
    if cfg!(windows) {
        "pnpm.cmd"
    } else {
        "pnpm"
    }
}

/// Parse the major version number from a Node.js version string (e.g. `v22.16.0`).
fn parse_node_major(version: &str) -> Result<u32, RuntimeError> {
    let stripped = version.strip_prefix('v').unwrap_or(version);
    let major_str = stripped.split('.').next().unwrap_or("");
    major_str
        .parse::<u32>()
        .map_err(|_| RuntimeError::VersionParse(version.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_node_major_valid() {
        assert_eq!(parse_node_major("v22.16.0").unwrap(), 22);
        assert_eq!(parse_node_major("v18.0.0").unwrap(), 18);
        assert_eq!(parse_node_major("v25.5.1").unwrap(), 25);
    }

    #[test]
    fn test_parse_node_major_no_prefix() {
        assert_eq!(parse_node_major("22.16.0").unwrap(), 22);
    }

    #[test]
    fn test_parse_node_major_invalid() {
        assert!(parse_node_major("not-a-version").is_err());
        assert!(parse_node_major("").is_err());
    }

    #[test]
    fn test_node_binary_path_unix() {
        if !cfg!(windows) {
            let dir = PathBuf::from("/app/runtime");
            assert_eq!(
                node_binary_path(&dir),
                PathBuf::from("/app/runtime/node/bin/node")
            );
        }
    }

    #[test]
    fn test_pnpm_script_path() {
        let dir = PathBuf::from("/app/runtime");
        assert_eq!(
            pnpm_script_path(&dir),
            PathBuf::from("/app/runtime/pnpm/bin/pnpm.cjs")
        );
    }

    #[test]
    fn test_tsx_import_path() {
        let dir = PathBuf::from("/app/runtime");
        assert_eq!(
            tsx_import_path(&dir),
            PathBuf::from("/app/runtime/tsx/node_modules/tsx/dist/esm/index.mjs")
        );
    }

    #[test]
    fn test_detect_returns_none_for_nonexistent() {
        let resolver = RuntimeResolver::new(Path::new("/nonexistent/bin/app"));
        assert!(!resolver.is_bundled());
        assert_eq!(resolver.node(), PathBuf::from("node"));
    }

    #[test]
    fn test_pnpm_program_and_args_fallback() {
        let resolver = RuntimeResolver::new(Path::new("/nonexistent/bin/app"));
        let (program, args) = resolver.pnpm_program_and_args();
        assert_eq!(program, PathBuf::from(pnpm_system_program()));
        assert!(args.is_empty());
    }

    #[test]
    fn test_pnpm_program_and_args_bundled() {
        // Simulate bundled mode with a temp directory
        let temp = std::env::temp_dir().join("hmcs-test-runtime");
        let node_dir = if cfg!(windows) {
            temp.join("node")
        } else {
            temp.join("node").join("bin")
        };
        let _ = std::fs::create_dir_all(&node_dir);
        let node_bin = if cfg!(windows) {
            node_dir.join("node.exe")
        } else {
            node_dir.join("node")
        };
        let _ = std::fs::write(&node_bin, "fake");

        let resolver = RuntimeResolver {
            bundled_runtime_dir: Some(temp.clone()),
        };
        assert!(resolver.is_bundled());
        let (program, args) = resolver.pnpm_program_and_args();
        assert_eq!(program, node_bin);
        assert_eq!(args.len(), 1);

        // Cleanup
        let _ = std::fs::remove_dir_all(&temp);
    }
}
