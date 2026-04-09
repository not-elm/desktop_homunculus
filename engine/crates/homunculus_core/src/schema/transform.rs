use bevy::math::{Quat, Vec3};
use bevy::prelude::Transform;
use serde::{Deserialize, Serialize};

/// Partial transform arguments for API requests.
/// All fields are optional — omitted fields retain their current value.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct TransformArgs {
    #[cfg_attr(feature = "openapi", schema(value_type = Option<[f32; 3]>))]
    pub translation: Option<Vec3>,
    #[cfg_attr(feature = "openapi", schema(value_type = Option<[f32; 4]>))]
    pub rotation: Option<Quat>,
    #[cfg_attr(feature = "openapi", schema(value_type = Option<[f32; 3]>))]
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
