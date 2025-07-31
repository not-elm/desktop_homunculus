//! When the application exits, it automatically saves the VRM's Transform to `vrm::<AVATAR_NAME>::transform`.
//! This allows the VRM to maintain its position and rotation across sessions.

use crate::{PrefsDatabase, PrefsKeys};
use bevy::prelude::*;
use bevy_vrm1::vrm::Vrm;

pub(super) struct PrefsVrmTransformPlugin;

impl Plugin for PrefsVrmTransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, save_vrm_transforms.run_if(on_event::<AppExit>));
    }
}

fn save_vrm_transforms(
    db: NonSend<PrefsDatabase>,
    transforms: Query<(&Name, &Transform), With<Vrm>>,
) {
    for (name, transform) in transforms.iter() {
        let key = PrefsKeys::vrm_transform(name.as_str());
        let _ = db.save(&key, transform);
    }
}
