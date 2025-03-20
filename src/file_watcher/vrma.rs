use crate::mascot::Mascot;
use crate::power_state::Loading;
use crate::util::{animations_dir, create_dir_all_if_need};
use bevy::app::{App, Startup, Update};
use bevy::asset::io::file::FileWatcher;
use bevy::asset::io::AssetSourceEvent;
use bevy::asset::{Handle, LoadedFolder};
use bevy::core::Name;
use bevy::ecs::world::DeferredWorld;
use bevy::log::error;
use bevy::prelude::{
    AssetServer, Assets, BuildChildren, Children, Commands, Component, Entity, Event, EventWriter,
    Local, ParallelCommands, Plugin, PreStartup, Query, Reflect, Res, Trigger, With,
};
use bevy_vrma::vrma::VrmaHandle;
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
            .add_observer(observer_load_vrma);
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
    let (entity, handle) = folder_handle.single();
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
    while watchers.single().receiver.try_recv().is_ok() {
        commands.trigger(LoadVrma);
    }
}

fn observer_load_vrma(
    _: Trigger<LoadVrma>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    folder_assets: Res<Assets<LoadedFolder>>,
    folder_handle: Query<&VrmaFolderHandle>,
    mascots: Query<(Entity, Option<&Children>), With<Mascot>>,
    // vrma: Query<&ActionProperties, Or<(With<Vrma>, With<VrmaHandle>)>>,
) {
    for vrma_path in all_loaded_vrma_path(&folder_assets, &folder_handle) {
        mascots.iter().for_each(|(mascot_entity, children)| {
            // if already_attached(children, &vrma, &state) {
            //     return;
            // }
            commands.entity(mascot_entity).with_child((
                Name::from(format!("{}", vrma_path.display())),
                VrmaHandle(asset_server.load(vrma_path.clone())),
            ));
        });
    }
}

// fn remove_all_removed_vrma(
//     mut commands: Commands,
//     folder_assets: Res<Assets<LoadedFolder>>,
//     folder_handle: Query<&AnimationFolderHandle>,
//     vrma: Query<(Entity, &ActionProperties), Or<(With<Vrma>, With<VrmaHandle>)>>,
// ) {
//     let current_exists = all_loaded_vrma_path(&folder_assets, &folder_handle)
//         .into_iter()
//         .map(|(state, _)| state)
//         .collect::<Vec<_>>();
//     for (remove_vrma_entity, state) in search_all_removed_vrma_path(&current_exists, vrma.iter()) {
//         debug!("Remove {state:?} from {remove_vrma_entity}");
//         commands.entity(remove_vrma_entity).despawn_recursive();
//     }
// }

fn all_loaded_vrma_path(
    folder_assets: &Res<Assets<LoadedFolder>>,
    folder_handle: &Query<&VrmaFolderHandle>,
) -> Vec<PathBuf> {
    let Some(folder) = folder_handle
        .get_single()
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

// fn already_attached(
//     children: Option<&Children>,
//     vrma: &Query<&ActionProperties, >,
//     state: &ActionProperties,
// ) -> bool {
//     let Some(children) = children else {
//         return false;
//     };
//     children
//         .iter()
//         .any(|entity| {
//             vrma.get(*entity).is_ok_and(|vrma_state| vrma_state == state)
//         })
// }
