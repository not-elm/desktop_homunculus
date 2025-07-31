use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
use macos::*;
#[cfg(target_os = "windows")]
use windows::*;

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
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        return Self(all_displays());
        //TODO: Implement for other platforms
        Self(vec![])
    }

    pub fn find_by_id(&self, id: DisplayId) -> Option<&GlobalDisplay> {
        self.0.iter().find(|d| d.id == id)
    }
}
