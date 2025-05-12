use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(target_os = "macos")]
mod macos;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deref, Serialize, Deserialize, Reflect)]
pub struct DisplayId(pub u32);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
pub struct GlobalDisplay {
    pub id: DisplayId,
    pub title: String,
    pub frame: Rect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect, Deref)]
pub struct GlobalDisplays(pub Vec<GlobalDisplay>);

impl GlobalDisplays {
    #[allow(unreachable_code)]
    pub fn find_all() -> GlobalDisplays {
        #[cfg(target_os = "macos")]
        return Self(macos::all_displays());
        //TODO: Implement for other platforms
        Self(vec![])
    }

    pub fn find_by_id(&self, id: DisplayId) -> Option<&GlobalDisplay> {
        self.0.iter().find(|d| d.id == id)
    }
}
