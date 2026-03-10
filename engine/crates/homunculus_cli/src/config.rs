//! `hmcs config` subcommand — manage application configuration
mod get;
mod list;
mod reset;
mod set;

use crate::config::{get::cmd_get, list::cmd_list, reset::cmd_reset, set::cmd_set};
use clap::{Args, Subcommand};
use homunculus_utils::error::UtilResult;

/// CLI arguments for the `hmcs config` subcommand.
#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigSubcommand,
}

/// Available operations on the application configuration.
#[derive(Subcommand)]
pub enum ConfigSubcommand {
    /// List all config keys with values
    List,
    /// Get a config value by key
    Get {
        /// Config key (e.g. port, mods_dir)
        key: String,
    },
    /// Set a config value
    Set {
        /// Config key (e.g. port, mods_dir)
        key: String,
        /// Value to set
        value: String,
    },
    /// Reset a config to default value
    Reset {
        /// Config key to reset (e.g. port, mods_dir)
        key: Option<String>,
        /// Reset all config keys to defaults
        #[arg(long)]
        all: bool,
    },
}

impl ConfigArgs {
    pub fn execute(self) -> UtilResult {
        match self.command {
            ConfigSubcommand::List => cmd_list(),
            ConfigSubcommand::Get { key } => cmd_get(&key),
            ConfigSubcommand::Set { key, value } => cmd_set(&key, &value),
            ConfigSubcommand::Reset { key, all } => cmd_reset(key.as_deref(), all),
        }
    }
}

/// Formats a TOML value for display.
fn format_value(value: &toml::Value) -> String {
    match value {
        toml::Value::String(s) => s.clone(),
        toml::Value::Integer(i) => i.to_string(),
        toml::Value::Float(f) => f.to_string(),
        toml::Value::Boolean(b) => b.to_string(),
        other => other.to_string(),
    }
}
