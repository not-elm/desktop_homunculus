use crate::application_windows::hit_test::UpdatedHitTest;
use crate::file_watcher::vrma::LoadVrma;
use crate::mascot::Mascot;
use crate::power_state::Loading;
use crate::settings::preferences::action::ActionName;
use crate::settings::preferences::MascotLocationPreferences;
use crate::system_param::coordinate::Coordinate;
use crate::util::{create_dir_all_if_need, models_dir, remove_mystery_file_if_exists};
use bevy::app::{App, PreStartup, Update};
use bevy::asset::io::file::FileWatcher;
use bevy::asset::io::AssetSourceEvent;
use bevy::asset::{Assets, Handle, LoadedFolder};
use bevy::log::error;
use bevy::prelude::*;
use bevy::render::camera::NormalizedRenderTarget;
use bevy_vrma::system_param::cameras::Cameras;
use bevy_vrma::vrm::loader::VrmHandle;
use bevy_vrma::vrm::VrmPath;
use crossbeam::channel::Receiver;
use std::path::PathBuf;
use std::time::Duration;

pub struct VrmFileWatcherPlugin;

impl Plugin for VrmFileWatcherPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.register_type::<VrmFolderHandle>()
            .add_event::<RequestLoadVrm>()
            .add_systems(
                PreStartup,
                (start_load_models_folder, start_watching).chain(),
            )
            .add_systems(
                Update,
                (
                    prepare_initial_loading,
                    load_models.run_if(on_event::<RequestLoadVrm>),
                    receive_events,
                ),
            );
    }
}

#[derive(Component)]
struct ModelFilesWatcher {
    _watcher: FileWatcher,
    receiver: Receiver<AssetSourceEvent>,
}

#[derive(Component, Reflect)]
struct VrmFolderHandle(Handle<LoadedFolder>);

#[derive(Event)]
struct RequestLoadVrm;

fn start_load_models_folder(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    remove_mystery_file_if_exists(&models_dir());
    commands.spawn((Loading, VrmFolderHandle(asset_server.load_folder("models"))));
}

fn prepare_initial_loading(
    mut commands: Commands,
    mut ew: EventWriter<RequestLoadVrm>,
    mut loaded: Local<bool>,
    folders: Res<Assets<LoadedFolder>>,
    handle: Query<(Entity, &VrmFolderHandle)>,
) {
    if *loaded {
        return;
    }
    let Ok((entity, handle)) = handle.single() else {
        return;
    };
    if folders.contains(handle.0.id()) {
        *loaded = true;
        commands.entity(entity).remove::<Loading>();
        ew.write(RequestLoadVrm);
    }
}

fn load_models(
    mut commands: Commands,
    coordinate: Coordinate,
    locations: Res<MascotLocationPreferences>,
    folders: Res<Assets<LoadedFolder>>,
    handle: Query<&VrmFolderHandle>,
    asset_server: Res<AssetServer>,
    mascots: Query<&VrmPath>,
    cameras: Cameras,
) {
    let Some(folder) = handle.single().ok().and_then(|h| folders.get(h.0.id())) else {
        return;
    };
    let exists_mascots = mascots.iter().map(|p| p.0.as_path()).collect::<Vec<_>>();
    for asset_path in folder
        .handles
        .iter()
        .flat_map(|handle| handle.path())
        .filter(|path| !exists_mascots.contains(&path.path()))
    {
        commands
            .spawn((
                Mascot,
                ActionName::idle(),
                locations.load_transform(asset_path.path(), &coordinate),
                VrmHandle(asset_server.load(asset_path)),
                cameras.all_layers(),
            ))
            .observe(enable_hit_test)
            .observe(disable_hit_test);
        commands.trigger(LoadVrma);
    }
}

fn enable_hit_test(
    trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
) {
    if let NormalizedRenderTarget::Window(window) = trigger.pointer_location.target {
        commands.trigger(UpdatedHitTest {
            window: window.entity(),
            hit_test: true,
        });
    }
}

fn disable_hit_test(
    trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
) {
    if let NormalizedRenderTarget::Window(window) = trigger.pointer_location.target {
        commands.trigger(UpdatedHitTest {
            window: window.entity(),
            hit_test: false,
        });
    }
}

fn start_watching(mut commands: Commands) {
    let (sender, receiver) = crossbeam::channel::unbounded();
    let models_folder = models_dir();
    create_dir_all_if_need(&models_folder);

    match FileWatcher::new(models_folder, sender, Duration::from_secs(1)) {
        Ok(watcher) => {
            commands.spawn(ModelFilesWatcher {
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
    coordinate: Coordinate,
    mascot_preferences: Res<MascotLocationPreferences>,
    asset_server: Res<AssetServer>,
    watchers: Query<&ModelFilesWatcher>,
) {
    let Ok(watcher) = watchers.single() else {
        return;
    };
    while let Ok(event) = watcher.receiver.try_recv() {
        if let AssetSourceEvent::AddedAsset(path) = event {
            let relative_path = PathBuf::from("models").join(&path);
            commands.spawn((
                mascot_preferences.load_transform(&relative_path, &coordinate),
                VrmHandle(asset_server.load(models_dir().join(path))),
            ));
        }
    }
}
