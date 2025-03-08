use crate::settings::state::{ActionGroup, ActionName, MascotAction};
use bevy::prelude::{Component, Deref, Reflect, Resource};
use bevy::utils::hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone, Hash, Component, Reflect)]
pub struct ActionProperties {
    pub is_repeat_animation: bool,
    pub transition: TransitionMode,
}

impl Default for ActionProperties {
    fn default() -> Self {
        Self {
            is_repeat_animation: false,
            transition: TransitionMode::back_to_idle(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Default, Serialize, Deserialize, Clone, Hash, Reflect)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum TransitionMode {
    #[default]
    None,
    Auto {
        min_secs: u64,
        max_secs: u64,
    },
    Manual {
        next: MascotAction,
    },
}

impl TransitionMode {
    pub fn auto(min_secs: u64, max_secs: u64) -> Self {
        TransitionMode::Auto { min_secs, max_secs }
    }

    pub fn back_to_idle() -> Self {
        Self::Manual { next: MascotAction::default() }
    }

    pub fn manual(next: MascotAction) -> Self {
        TransitionMode::Manual { next }
    }
}

macro_rules! groups {
    ( $( $key:literal: $value:expr ),* $(,)? ) => {{
        let mut map = HashMap::new();
        $( map.insert(ActionGroup::from($key), $value); )*
        map
    }};
}

macro_rules! actions {
    ( $( $key:literal: $value:expr ),* $(,)? ) => {{
        let mut map = HashMap::new();
        $( map.insert(ActionName::from($key), $value); )*
        map
    }};
}


#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Resource, Deref)]
pub struct ActionPreferences(HashMap<ActionGroup, HashMap<ActionName, ActionProperties>>);

impl ActionPreferences {
    pub fn properties(&self, action: &MascotAction) -> ActionProperties {
        self.0
            .get(&action.group)
            .and_then(|group| group.get(&action.name))
            .cloned()
            .unwrap_or_default()
    }

    pub fn cleanup(&mut self, exists_actions: &[MascotAction]) {
        let mut groups = Self(HashMap::new());
        for (action, properties) in self.0
            .iter()
            .flat_map(|(group_name, group)| {
                group.iter().map(move |(action_name, action)| {
                    (MascotAction {
                        group: group_name.clone(),
                        name: action_name.clone(),
                    }, action)
                })
            })
        {
            if exists_actions.contains(&action) {
                groups.update(action, properties.clone());
            }
        }
        self.0 = groups.0;
    }

    pub fn register_if_not_exists(&mut self, action: MascotAction) {
        self.0
            .entry(action.group)
            .or_default()
            .entry(action.name)
            .or_default();
    }

    pub fn update(&mut self, action: MascotAction, properties: ActionProperties) {
        self.0
            .entry(action.group)
            .or_default()
            .insert(action.name, properties);
    }
}


impl Default for ActionPreferences {
    fn default() -> Self {
        Self(
            groups! {
                "idle": actions!{
                    "index": ActionProperties{
                        is_repeat_animation: true,
                        transition: TransitionMode::auto(10, 60),
                    },
                },
                "drag": actions!{
                    "index": ActionProperties{
                        is_repeat_animation: false,
                        transition: TransitionMode::manual(MascotAction {
                            group: ActionGroup::drag(),
                            name: ActionName::hold(),
                        }),
                    },
                    "hold": ActionProperties{
                        is_repeat_animation: true,
                        transition: TransitionMode::None,
                    },
                },
                "sit_down": actions!{
                    "index": ActionProperties{
                        is_repeat_animation: false,
                        transition: TransitionMode::manual(MascotAction {
                            group: ActionGroup::sit_down(),
                            name: ActionName::sitting(),
                        }),
                    },
                    "sitting": ActionProperties{
                        is_repeat_animation: true,
                        transition: TransitionMode::None,
                    },
                },
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::settings::preferences::action::{ActionPreferences, ActionProperties, TransitionMode};
    use crate::settings::state::MascotAction;

    #[test]
    fn not_update_if_already_exists() {
        let expect = ActionProperties {
            transition: TransitionMode::None,
            is_repeat_animation: true,
        };
        let mut preferences = ActionPreferences::default();
        preferences.update(MascotAction::default(), expect.clone());

        preferences.register_if_not_exists(MascotAction::default());
        let properties = preferences.properties(&MascotAction::default());
        assert_eq!(properties, expect);
    }
}