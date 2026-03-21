//! When the application exits, it automatically saves each VRM's Transform
//! using the asset ID as the preferences key (e.g., `elmer:vrm:transform`).
//! VRM transforms are also periodically saved to protect against unexpected
//! termination (SIGKILL, Force Quit, power loss).

use crate::characters::CharactersTable;
use crate::{PrefsDatabase, PrefsKeys};
use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_vrm1::vrm::Vrm;
use homunculus_core::prelude::{AssetIdComponent, CharacterId};
use std::time::Duration;

/// Interval in seconds between periodic VRM transform saves.
pub(super) struct PrefsVrmTransformPlugin;

impl Plugin for PrefsVrmTransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, save_vrm_transforms.run_if(on_message::<AppExit>));
        app.add_systems(
            Update,
            periodic_save_vrm_transforms.run_if(on_timer(Duration::from_secs(5))),
        );
    }
}

fn save_vrm_transforms(
    db: NonSend<PrefsDatabase>,
    transforms: Query<(&AssetIdComponent, &Transform, Option<&CharacterId>), With<Vrm>>,
) {
    info!("Saving VRM transforms to preferences...");
    for (asset_id, transform, character_id) in transforms.iter() {
        save_single_transform(&db, asset_id, transform, character_id);
    }
}

fn periodic_save_vrm_transforms(
    db: NonSend<PrefsDatabase>,
    transforms: Query<(&AssetIdComponent, &Transform, Option<&CharacterId>), With<Vrm>>,
) {
    for (asset_id, transform, character_id) in transforms.iter() {
        save_single_transform(&db, asset_id, transform, character_id);
    }
}

/// Saves a single VRM transform. When an `CharacterId` is present, the transform
/// is persisted to the character table instead of the global preferences.
fn save_single_transform(
    db: &PrefsDatabase,
    asset_id: &AssetIdComponent,
    transform: &Transform,
    character_id: Option<&CharacterId>,
) {
    if let Some(id) = character_id {
        let json = serde_json::to_string(transform).unwrap_or_else(|_| "{}".to_string());
        let _ = CharactersTable::new(db).update_transform(id, &json);
    } else {
        let key = PrefsKeys::asset_transform(asset_id.0.as_ref());
        let _ = db.save_as(&key, transform);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::PostUpdate;
    use homunculus_core::prelude::AssetId;

    fn emit_app_exit_once(mut emitted: Local<bool>, mut ew: MessageWriter<AppExit>) {
        if !*emitted {
            ew.write(AppExit::Success);
            *emitted = true;
        }
    }

    #[test]
    fn saves_vrm_transform_when_exit_message_is_emitted_in_post_update() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_non_send_resource(PrefsDatabase::open_in_memory())
            .add_plugins(PrefsVrmTransformPlugin)
            .add_systems(PostUpdate, emit_app_exit_once);

        let asset_id = AssetId::new("elmer:vrm");
        let expected = Transform::from_xyz(42.0, -3.0, 1.5);
        app.world_mut()
            .spawn((AssetIdComponent(asset_id.clone()), expected, Vrm));

        // Frame 1: PostUpdate writes AppExit; Update doesn't see it yet
        app.update();
        // Frame 2: First processes AppExit → Update's on_message::<AppExit> fires
        app.update();

        let key = PrefsKeys::asset_transform(asset_id.as_ref());
        let db = app.world().non_send_resource::<PrefsDatabase>();
        let saved = db
            .load_as::<Transform>(&key)
            .expect("loading transform should not fail");
        assert_eq!(saved, Some(expected));
    }
}
