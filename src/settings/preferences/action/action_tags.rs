use bevy::prelude::{Component, Deref, Reflect};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect, Component, Deref,
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
