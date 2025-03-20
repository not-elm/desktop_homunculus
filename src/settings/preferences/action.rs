mod action_name;
mod action_tags;

use bevy::prelude::{Deref, Resource};
use bevy::utils::hashbrown::HashMap;
use rand::prelude::IteratorRandom;
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
    pub fn random_next_action(
        &self,
        tags: &ActionTags,
        current: &ActionName,
    ) -> Option<&ActionName> {
        self.0
            .iter()
            .filter(|(name, _)| name != &current)
            .filter_map(|(name, properties)| {
                tags.iter()
                    .any(|tag| properties.tags.contains(tag))
                    .then_some(name)
            })
            .choose(&mut rand::rng())
    }

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
            ActionName::idle(),
            ActionProperties {
                tags: vec!["idle"].into(),
                actions: vec![
                    MascotAction::animation("idle.vrma", true),
                    MascotAction::range_timer(10f32..60.),
                    MascotAction::auto_transition(),
                ],
            },
        );
        actions.insert(
            ActionName::from("peace"),
            ActionProperties {
                tags: vec!["idle"].into(),
                actions: vec![
                    MascotAction::animation("peace.vrma", false),
                    MascotAction::wait_animation(),
                    MascotAction::transition(ActionName::idle()),
                ],
            },
        );
        actions.insert(
            ActionName::from("destroy"),
            ActionProperties {
                tags: vec!["idle"].into(),
                actions: vec![
                    MascotAction::animation("destroy.vrma", false),
                    MascotAction::wait_animation(),
                    MascotAction::transition(ActionName::idle()),
                ],
            },
        );
        actions.insert(
            ActionName::from("rotate"),
            ActionProperties {
                tags: vec!["idle"].into(),
                actions: vec![
                    MascotAction::animation("rotate.vrma", false),
                    MascotAction::wait_animation(),
                    MascotAction::transition(ActionName::idle()),
                ],
            },
        );
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
        actions.insert(
            ActionName::sit_down(),
            ActionProperties {
                tags: vec!["sitting"].into(),
                actions: vec![
                    MascotAction::animation("sit_down.vrma", false),
                    MascotAction::wait_animation(),
                    MascotAction::transition(ActionName::sitting()),
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
    use crate::settings::preferences::action::{ActionName, ActionPreferences, ActionProperties};
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

    #[test]
    fn return_none_if_has_not_actions() {
        let preferences = ActionPreferences(HashMap::default());
        let next = preferences.random_next_action(&vec!["drag"].into(), &"idle".into());
        assert_eq!(next, None);
    }

    #[test]
    fn return_action_has_same_tag() {
        let mut preferences = ActionPreferences(HashMap::default());
        preferences.0.insert(
            ActionName::drop(),
            ActionProperties {
                tags: vec!["drag"].into(),
                actions: vec![],
            },
        );
        let next = preferences.random_next_action(&vec!["drag"].into(), &"idle".into());
        assert_eq!(next, Some(&ActionName::drop()));
    }
}
