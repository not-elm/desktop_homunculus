mod flow;
mod global;
// mod homunculus_screen;
mod load;
mod lua_args;
mod prefs;
mod random;
mod shadow_panel;
pub mod util;
mod vrm;
mod vrma;
mod webview;

use crate::flow::FlowScriptPlugin;
use crate::global::GlobalScriptsPlugin;
// use crate::homunculus_screen::GlobalWindowsScriptPlugin;
use crate::load::LoadScriptsPlugin;
use crate::prefs::ScriptPrefsPlugin;
use crate::random::RandomScriptPlugin;
use crate::shadow_panel::ShadowPanelScriptsPlugin;
use crate::vrm::VrmScriptsPlugin;
use crate::vrma::VrmaScriptsPlugin;
use crate::webview::HomunculusWebviewPlugin;
use bevy::prelude::*;
use bevy_mod_scripting::BMSPlugin;
use bevy_mod_scripting::core::error::InteropError;

pub mod prelude {
    pub use crate::lua_args::*;
}

pub(crate) type ScriptVal<V> = bevy_mod_scripting::core::bindings::Val<V>;
pub(crate) type ScriptValueResult<V> = std::result::Result<ScriptVal<V>, InteropError>;
pub(crate) type ScriptResult<V> = std::result::Result<V, InteropError>;
pub(crate) struct RunOnMainThread;

pub struct HomunculusScriptPlugin;

impl Plugin for HomunculusScriptPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BMSPlugin,
            ScriptPrefsPlugin,
            VrmScriptsPlugin,
            VrmaScriptsPlugin,
            FlowScriptPlugin,
            LoadScriptsPlugin,
            GlobalScriptsPlugin,
            // GlobalWindowsScriptPlugin,
            RandomScriptPlugin,
            ShadowPanelScriptsPlugin,
            HomunculusWebviewPlugin,
        ));
    }
}
