pub mod loader;
pub mod spawn;
pub mod extensions;
mod load;

use crate::new_type;
use crate::vrm::load::VrmLoadPlugin;
use crate::vrm::loader::{Vrm, VrmLoaderPlugin};
use crate::vrm::spawn::VrmSpawnPlugin;
use bevy::app::{App, Plugin};
use bevy::asset::AssetApp;
use bevy::math::Quat;
use bevy::prelude::{Component, Entity, Reflect, Transform};
use std::path::PathBuf;

new_type!(VrmBone, String);
new_type!(VrmExpression, String);

#[derive(Debug, Reflect, Clone, Component)]
pub struct VrmPath(pub PathBuf);

#[derive(Debug, Reflect, Copy, Clone, Component)]
pub struct BoneRestTransform(pub Transform);

#[derive(Debug, Reflect, Copy, Clone, Component)]
pub struct BonePgRestQuaternion(pub Quat);

#[derive(Debug, Reflect, Copy, Clone, Component)]
pub struct VrmHipsBoneTo(pub Entity);

pub struct VrmPlugin;

impl Plugin for VrmPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<Vrm>()
            .register_type::<VrmPath>()
            .register_type::<BoneRestTransform>()
            .register_type::<VrmHipsBoneTo>()
            .register_type::<VrmBone>()
            .add_plugins((
                VrmLoadPlugin,
                VrmLoaderPlugin,
                VrmSpawnPlugin,
            ));
    }
}


