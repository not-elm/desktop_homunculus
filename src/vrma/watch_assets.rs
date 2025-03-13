use crate::util::{animations_dir, create_dir_all_if_need};
use crate::vrma::load::RequestLoadVrma;
use bevy::app::{App, Startup, Update};
use bevy::asset::io::file::FileWatcher;
use bevy::asset::io::AssetSourceEvent;
use bevy::log::error;
use bevy::prelude::{Commands, Component, EventWriter, Plugin, Query};
use crossbeam::channel::Receiver;
use std::time::Duration;

pub struct VrmaWatchAssetsPlugin;

impl Plugin for VrmaWatchAssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, start_watching)
            .add_systems(Update, receive_events);
    }
}

#[derive(Component)]
struct VrmaFilesWatcher {
    _watcher: FileWatcher,
    receiver: Receiver<AssetSourceEvent>,
}

fn start_watching(
    mut commands: Commands,
) {
    let (sender, receiver) = crossbeam::channel::unbounded();
    let vrma_folder = animations_dir();

    create_dir_all_if_need(&vrma_folder);

    match FileWatcher::new(
        vrma_folder,
        sender,
        Duration::from_secs(2),
    ) {
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
    mut request_load: EventWriter<RequestLoadVrma>,
    watchers: Query<&VrmaFilesWatcher>,
) {
    while watchers.single().receiver.try_recv().is_ok() {
        request_load.send(RequestLoadVrma);
    }
}

