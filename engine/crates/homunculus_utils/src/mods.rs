pub mod list;
use std::process::Command;

use crate::{
    config::HomunculusConfig,
    error::{ModsError, UtilError, UtilResult},
};

/// Validate an npm package specifier for safe use with `pnpm add`/`pnpm remove`.
///
/// Rejects shell metacharacters, path traversal, and empty names.
/// Accepts scoped (`@scope/name`) and unscoped (`name`) packages with
/// optional version specifiers (`@version`).
fn validate_package_name(spec: &str) -> UtilResult {
    if spec.is_empty() {
        return Err(UtilError::ForkProcess(
            "package name must not be empty".to_string(),
        ));
    }

    // Split off version specifier: "@scope/name@version" or "name@version"
    let name_part = if let Some(after_scope) = spec.strip_prefix('@') {
        // Scoped: find the second '@' (version separator)
        match after_scope.find('@') {
            Some(pos) => &spec[..pos + 1],
            None => spec,
        }
    } else {
        // Unscoped: find the first '@' (version separator)
        match spec.find('@') {
            Some(pos) => &spec[..pos],
            None => spec,
        }
    };

    // Reject shell metacharacters and path traversal in the name portion
    const FORBIDDEN: &[char] = &[
        ';', '&', '|', '$', '`', '(', ')', '{', '}', '<', '>', '!', '#', '\\', '"', '\'', '\n',
        '\r', ' ', '\t',
    ];
    if name_part.contains(FORBIDDEN) {
        return Err(UtilError::ForkProcess(format!(
            "invalid package name: contains forbidden characters: {name_part}"
        )));
    }
    if name_part.contains("..") {
        return Err(UtilError::ForkProcess(format!(
            "invalid package name: contains path traversal: {name_part}"
        )));
    }

    Ok(())
}

/// Pinned tsx version for deterministic mod service execution.
const TSX_PACKAGE: &str = "tsx@4.21.0";

/// Ensure tsx is installed in the mods directory.
///
/// Runs `pnpm -C <mods_dir> add --save-dev --save-exact tsx@4.21.0` on every
/// app startup. If the pinned version is already installed, pnpm resolves
/// quickly without network access.
/// The installed tsx is used by mod services via `node --import tsx`.
pub fn ensure_tsx() -> UtilResult {
    let status = create_pnpm_command_base()?
        .arg("add")
        .arg("--save-dev")
        .arg("--save-exact")
        .arg(TSX_PACKAGE)
        .status()
        .map_err(|e| UtilError::Mods(ModsError::Install(e)))?;

    if !status.success() {
        return Err(UtilError::ForkProcess(format!(
            "pnpm add {TSX_PACKAGE} failed with status: {status}"
        )));
    }
    Ok(())
}

/// Install the mod.
/// The argument `pkg` is same as `pnpm add <pkg>`.
pub fn install<S: AsRef<str>>(pkg: &[S]) -> UtilResult {
    for p in pkg {
        validate_package_name(p.as_ref())?;
    }

    let status = create_pnpm_command_base()?
        .arg("add")
        .args(pkg.iter().map(|s| s.as_ref()))
        .status()
        .map_err(|e| UtilError::Mods(ModsError::Install(e)))?;

    if !status.success() {
        return Err(UtilError::ForkProcess(format!(
            "pnpm add failed with status: {status}"
        )));
    }
    Ok(())
}

/// Uninstall the mod.
pub fn uninstall<S: AsRef<str>>(mod_names: &[S]) -> UtilResult {
    for name in mod_names {
        validate_package_name(name.as_ref())?;
    }

    let status = create_pnpm_command_base()?
        .arg("remove")
        .args(mod_names.iter().map(|s| s.as_ref()))
        .status()
        .map_err(|e| UtilError::Mods(ModsError::Uninstall(e)))?;

    if !status.success() {
        return Err(UtilError::ForkProcess(format!(
            "pnpm remove failed with status: {status}"
        )));
    }
    Ok(())
}

fn create_pnpm_command_base() -> UtilResult<Command> {
    let config = HomunculusConfig::load()?;
    let mut command = Command::new("pnpm");
    command.args(["-C", &format!("{}", &config.mods_dir.display())]);
    Ok(command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_package_name_valid() {
        assert!(validate_package_name("my-mod").is_ok());
        assert!(validate_package_name("@hmcs/elmer").is_ok());
        assert!(validate_package_name("@hmcs/elmer@1.0.0").is_ok());
        assert!(validate_package_name("some-pkg@latest").is_ok());
    }

    #[test]
    fn test_validate_package_name_empty() {
        assert!(validate_package_name("").is_err());
    }

    #[test]
    fn test_validate_package_name_shell_metachar() {
        assert!(validate_package_name("foo;rm -rf /").is_err());
        assert!(validate_package_name("foo && bar").is_err());
        assert!(validate_package_name("foo|bar").is_err());
        assert!(validate_package_name("$(whoami)").is_err());
        assert!(validate_package_name("`whoami`").is_err());
    }

    #[test]
    fn test_validate_package_name_path_traversal() {
        assert!(validate_package_name("../etc/passwd").is_err());
        assert!(validate_package_name("@hmcs/../secret").is_err());
    }
}
