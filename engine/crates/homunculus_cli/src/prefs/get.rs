use homunculus_prefs::{PrefsDatabase, SqlValue};
use homunculus_utils::error::{UtilError, UtilResult};

pub(super) fn cmd_get(db: &PrefsDatabase, key: &str) -> UtilResult {
    match db
        .load(key)
        .map_err(|e| UtilError::Other(anyhow::anyhow!(e)))?
    {
        Some((value, value_type)) => {
            let display = match (&value, value_type.as_str()) {
                (SqlValue::Null, _) => "null".to_owned(),
                (SqlValue::Integer(i), "bool") => if *i != 0 { "true" } else { "false" }.to_owned(),
                (SqlValue::Integer(i), _) => i.to_string(),
                (SqlValue::Real(f), _) => f.to_string(),
                (SqlValue::Text(s), "json") => serde_json::from_str::<serde_json::Value>(s)
                    .and_then(|v| serde_json::to_string_pretty(&v))
                    .unwrap_or_else(|_| s.clone()),
                (SqlValue::Text(s), _) => s.clone(),
                _ => format!("{value:?}"),
            };
            println!("{display} ({value_type})");
        }
        None => {
            return Err(UtilError::Other(anyhow::anyhow!("key not found: {key}")));
        }
    }
    Ok(())
}
