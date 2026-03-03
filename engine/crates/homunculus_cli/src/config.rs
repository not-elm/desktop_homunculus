//! `hmcs config` subcommand — manage application configuration
mod get;
mod list;
mod set;

use crate::config::{get::cmd_get, list::cmd_list, set::cmd_set};
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
}

impl ConfigArgs {
    pub fn execute(self) -> UtilResult {
        match self.command {
            ConfigSubcommand::List => cmd_list(),
            ConfigSubcommand::Get { key } => cmd_get(&key),
            ConfigSubcommand::Set { key, value } => cmd_set(&key, &value),
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
