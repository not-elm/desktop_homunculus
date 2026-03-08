use anyhow;
use homunculus_utils::{
    config::HomunculusConfig,
    error::{UtilError, UtilResult},
};

use super::format_value;

pub(super) fn cmd_reset(key: Option<&str>, all: bool) -> UtilResult {
    match apply_reset(key, all) {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

fn apply_reset(key: Option<&str>, all: bool) -> UtilResult {
    if !all && key.is_none() {
        return Err(UtilError::Other(anyhow::anyhow!(
            "specify a key to reset, or use --all to reset all config"
        )));
    }

    if all {
        let config = HomunculusConfig::default();
        config.save()?;
        println!("all config reset to defaults");
        return Ok(());
    }

    let key = key.unwrap();

    let defaults = HomunculusConfig::default();
    let default_table = toml::Value::try_from(&defaults).expect("config serialization failed");
    let default_map = default_table.as_table().unwrap();

    if !default_map.contains_key(key) {
        let valid: Vec<&String> = default_map.keys().collect();
        return Err(UtilError::Other(anyhow::anyhow!(
            "unknown config key '{key}'. valid keys: {}",
            valid
                .iter()
                .map(|k| k.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )));
    }

    let mut config = HomunculusConfig::load()?;
    let mut table = toml::Value::try_from(&config).expect("config serialization failed");
    let map = table.as_table_mut().unwrap();

    let default_value = default_map[key].clone();
    map.insert(key.to_string(), default_value.clone());

    config = table
        .try_into()
        .map_err(|e: toml::de::Error| anyhow::anyhow!("failed to reset '{key}': {e}"))?;
    config.save()?;

    println!("{key} = {}", format_value(&default_value));
    Ok(())
}
