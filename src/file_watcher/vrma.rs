use crate::mascot::Mascot;
use crate::power_state::Loading;
use crate::util::{animations_dir, create_dir_all_if_need};
use bevy::app::{App, Startup, Update};
use bevy::asset::io::file::FileWatcher;
use bevy::asset::io::AssetSourceEvent;
use bevy::asset::{Handle, LoadedFolder};
use bevy::log::error;
use bevy::prelude::*;
use bevy_vrm1::vrma::{VrmaHandle, VrmaPath};
use crossbeam::channel::Receiver;
use std::path::PathBuf;
use std::time::Duration;

pub struct VrmaFileWatcherPlugin;

impl Plugin for VrmaFileWatcherPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_event::<LoadVrma>()
            .register_type::<VrmaFolderHandle>()
            .add_systems(PreStartup, start_load_folder)
            .add_systems(Startup, start_watching)
            .add_systems(Update, (receive_events, wait_load))
            .add_observer(observe_added_vrma)
            .add_observer(observe_removed_vrma);
    }
}

#[derive(Event)]
pub struct LoadVrma;

#[derive(Component)]
struct VrmaFilesWatcher {
    _watcher: FileWatcher,
    receiver: Receiver<AssetSourceEvent>,
}

#[derive(Component, Reflect)]
struct VrmaFolderHandle(Handle<LoadedFolder>);

fn start_load_folder(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Loading, VrmaFolderHandle(asset_server.load_folder("vrma"))));
}

fn wait_load(
    mut commands: Commands,
    mut loaded: Local<bool>,
    folder_assets: Res<Assets<LoadedFolder>>,
    folder_handle: Query<(Entity, &VrmaFolderHandle)>,
) {
    if *loaded {
        return;
    }
    let Ok((entity, handle)) = folder_handle.single() else {
        return;
    };
    if folder_assets.get(handle.0.id()).is_some() {
        *loaded = true;
        commands.entity(entity).remove::<Loading>();
        commands.trigger(LoadVrma);
    }
}

fn start_watching(mut commands: Commands) {
    let (sender, receiver) = crossbeam::channel::unbounded();
    let vrma_folder = animations_dir();

    create_dir_all_if_need(&vrma_folder);

    match FileWatcher::new(vrma_folder, sender, Duration::from_secs(2)) {
        Ok(watcher) => {
            commands.spawn(VrmaFilesWatcher {
                _watcher: watcher,
                receiver,
            });
        }
        Err(e) => {
            error!("[FileWatcher] Failed to start watching files:\n{e}");
        }
    }
}

fn receive_events(
    mut commands: Commands,
    watchers: Query<&VrmaFilesWatcher>,
) {
    let Ok(watcher) = watchers.single() else {
        return;
    };
    while watcher.receiver.try_recv().is_ok() {
        commands.trigger(LoadVrma);
    }
}

fn observe_added_vrma(
    _: Trigger<LoadVrma>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    folder_assets: Res<Assets<LoadedFolder>>,
    folder_handle: Query<&VrmaFolderHandle>,
    mascots: Query<(Entity, Option<&Children>), With<Mascot>>,
    vrma: Query<&VrmaPath>,
) {
    for vrma_path in all_loaded_vrma_path(&folder_assets, &folder_handle) {
        mascots.iter().for_each(|(mascot_entity, children)| {
            if already_attached(children, &vrma, &vrma_path) {
                return;
            }
            commands.entity(mascot_entity).with_child((
                Name::from(format!("{}", vrma_path.display())),
                VrmaHandle(asset_server.load(vrma_path.clone())),
            ));
        });
    }
}

fn observe_removed_vrma(
    _: Trigger<LoadVrma>,
    mut commands: Commands,
    folder_assets: Res<Assets<LoadedFolder>>,
    folder_handle: Query<&VrmaFolderHandle>,
    vrma: Query<(Entity, &VrmaPath)>,
) {
    let all_loaded_path = all_loaded_vrma_path(&folder_assets, &folder_handle);
    for (remove_vrma_entity, path) in obtain_all_remove_vrma_path(&all_loaded_path, &vrma) {
        debug!("Remove {path:?} from {remove_vrma_entity}");
        commands.entity(remove_vrma_entity).despawn();
    }
}

fn all_loaded_vrma_path(
    folder_assets: &Res<Assets<LoadedFolder>>,
    folder_handle: &Query<&VrmaFolderHandle>,
) -> Vec<PathBuf> {
    let Some(folder) = folder_handle
        .single()
        .ok()
        .and_then(|handle| folder_assets.get(handle.0.id()))
    else {
        return Vec::with_capacity(0);
    };
    folder
        .handles
        .iter()
        .filter_map(|handle| {
            let path = handle.path()?.path();
            Some(path.to_path_buf())
        })
        .collect()
}

fn already_attached(
    children: Option<&Children>,
    vrma: &Query<&VrmaPath>,
    path: &PathBuf,
) -> bool {
    let Some(children) = children else {
        return false;
    };
    children
        .iter()
        .any(|entity| vrma.get(entity).is_ok_and(|vrma_path| &vrma_path.0 == path))
}

fn obtain_all_remove_vrma_path(
    all_loaded_path: &[PathBuf],
    vrma: &Query<(Entity, &VrmaPath)>,
) -> Vec<(Entity, PathBuf)> {
    vrma.iter()
        .filter_map(|(entity, vrma_path)| {
            if all_loaded_path.contains(&vrma_path.0) {
                None
            } else {
                Some((entity, vrma_path.0.clone()))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::file_watcher::vrma::{already_attached, obtain_all_remove_vrma_path};
    use crate::tests::{test_app, TestResult};
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{Children, Commands, Entity, Query, With};
    use bevy_vrm1::vrm::Vrm;
    use bevy_vrm1::vrma::VrmaPath;
    use std::path::PathBuf;

    #[test]
    fn return_false_if_has_not_children() -> TestResult {
        let mut app = test_app();
        let attached = app.world_mut().run_system_once(|vrma: Query<&VrmaPath>| {
            already_attached(None, &vrma, &PathBuf::from("/root"))
        })?;
        assert!(!attached);
        Ok(())
    }

    #[test]
    fn return_true_if_has_been_attached() -> TestResult {
        let mut app = test_app();
        app.world_mut().run_system_once(|mut commands: Commands| {
            commands
                .spawn(Vrm)
                .with_child(VrmaPath(PathBuf::from("/root")));
        })?;
        let attached = app.world_mut().run_system_once(
            |vrm: Query<&Children, With<Vrm>>, vrma: Query<&VrmaPath>| {
                already_attached(Some(vrm.single().unwrap()), &vrma, &PathBuf::from("/root"))
            },
        )?;
        assert!(attached);
        Ok(())
    }

    #[test]
    fn return_false_if_has_not_been_attached() -> TestResult {
        let mut app = test_app();
        app.world_mut().run_system_once(|mut commands: Commands| {
            commands
                .spawn(Vrm)
                .with_child(VrmaPath(PathBuf::from("/user")));
        })?;
        let attached = app.world_mut().run_system_once(
            |vrm: Query<&Children, With<Vrm>>, vrma: Query<&VrmaPath>| {
                already_attached(Some(vrm.single().unwrap()), &vrma, &PathBuf::from("/root"))
            },
        )?;
        assert!(!attached);
        Ok(())
    }

    #[test]
    fn empty_if_not_removed() -> TestResult {
        let mut app = test_app();
        app.world_mut().run_system_once(|mut commands: Commands| {
            commands.spawn(VrmaPath(PathBuf::from("/")));
        })?;
        let all_removed: Vec<_> =
            app.world_mut()
                .run_system_once(|vrma: Query<(Entity, &VrmaPath)>| {
                    obtain_all_remove_vrma_path(&[PathBuf::from("/")], &vrma)
                })?;
        assert!(all_removed.is_empty());
        Ok(())
    }

    #[test]
    fn all_removed_len_is_1() -> TestResult {
        let mut app = test_app();
        app.world_mut().run_system_once(|mut commands: Commands| {
            commands.spawn(VrmaPath(PathBuf::from("/")));
        })?;
        let all_removed: Vec<_> =
            app.world_mut()
                .run_system_once(|vrma: Query<(Entity, &VrmaPath)>| {
                    obtain_all_remove_vrma_path(&[], &vrma)
                })?;
        assert_eq!(all_removed.len(), 1);
        Ok(())
    }
}
