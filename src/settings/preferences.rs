pub mod action;

use crate::settings::preferences::action::ActionPreferences;
use crate::system_param::cameras::Cameras;
use crate::system_param::coordinate::Coordinate;
use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{Reflect, Resource, Transform};
use bevy::render::view::RenderLayers;
use bevy::utils::{default, HashMap};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Resource, Clone, Reflect)]
pub struct MascotLocationPreferences(pub HashMap<PathBuf, MascotLocation>);

impl MascotLocationPreferences {
    pub fn load(&self, path: &Path, coordinate: &Coordinate) -> (Transform, RenderLayers) {
        match self.load_transform(path, coordinate) {
            Some(transform_and_layers) => transform_and_layers,
            None => {
                let (world_pos, layers) = coordinate.default_mascot_pos_and_layers();
                (Transform {
                    translation: world_pos,
                    ..default()
                }, layers)
            }
        }
    }

    fn load_transform(&self, path: &Path, coordinate: &Coordinate) -> Option<(Transform, RenderLayers)> {
        let location = self.0.get(path)?;
        let (world_pos, layers) = coordinate.initial_mascot_pos_and_layers(location.ndc, &location.monitor_name)?;
        Some((Transform {
            translation: world_pos,
            scale: location.scale,
            rotation: location.rotation,
        }, layers))
    }
}

#[derive(PartialEq, Debug, Default, Serialize, Deserialize, Clone, Reflect)]
pub struct MascotLocation {
    pub monitor_name: Option<String>,
    pub ndc: Vec3,
    pub scale: Vec3,
    pub rotation: Quat,
}

