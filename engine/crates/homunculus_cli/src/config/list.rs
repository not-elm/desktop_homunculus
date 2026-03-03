use crate::config::format_value;
use homunculus_utils::{config::HomunculusConfig, error::UtilResult};

pub(super) fn cmd_list() -> UtilResult {
    let config = HomunculusConfig::load()?;
    println!("{}", format_list(&config));
    Ok(())
}

/// Formats the config as a table sorted by key.
fn format_list(config: &HomunculusConfig) -> String {
    let toml_val = toml::Value::try_from(config).expect("config serialization failed");
    let map = toml_val.as_table().expect("config must serialize to table");

    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort();

    let mut table = comfy_table::Table::new();
    table
        .load_preset(comfy_table::presets::NOTHING)
        .set_header(["KEY", "VALUE"]);

    for key in &keys {
        table.add_row([key.as_str(), &format_value(&map[key.as_str()])]);
    }

    table.to_string()
}

#[cfg(test)]
mod tests {
    use crate::config::list::format_list;
    use homunculus_utils::config::HomunculusConfig;

    #[test]
    fn test_list_format_all_defaults() {
        let config = HomunculusConfig::default();
        let output = format_list(&config);
        assert!(output.contains("port"));
        assert!(output.contains("3100"));
        assert!(output.contains("mods_dir"));
    }

    #[test]
    fn test_list_format_with_configured_value() {
        let config = HomunculusConfig {
            port: 8080,
            ..Default::default()
        };
        let output = format_list(&config);
        assert!(output.contains("port"));
        assert!(output.contains("8080"));
    }
}
