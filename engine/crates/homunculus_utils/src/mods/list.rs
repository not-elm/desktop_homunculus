use crate::{
    error::{ModsError, UtilError, UtilResult},
    mods::create_pnpm_command_base,
    prelude::{ModInfo, ModPackageJson},
};
use std::{
    path::{Path, PathBuf},
    process::Output,
};

/// Retrieves list of installation mods.
pub fn list_installation_mods() -> UtilResult<Vec<ModInfo>> {
    let mods = list_candidate_paths()?;
    Ok(list_mods(&mods))
}

/// Retrieves all installation mod metadata in `mods_dir`.
/// Pass the path to the MOD from the `pnpm ls --parseable` command as the argument.
fn list_mods<P: AsRef<Path>>(mod_paths: &[P]) -> Vec<ModInfo> {
    let mut mods = vec![];
    for m in mod_paths {
        let buf = match std::fs::read_to_string(m.as_ref().join("package.json")) {
            Ok(buf) => buf,
            Err(e) => {
                let path = m.as_ref().display().to_string();
                eprintln!("failed to read mod's package.json : path={path} error={e}");
                continue;
            }
        };

        match serde_json::from_str::<ModPackageJson>(&buf) {
            Ok(pkg) => {
                mods.push(convert_to_mod_info(pkg, m.as_ref()));
            }
            Err(e) => {
                let path = m.as_ref().display().to_string();
                eprintln!("failed to desrialize mod's package.json : path={path} error={e}");
                continue;
            }
        }
    }
    mods
}

/// Returns a list of paths that may be MODs.
/// It is not guaranteed that they are actually MODs, so you need to filter them appropriately.
fn list_candidate_paths() -> UtilResult<Vec<PathBuf>> {
    let output = create_pnpm_command_base()?
        .args(["ls", "--parseable", "-P", "--depth", "0"])
        .output()
        .map_err(|e| UtilError::Mods(ModsError::List(e.to_string())))?;
    error_if_failed_pnpm_ls(&output)?;
    Ok(parse_pnpm_ls_output(&String::from_utf8_lossy(
        &output.stdout,
    )))
}

fn convert_to_mod_info(pkg: ModPackageJson, path: &Path) -> ModInfo {
    ModInfo {
        author: pkg.author,
        name: pkg.name,
        service_script_path: pkg
            .homunculus
            .service
            .and_then(|p| path.join(p).canonicalize().ok()),
        license: pkg.license,
        version: pkg.version,
        description: pkg.description,
        commands: pkg
            .bin
            .as_ref()
            .map(|b| b.keys().cloned().collect())
            .unwrap_or_default(),
        assets: pkg.homunculus.assets.unwrap_or_default(),
        menus: pkg.homunculus.menus.unwrap_or_default(),
        tray: pkg.homunculus.tray,
        mod_dir: path.to_path_buf(),
    }
}

/// Parses the stdout of `pnpm ls --parseable` into a list of mod paths.
///
/// The first line of `pnpm ls --parseable` is the root mods directory itself,
/// which is not a mod — skip it.
fn parse_pnpm_ls_output(stdout: &str) -> Vec<PathBuf> {
    stdout
        .lines()
        .skip(1)
        .filter(|line| !line.is_empty())
        .map(PathBuf::from)
        .collect()
}

fn error_if_failed_pnpm_ls(output: &Output) -> UtilResult {
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let msg = if stderr.is_empty() {
            format!("command failed with status: {}", output.status)
        } else {
            stderr.into_owned()
        };
        return Err(UtilError::Mods(ModsError::List(msg)));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pnpm_ls_output_multiple_paths() {
        let stdout = "/Users/me/.homunculus/mods\n/Users/me/.homunculus/mods/node_modules/elmer\n/Users/me/.homunculus/mods/node_modules/@hmcs/menu\n";
        let result = parse_pnpm_ls_output(stdout);
        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0],
            PathBuf::from("/Users/me/.homunculus/mods/node_modules/elmer")
        );
        assert_eq!(
            result[1],
            PathBuf::from("/Users/me/.homunculus/mods/node_modules/@hmcs/menu")
        );
    }

    #[test]
    fn parse_pnpm_ls_output_empty_string() {
        let result = parse_pnpm_ls_output("");
        assert!(result.is_empty());
    }

    #[test]
    fn parse_pnpm_ls_output_single_path_no_trailing_newline() {
        let stdout = "/Users/me/.homunculus/mods\n/Users/me/.homunculus/mods/node_modules/elmer";
        let result = parse_pnpm_ls_output(stdout);
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0],
            PathBuf::from("/Users/me/.homunculus/mods/node_modules/elmer")
        );
    }

    #[test]
    fn parse_pnpm_ls_output_trailing_newline_ignored() {
        let stdout = "/some/root\n/some/path\n";
        let result = parse_pnpm_ls_output(stdout);
        assert_eq!(
            result.len(),
            1,
            "trailing newline should not produce an empty PathBuf"
        );
    }

    #[cfg(unix)]
    fn make_output(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
        use std::os::unix::process::ExitStatusExt;
        Output {
            status: std::process::ExitStatus::from_raw(code << 8),
            stdout: stdout.to_vec(),
            stderr: stderr.to_vec(),
        }
    }

    #[cfg(unix)]
    #[test]
    fn error_if_failed_pnpm_ls_success() {
        let output = make_output(0, b"", b"");
        assert!(error_if_failed_pnpm_ls(&output).is_ok());
    }

    #[cfg(unix)]
    #[test]
    fn error_if_failed_pnpm_ls_failure_with_stderr() {
        let output = make_output(1, b"", b"ERR! something went wrong");
        let err = error_if_failed_pnpm_ls(&output).unwrap_err();
        assert!(err.to_string().contains("something went wrong"));
    }

    #[cfg(unix)]
    #[test]
    fn error_if_failed_pnpm_ls_failure_without_stderr() {
        let output = make_output(1, b"", b"");
        let err = error_if_failed_pnpm_ls(&output).unwrap_err();
        assert!(err.to_string().contains("command failed with status"));
    }
}
