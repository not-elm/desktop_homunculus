use crate::{
    error::{ConfigError, UtilResult},
    path::homunculus_dir,
};
#[cfg(feature = "bevy")]
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn default_mods_dir() -> PathBuf {
    crate::path::mod_dir()
}

fn default_port() -> u16 {
    3100
}

/// Application configuration stored at `~/.homunculus/config.toml`.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Resource))]
#[serde(deny_unknown_fields)]
pub struct HomunculusConfig {
    /// MOD installation directory (default: `~/.homunculus/mods/`).
    #[serde(default = "default_mods_dir")]
    pub mods_dir: PathBuf,

    /// HTTP server port for the engine (default: `3100`).
    #[serde(default = "default_port")]
    pub port: u16,
}

impl Default for HomunculusConfig {
    fn default() -> Self {
        Self {
            mods_dir: default_mods_dir(),
            port: default_port(),
        }
    }
}

impl HomunculusConfig {
    /// Returns the path to the config file: `~/.homunculus/config.toml`.
    pub fn path() -> PathBuf {
        homunculus_dir().join("config.toml")
    }

    /// Loads config from `~/.homunculus/config.toml`.
    ///
    /// Returns `HomunculusConfig::default()` if the file doesn't exist.
    pub fn load() -> UtilResult<Self> {
        let path = Self::path();
        if path.exists() {
            let content =
                std::fs::read_to_string(&path).map_err(|e| ConfigError::Read(path.clone(), e))?;
            let config = toml::from_str(&content).map_err(|e| ConfigError::Parse(path, e))?;
            return Ok(config);
        }

        Ok(Self::default())
    }

    /// Loads the raw TOML table from `~/.homunculus/config.toml`.
    ///
    /// Returns an empty table if the file doesn't exist.
    pub fn load_raw() -> UtilResult<toml::map::Map<String, toml::Value>> {
        let path = Self::path();
        if path.exists() {
            let content =
                std::fs::read_to_string(&path).map_err(|e| ConfigError::Read(path.clone(), e))?;
            let value: toml::Value =
                toml::from_str(&content).map_err(|e| ConfigError::Parse(path, e))?;
            if let toml::Value::Table(table) = value {
                return Ok(table);
            }
        }
        Ok(toml::map::Map::new())
    }

    /// Saves config to `~/.homunculus/config.toml`.
    /// Creates the parent directory if it doesn't exist.
    pub fn save(&self) -> UtilResult {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| ConfigError::Write(path.clone(), e))?;
        }
        let content = toml::to_string_pretty(self).map_err(ConfigError::Serialize)?;
        std::fs::write(&path, content).map_err(|e| ConfigError::Write(path, e))?;
        Ok(())
    }

    /// Returns the host address (`127.0.0.1:<port>`).
    pub fn host(&self) -> String {
        format!("127.0.0.1:{}", self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = HomunculusConfig::default();
        assert_eq!(config.port, 3100);
        assert_eq!(config.mods_dir, crate::path::mod_dir());
    }

    #[test]
    fn test_config_roundtrip() {
        let config = HomunculusConfig {
            mods_dir: PathBuf::from("/custom/mods"),
            port: 4000,
        };
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed: HomunculusConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.mods_dir, config.mods_dir);
        assert_eq!(parsed.port, config.port);
    }

    #[test]
    fn test_config_empty_toml() {
        let config: HomunculusConfig = toml::from_str("").unwrap();
        assert_eq!(config.port, 3100);
        assert_eq!(config.mods_dir, crate::path::mod_dir());
    }

    #[test]
    fn test_config_partial_toml() {
        let config: HomunculusConfig = toml::from_str("port = 3200").unwrap();
        assert_eq!(config.mods_dir, crate::path::mod_dir());
        assert_eq!(config.port, 3200);
    }

    #[test]
    fn test_host() {
        let config = HomunculusConfig {
            port: 5000,
            ..Default::default()
        };
        assert_eq!(config.host(), "127.0.0.1:5000");
    }

    #[test]
    fn test_host_default() {
        let config = HomunculusConfig::default();
        assert_eq!(config.host(), "127.0.0.1:3100");
    }

    #[test]
    fn test_config_snake_case_keys() {
        let config = HomunculusConfig {
            mods_dir: PathBuf::from("/my/mods"),
            port: 8080,
        };
        let toml_str = toml::to_string(&config).unwrap();
        assert!(
            toml_str.contains("mods_dir"),
            "expected snake_case key 'mods_dir' in TOML: {toml_str}"
        );
        assert!(
            toml_str.contains("port"),
            "expected key 'port' in TOML: {toml_str}"
        );
        assert!(
            !toml_str.contains("modsDir"),
            "unexpected camelCase key 'modsDir' in TOML: {toml_str}"
        );
    }

    #[test]
    fn test_config_deny_unknown_fields() {
        let result = toml::from_str::<HomunculusConfig>("port = 3100\nunknown = true");
        assert!(result.is_err(), "expected error for unknown field");
    }

    #[test]
    fn test_default_serializes_all_fields() {
        let config = HomunculusConfig::default();
        let table: toml::Value = toml::Value::try_from(&config).unwrap();
        let map = table.as_table().unwrap();
        assert!(map.contains_key("port"), "default must serialize 'port'");
        assert!(
            map.contains_key("mods_dir"),
            "default must serialize 'mods_dir'"
        );
    }
}
