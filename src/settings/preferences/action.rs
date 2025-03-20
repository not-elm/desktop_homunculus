mod action_name;
mod action_tags;

use bevy::prelude::{Component, Deref, Reflect, Resource};
use bevy::utils::hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::mascot::action::MascotAction;
pub use action_name::ActionName;
pub use action_tags::ActionTags;

#[macro_export]
macro_rules! actions {
    ( $( $key:literal: $value:expr ),* $(,)? ) => {{
        let mut map = bevy::utils::HashMap::new();
        $( map.insert(ActionName::from($key), $value); )*
        map
    }};
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Resource, Deref)]
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

impl Default for ActionPreferences {
    fn default() -> Self {
        let mut actions = HashMap::new();

        actions.insert(
            ActionName::drag_start(),
            ActionProperties {
                tags: vec!["drag"].into(),
                actions: vec![
                    MascotAction::animation("drag_start.vrma", false),
                    MascotAction::wait_animation(),
                    MascotAction::transition(ActionName::drag()),
                ],
            },
        );
        actions.insert(
            ActionName::drag(),
            ActionProperties {
                tags: vec!["drag"].into(),
                actions: vec![MascotAction::animation("drag.vrma", true)],
            },
        );

        actions.insert(
            ActionName::drop(),
            ActionProperties {
                tags: vec!["drag"].into(),
                actions: vec![
                    MascotAction::animation("drop.vrma", false),
                    MascotAction::wait_animation(),
                    MascotAction::transition(ActionName::idle()),
                ],
            },
        );
        Self(actions)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct ActionProperties {
    pub tags: ActionTags,
    pub actions: Vec<MascotAction>,
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
