use homunculus_prefs::PrefsDatabase;
use homunculus_utils::error::{UtilError, UtilResult};

pub(super) fn cmd_delete(db: &PrefsDatabase, key: &str) -> UtilResult {
    if let Err(e) = db.delete(key) {
        return Err(UtilError::Other(anyhow::anyhow!(e)));
    }
    Ok(())
}
