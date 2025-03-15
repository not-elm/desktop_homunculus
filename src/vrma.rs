mod loader;
pub mod load;
pub mod retarget;
pub mod animation;
mod extensions;
mod watch_assets;

use crate::vrma::animation::VrmaAnimationPlayersPlugin;
use crate::vrma::load::VrmaLoadPlugin;
use crate::vrma::loader::{VrmaAsset, VrmaLoaderPlugin};
use crate::vrma::retarget::VrmaRetargetPlugin;
use crate::vrma::watch_assets::VrmaWatchAssetsPlugin;
use bevy::app::App;
use bevy::asset::Handle;
use bevy::prelude::{Component, Entity, Plugin, Reflect};
use std::time::Duration;

pub struct VrmaPlugin;

impl Plugin for VrmaPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Vrma>()
            .register_type::<VrmaEntity>()
            .register_type::<VrmaHandle>()
            .register_type::<VrmaDuration>()
            .register_type::<RetargetTo>()
            .register_type::<RetargetSource>()
            .add_plugins((
                VrmaLoaderPlugin,
                VrmaLoadPlugin,
                VrmaRetargetPlugin,
                VrmaAnimationPlayersPlugin,
                VrmaWatchAssetsPlugin,
            ));
    }
}

#[derive(Debug, Component, Reflect, Copy, Clone)]
pub struct Vrma;

#[derive(Debug, Reflect, Copy, Clone)]
pub struct VrmaEntity(pub Entity);

#[derive(Debug, Component, Reflect)]
pub struct VrmaHandle(pub Handle<VrmaAsset>);

#[derive(Debug, Component, Reflect)]
pub struct VrmaDuration(pub Duration);

#[derive(Debug, Component, Reflect)]
struct RetargetTo(pub Entity);

#[derive(Component, Reflect)]
struct RetargetSource;

