mod close;
mod get;
mod is_closed;
mod linked_character;
mod list;
mod navigate;
pub(super) mod open;
mod reload;
mod update;

use crate::api;
use bevy::prelude::*;

api!(WebviewApi);

pub struct WebviewApiPlugin;

impl Plugin for WebviewApiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(open::WebviewOpenPlugin);
    }
}
