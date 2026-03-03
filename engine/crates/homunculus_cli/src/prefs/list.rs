use homunculus_prefs::PrefsDatabase;
use homunculus_utils::error::{UtilError, UtilResult};

pub(super) fn cmd_list(db: &PrefsDatabase) -> UtilResult {
    match db.list_keys() {
        Ok(keys) if keys.is_empty() => println!("No preferences found."),
        Ok(keys) => keys.iter().for_each(|k| println!("{k}")),
        Err(e) => {
            return Err(UtilError::Other(anyhow::anyhow!(e)));
        }
    }
    Ok(())
}
