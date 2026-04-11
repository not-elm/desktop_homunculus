//! Managed process component and spawn helper.
//!
//! A managed process is a long-running MOD command process started on demand
//! and managed by the engine with full lifecycle guarantees.

use crate::mod_service::{append_pid_file, build_process_handle};
#[cfg(doc)]
use crate::node_process::NodeProcessHandle;
use bevy::prelude::*;
use chrono::{DateTime, Utc};
use homunculus_core::prelude::{HomunculusConfig, ModRegistry};
use homunculus_utils::prelude::ModInfo;
use homunculus_utils::process::CommandNoWindow;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

/// Maximum number of concurrent managed processes.
pub const MAX_PROCESSES: usize = 64;

/// Metadata for a managed long-running process.
///
/// Each managed process is an ECS entity with both `ManagedProcess` and
/// [`NodeProcessHandle`] components. The entity is despawned when the
/// process exits or is explicitly stopped.
#[derive(Component)]
pub struct ManagedProcess {
    /// Unique handle identifier (UUID).
    pub handle_id: String,
    /// Full MOD command reference (`mod-name:bin-name`).
    pub command: String,
    /// Arguments forwarded to the process.
    pub args: Vec<String>,
    /// When the process was started.
    pub started_at: DateTime<Utc>,
    /// OS process ID.
    pub pid: u32,
}

/// Result of successfully spawning a managed process.
pub struct SpawnResult {
    pub handle_id: String,
    pub pid: u32,
    pub started_at: DateTime<Utc>,
}

/// Spawns a managed process and inserts the ECS entity.
///
/// Resolves the command to a bin script path via the MOD registry,
/// spawns `node --import tsx <script> <args>`, creates a
/// [`NodeProcessHandle`], and inserts both components on a new entity.
pub fn spawn_managed_process(
    commands: &mut Commands,
    registry: &ModRegistry,
    config: &HomunculusConfig,
    command: &str,
    args: Vec<String>,
) -> Result<SpawnResult, String> {
    let (mod_name, bin_name) = parse_command(command)?;
    let mod_info = registry
        .find_by_name(mod_name)
        .ok_or_else(|| format!("Mod not found: {mod_name}"))?;

    let bin_script = find_bin_script(mod_info, bin_name)?;

    let child = spawn_node_process(&bin_script, &args, &config.mods_dir, mod_name)?;
    let pid = child.id();
    let handle_id = uuid::Uuid::new_v4().to_string();
    let started_at = Utc::now();

    append_pid_file(pid);

    let log_prefix = format!("{mod_name}:{}", &handle_id[..8]);
    let handle = build_process_handle(child, &log_prefix);

    let managed = ManagedProcess {
        handle_id: handle_id.clone(),
        command: command.to_string(),
        args,
        started_at,
        pid,
    };

    commands.spawn((managed, handle));

    Ok(SpawnResult {
        handle_id,
        pid,
        started_at,
    })
}

/// Parse `"@hmcs/persona:default-behavior"` into `("@hmcs/persona", "default-behavior")`.
fn parse_command(command: &str) -> Result<(&str, &str), String> {
    command
        .rsplit_once(':')
        .ok_or_else(|| format!("Invalid command format (expected 'mod-name:bin-name'): {command}"))
}

/// Find the bin script path from mod info's package.json bin declarations.
///
/// Reads the mod's `package.json` `bin` field to locate the actual script file.
fn find_bin_script(mod_info: &ModInfo, bin_name: &str) -> Result<PathBuf, String> {
    let pkg_path = mod_info.mod_dir.join("package.json");
    let pkg_content = std::fs::read_to_string(&pkg_path)
        .map_err(|e| format!("Failed to read {}: {e}", pkg_path.display()))?;
    let pkg: serde_json::Value = serde_json::from_str(&pkg_content)
        .map_err(|e| format!("Failed to parse {}: {e}", pkg_path.display()))?;

    let bin_path = pkg
        .get("bin")
        .and_then(|b| b.get(bin_name))
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("Bin '{}' not declared in {}", bin_name, pkg_path.display()))?;

    Ok(mod_info.mod_dir.join(bin_path))
}

fn spawn_node_process(
    script_path: &Path,
    args: &[String],
    mods_dir: &Path,
    mod_name: &str,
) -> Result<Child, String> {
    Command::new("node")
        .no_window_process_group()
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("--import")
        .arg("tsx")
        .arg(script_path)
        .args(args)
        .current_dir(mods_dir)
        .env("HMCS_MOD_NAME", mod_name)
        .spawn()
        .map_err(|e| format!("Failed to spawn process: {e}"))
}
