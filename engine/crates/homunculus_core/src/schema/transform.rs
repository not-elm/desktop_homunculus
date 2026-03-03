use bevy::math::{Quat, Vec3};
use bevy::prelude::Transform;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransformArgs {
    pub translation: Option<Vec3>,
    pub rotation: Option<Quat>,
    pub scale: Option<Vec3>,
}

impl TransformArgs {
    pub fn as_transform(&self) -> Transform {
        Transform {
            translation: self.translation.unwrap_or_default(),
            rotation: self.rotation.unwrap_or(Quat::IDENTITY),
            scale: self.scale.unwrap_or(Vec3::ONE),
        }
    }
}
