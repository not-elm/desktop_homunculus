mod action_name;
mod action_tags;

use crate::mascot::action::MascotAction;
pub use action_name::ActionName;
pub use action_tags::ActionTags;
use bevy::platform_support::collections::HashMap;
use bevy::prelude::{Deref, Resource};
use rand::prelude::IteratorRandom;
use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! actions {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        let mut map = HashMap::<ActionName, ActionProperties>::default();
        $( map.insert($key, $value); )*
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
        Self(actions!(
            ActionName::idle() => ActionProperties {
                tags: vec!["idle"].into(),
                actions: vec![
                    MascotAction::animation("idle.vrma", true),
                    MascotAction::range_timer(3f32..5.),
                    MascotAction::auto_transition(),
                ],
            },
            ActionName::from("peace") => simple_animation("idle", "peace.vrma", ActionName::idle()),
            ActionName::from("destroy") => simple_animation("idle", "destroy.vrma", ActionName::idle()),
            ActionName::from("rotate") => simple_animation("idle", "rotate.vrma", ActionName::idle()),
            ActionName::drag_start() => simple_animation("drag", "drag_start.vrma", ActionName::drag()),
            ActionName::drag() => ActionProperties {
                tags: vec!["drag"].into(),
                actions: vec![
                    MascotAction::animation("drag.vrma", true),
                ],
            },
            ActionName::drop() => simple_animation("drag", "drop.vrma", ActionName::idle()),
            ActionName::sit_down() => simple_animation("sitting", "sit_down.vrma", ActionName::sitting()),
        ))
    }
}

fn simple_animation(
    tag: &str,
    vrma_name: &str,
    next: ActionName,
) -> ActionProperties {
    ActionProperties {
        tags: vec![tag].into(),
        actions: vec![
            MascotAction::animation(vrma_name, false),
            MascotAction::wait_animation(),
            MascotAction::transition(next),
        ],
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
    use bevy::platform_support::collections::HashMap;
    use bevy::prelude::default;

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
