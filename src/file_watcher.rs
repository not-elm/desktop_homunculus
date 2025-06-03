mod vrm;
mod vrma;

use crate::file_watcher::vrm::VrmFileWatcherPlugin;
use crate::file_watcher::vrma::VrmaFileWatcherPlugin;
use bevy::app::{App, Plugin};

pub struct FileWatcherPlugin;

impl Plugin for FileWatcherPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((VrmFileWatcherPlugin, VrmaFileWatcherPlugin));
    }
}
