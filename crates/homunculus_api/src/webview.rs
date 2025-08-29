mod close;
mod is_closed;
pub(super) mod open;

use crate::api;
use crate::webview::close::ClosingWebviewSounds;
use bevy::prelude::*;

api!(WebviewApi);

pub struct WebviewApiPlugin;

impl Plugin for WebviewApiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClosingWebviewSounds>()
            .add_plugins(open::WebviewOpenPlugin);
    }
}
