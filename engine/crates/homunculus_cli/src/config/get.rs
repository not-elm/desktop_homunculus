use homunculus_utils::{
    config::HomunculusConfig,
    error::{UtilError, UtilResult},
};

use crate::config::format_value;

pub(super) fn cmd_get(key: &str) -> UtilResult {
    let config = HomunculusConfig::load()?;
    match get_value(&config, key) {
        Ok(value) => println!("{value}"),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}

/// Looks up a config key's value. Returns an error for unknown keys.
fn get_value(config: &HomunculusConfig, key: &str) -> UtilResult<String> {
    let table = toml::Value::try_from(config).expect("config serialization failed");
    let map = table.as_table().expect("config must serialize to table");
    match map.get(key) {
        Some(value) => Ok(format_value(value)),
        None => {
            let valid: Vec<&String> = map.keys().collect();
            Err(UtilError::Other(anyhow::anyhow!(format!(
                "unknown config key '{key}'. valid keys: {}",
                valid
                    .iter()
                    .map(|k| k.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::get::get_value;
    use homunculus_utils::config::HomunculusConfig;

    #[test]
    fn test_get_known_key() {
        let config = HomunculusConfig::default();
        let result = get_value(&config, "port");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "3100");
    }

    #[test]
    fn test_get_unknown_key() {
        let config = HomunculusConfig::default();
        let result = get_value(&config, "nonexistent");
        assert!(result.is_err());
    }
}
