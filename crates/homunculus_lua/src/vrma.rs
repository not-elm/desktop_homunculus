mod chain;
mod play;
mod repeat;
mod spawn;

use crate::vrma::chain::VrmaChainPlugin;
use crate::vrma::play::VrmaPlayPlugin;
use crate::vrma::repeat::RepeatPlugin;
use crate::vrma::spawn::VrmaSpawnPlugin;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_vrm1::prelude::PlayVrma;
use bevy_vrm1::vrma::VrmaPath;
use std::path::Path;

#[derive(Reflect)]
#[reflect(type_path = false)]
pub(crate) struct VrmaInstance {
    pub vrm: Entity,
    pub vrmas: Vec<(Entity, PlayVrma)>,
}

impl TypePath for VrmaInstance {
    fn type_path() -> &'static str {
        "vrma"
    }

    fn short_type_path() -> &'static str {
        "vrma"
    }

    fn type_ident() -> Option<&'static str> {
        Some("vrma")
    }
}

pub(super) struct VrmaScriptsPlugin;

impl Plugin for VrmaScriptsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RepeatPlugin,
            VrmaSpawnPlugin,
            VrmaChainPlugin,
            VrmaPlayPlugin,
        ));
    }
}

// fn find_vrm<'w>(
//     vrm_name: &str,
//     vrms: &'w Query<(Entity, &Name, &Children), With<Vrm>>,
// ) -> Option<(Entity, &'w Children)> {
//     vrms.iter().find_map(|(entity, name, children)| {
//         (name.as_str() == vrm_name).then_some((entity, children))
//     })
// }

fn find_vrma(
    vrm_children: &Children,
    vrmas: &Query<(Entity, &VrmaPath)>,
    path: &String,
) -> Option<Entity> {
    let path = Path::new(path);
    vrm_children
        .iter()
        .flat_map(|child| vrmas.get(child).ok())
        .find_map(|(vrma, vrma_path)| (vrma_path.0 == path).then_some(vrma))
}
