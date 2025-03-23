pub mod action;

use crate::system_param::coordinate::Coordinate;
use bevy::math::{Quat, Vec3};
use bevy::platform_support::collections::HashMap;
use bevy::prelude::{Reflect, Resource, Transform};
use bevy::utils::default;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Resource, Clone, Reflect)]
pub struct MascotLocationPreferences(pub HashMap<PathBuf, MascotLocation>);

impl MascotLocationPreferences {
    pub fn load_transform(
        &self,
        path: &Path,
        coordinate: &Coordinate,
    ) -> Transform {
        match self.try_load_transform(path, coordinate) {
            Some(transform) => transform,
            None => {
                let world_pos = coordinate.default_mascot_pos_and_layers();
                Transform {
                    translation: world_pos,
                    ..default()
                }
            }
        }
    }

    fn try_load_transform(
        &self,
        path: &Path,
        coordinate: &Coordinate,
    ) -> Option<Transform> {
        let location = self.0.get(path)?;
        let world_pos =
            coordinate.mascot_position(location.viewport_pos, &location.monitor_name)?;
        Some(Transform {
            translation: world_pos,
            scale: location.scale,
            rotation: location.rotation,
        })
    }
}

#[derive(PartialEq, Debug, Default, Serialize, Deserialize, Clone, Reflect)]
pub struct MascotLocation {
    pub monitor_name: Option<String>,
    pub viewport_pos: Vec3,
    pub scale: Vec3,
    pub rotation: Quat,
}
