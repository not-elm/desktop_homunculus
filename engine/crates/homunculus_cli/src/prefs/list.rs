use comfy_table::Table;
use comfy_table::presets::NOTHING;
use homunculus_prefs::PrefsDatabase;
use homunculus_utils::error::{UtilError, UtilResult};

pub(super) fn cmd_list(db: &PrefsDatabase) -> UtilResult {
    match db.list_entries() {
        Ok(entries) if entries.is_empty() => println!("No preferences found."),
        Ok(mut entries) => {
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            println!("{}", format_table(&entries));
        }
        Err(e) => return Err(UtilError::Other(anyhow::anyhow!(e))),
    }
    Ok(())
}

fn format_table(entries: &[(String, serde_json::Value)]) -> String {
    let mut table = Table::new();
    table.load_preset(NOTHING).set_header(["KEY", "VALUE"]);
    for (key, value) in entries {
        let display = match value {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        };
        table.add_row([key.as_str(), &display]);
    }
    table.to_string()
}
