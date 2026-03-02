use anyhow::{self};
use homunculus_utils::{
    config::HomunculusConfig,
    error::{UtilError, UtilResult},
};

pub(super) fn cmd_set(key: &str, value: &str) -> UtilResult {
    let config = HomunculusConfig::load()?;
    let new_config = match apply_set(&config, key, value) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };
    new_config.save()?;
    Ok(())
}

/// Applies a set operation to the config. Returns the validated new config or an error.
fn apply_set(config: &HomunculusConfig, key: &str, value: &str) -> UtilResult<HomunculusConfig> {
    let table_value = toml::Value::try_from(config).expect("config serialization failed");
    let mut map = match table_value {
        toml::Value::Table(t) => t,
        _ => unreachable!("config must serialize to table"),
    };

    // Validate key exists
    if !map.contains_key(key) {
        let valid: Vec<&String> = map.keys().collect();
        return Err(UtilError::Other(anyhow::anyhow!(
            "unknown config key '{key}'. valid keys: {}",
            valid
                .iter()
                .map(|k| k.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )));
    }

    // Parse value as TOML literal. Wrap in a dummy key to parse as valid TOML.
    let toml_str = format!("v = {value}");
    let parsed: toml::Value = match toml::from_str(&toml_str) {
        Ok(v) => v,
        Err(_) => {
            // Fall back to treating the value as a string
            let toml_str = format!("v = \"{value}\"");
            toml::from_str(&toml_str).map_err(|e| anyhow::anyhow!("invalid value: {e}"))?
        }
    };
    let parsed_value = parsed
        .as_table()
        .and_then(|t| t.get("v"))
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("failed to parse value"))?;

    map.insert(key.to_string(), parsed_value);

    // Validate by deserializing back to HomunculusConfig
    let new_config: HomunculusConfig = toml::Value::Table(map)
        .try_into()
        .map_err(|e: toml::de::Error| anyhow::anyhow!("invalid value for '{key}': {e}"))?;

    Ok(new_config)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::config::set::apply_set;
    use homunculus_utils::config::HomunculusConfig;

    #[test]
    fn test_set_valid_port() {
        let config = HomunculusConfig::default();
        let result = apply_set(&config, "port", "8080");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().port, 8080);
    }

    #[test]
    fn test_set_valid_mods_dir() {
        let config = HomunculusConfig::default();
        let result = apply_set(&config, "mods_dir", "/custom/path");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().mods_dir, PathBuf::from("/custom/path"));
    }

    #[test]
    fn test_set_unknown_key() {
        let config = HomunculusConfig::default();
        let result = apply_set(&config, "nonexistent", "value");
        assert!(result.is_err());
    }

    #[test]
    fn test_set_invalid_type() {
        let config = HomunculusConfig::default();
        let result = apply_set(&config, "port", "not_a_number");
        assert!(result.is_err());
    }
}
