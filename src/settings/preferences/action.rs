use crate::new_type;
use bevy::prelude::{Component, Deref, Reflect, Resource};
use bevy::utils::hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! actions {
    ( $( $key:literal: $value:expr ),* $(,)? ) => {{
        let mut map = bevy::utils::HashMap::new();
        $( map.insert(ActionName::from($key), $value); )*
        map
    }};
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Resource, Deref, Default)]
pub struct ActionPreferences(pub(crate) HashMap<ActionName, ActionProperties>);

impl ActionPreferences {
    pub fn cleanup(
        &mut self,
        exists_actions: &[ActionName],
    ) {
        self.0 = self
            .0
            .iter()
            .filter_map(|(k, v)| exists_actions.contains(k).then_some((k.clone(), v.clone())))
            .collect::<HashMap<_, _>>();
    }

    pub fn register_if_not_exists(
        &mut self,
        name: ActionName,
        action: ActionProperties,
    ) {
        self.0.entry(name).or_insert(action);
    }

    pub fn update(
        &mut self,
        name: ActionName,
        properties: ActionProperties,
    ) {
        self.0.entry(name).insert(properties);
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct ActionProperties {
    pub tags: ActionTags,
    pub action_id: String,
    pub params: String,
}

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect, Component,
)]
pub struct ActionTags(pub Vec<String>);

impl ActionTags {
    pub fn contains(
        &self,
        tag: &str,
    ) -> bool {
        self.0.contains(&tag.to_string())
    }
}

impl From<Vec<&str>> for ActionTags {
    fn from(value: Vec<&str>) -> Self {
        Self(value.iter().map(|v| v.to_string()).collect())
    }
}

new_type!(ActionName, String);

impl ActionName {
    pub const IDLE: &'static str = "idle";
    pub const SIT_DOWN: &'static str = "sit_down";
    pub const SITTING: &'static str = "sitting";
    pub const DRAG: &'static str = "drag";
    pub const DRAG_START: &'static str = "drag_start";
    pub const DROP: &'static str = "drag_drop";

    pub fn idle() -> Self {
        Self::from(Self::IDLE)
    }

    pub fn sit_down() -> Self {
        Self::from(Self::SIT_DOWN)
    }

    pub fn sitting() -> Self {
        Self::from(Self::SITTING)
    }

    pub fn drag_start() -> ActionName {
        ActionName::from(Self::DRAG_START)
    }

    pub fn drag() -> Self {
        Self::from(Self::DRAG)
    }

    pub fn drop() -> Self {
        Self::from(Self::DROP)
    }

    #[inline]
    pub fn is_index(&self) -> bool {
        self.0 == Self::IDLE
    }

    #[inline]
    pub fn is_sit_down(&self) -> bool {
        self.0 == Self::SIT_DOWN
    }

    #[inline]
    pub fn is_sitting(&self) -> bool {
        self.0 == Self::SITTING
    }

    #[inline]
    pub fn is_drag_start(&self) -> bool {
        self.0 == Self::DRAG_START
    }
}

impl Default for ActionName {
    fn default() -> Self {
        Self::idle()
    }
}

#[cfg(test)]
mod tests {
    use crate::settings::preferences::action::{ActionPreferences, ActionProperties};
    use bevy::prelude::default;
    use bevy::utils::HashMap;

    #[test]
    fn test_cleanup() {
        let mut preferences = ActionPreferences(HashMap::default());
        preferences.0.insert(
            "rotate".into(),
            ActionProperties {
                tags: vec!["idle"].into(),
                ..default()
            },
        );
        preferences.cleanup(&["rotate".into()]);
        assert_eq!(preferences.0.len(), 1);
    }

    #[test]
    fn test_remove_all() {
        let mut preferences = ActionPreferences(HashMap::default());
        preferences.0.insert(
            "rotate".into(),
            ActionProperties {
                tags: vec!["idle"].into(),
                ..default()
            },
        );
        preferences.cleanup(&[]);
        assert!(preferences.0.is_empty());
    }
}
