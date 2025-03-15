use crate::new_type;
use bevy::prelude::{Component, Reflect};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Debug, Default, Component, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub struct MascotAction {
    pub group: ActionGroup,
    pub name: ActionName,
}

new_type!(ActionGroup, String);
new_type!(ActionName, String);

impl MascotAction {
    pub fn new(vrma_path: &Path) -> Option<Self> {
        if vrma_path.extension()?.to_str()? != "vrma" {
            return None;
        }
        let main = vrma_path.parent()?.file_name()?.to_str()?;
        let sub = vrma_path.file_stem()?.to_str()?;
        Some(MascotAction {
            group: main.into(),
            name: sub.into(),
        })
    }

    pub fn from_group(group: ActionGroup) -> Self {
        MascotAction {
            group,
            name: ActionName::default(),
        }
    }
}

impl Display for MascotAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", self.group, self.name))
    }
}

impl ActionGroup {
    const IDLE: &'static str = "idle";
    const DRAG: &'static str = "drag";
    const SIT_DOWN: &'static str = "sit_down";

    pub fn idle() -> Self {
        Self::from(Self::IDLE)
    }

    pub fn drag() -> Self {
        Self::from(Self::DRAG)
    }

    pub fn sit_down() -> Self {
        Self::from(Self::SIT_DOWN)
    }

    #[inline]
    pub fn is_sit_down(&self) -> bool {
        self.0 == Self::SIT_DOWN
    }

    #[inline]
    pub fn is_drag(&self) -> bool {
        self.0 == Self::DRAG
    }
}

impl Default for ActionGroup {
    fn default() -> Self {
        Self::idle()
    }
}

impl ActionName {
    pub const INDEX: &'static str = "index";
    pub const SITTING: &'static str = "sitting";
    pub const HOLD: &'static str = "hold";
    pub const DROP: &'static str = "drop";

    pub fn index() -> Self {
        Self::from(Self::INDEX)
    }

    pub fn sitting() -> Self {
        Self::from(Self::SITTING)
    }

    pub fn hold() -> Self {
        Self::from(Self::HOLD)
    }

    pub fn drop() -> Self {
        Self::from(Self::DROP)
    }

    #[inline]
    pub fn is_index(&self) -> bool {
        self.0 == Self::INDEX
    }
}

impl Default for ActionName {
    fn default() -> Self {
        Self::index()
    }
}

#[cfg(test)]
mod tests {
    use crate::settings::state::{ActionGroup, ActionName, MascotAction};
    use std::path::PathBuf;

    #[test]
    fn correct_animation() {
        let vrma_path = PathBuf::from("idle/index.vrma");
        let animation = MascotAction::new(&vrma_path);
        assert_eq!(animation, Some(MascotAction {
            group: ActionGroup::idle(),
            name: ActionName::from("index"),
        }));
    }

    #[test]
    fn check_extension() {
        let vrma_path = PathBuf::from("idle/index.exe");
        let animation = MascotAction::new(&vrma_path);
        assert_eq!(animation, None);
    }
}