mod on_listen;
mod send;

use crate::lua_args::*;
use crate::vrm::VrmHandlers;
use crate::webview::on_listen::{WebviewOnListenPlugin, WebviewScriptHandler};
use crate::webview::send::WebviewSendPlugin;
use crate::{ScriptVal, ScriptValueResult};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_mod_scripting::core::bindings::{FunctionCallContext, ScriptValue};
use bevy_mod_scripting::script_bindings;
use bevy_webview_wry::core::{Background, InitializationScripts, Webview, WebviewUri};
use homunculus_macros::ScriptArgs;
use std::collections::HashMap;

pub(super) struct HomunculusWebviewPlugin;

impl Plugin for HomunculusWebviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WebviewOnListenPlugin, WebviewSendPlugin));
        register_webview_functions(app.world_mut());
    }
}

#[derive(Reflect, Copy, Clone)]
#[reflect(type_path = false)]
pub struct WebviewInstance(Entity);

#[script_bindings(name = "webview_functions")]
#[allow(unused)]
impl WebviewInstance {
    fn open(
        ctx: FunctionCallContext,
        uri: String,
        options: Option<HashMap<String, ScriptValue>>,
    ) -> ScriptValueResult<WebviewInstance> {
        let options = OpenOptions::from_args(options.unwrap_or_default(), ctx.world()?.clone());
        ctx.world()?.with_global_access(|world| {
            let mut window = Window::default();
            let mut background = Background::default();
            if let Some(resolution) = options.resolution {
                window.resolution = WindowResolution::new(resolution.x, resolution.y);
            }
            if let Some(pos) = options.position {
                window.position = WindowPosition::At(pos);
            }
            if options.transparent.is_some_and(|transparent| transparent) {
                background = Background::Transparent;
                window.transparent = true;
                window.composite_alpha_mode = bevy::window::CompositeAlphaMode::PostMultiplied;
            }
            window.has_shadow = options.shadow.unwrap_or(true);
            window.titlebar_shown = options.show_toolbar.unwrap_or(true);
            let entity = world
                .spawn((
                    Webview::Uri(WebviewUri::relative_local(uri)),
                    InitializationScripts::new([include_str!("./webview/webview.js")]),
                    VrmHandlers::<WebviewScriptHandler>::default(),
                    background,
                    window,
                ))
                .id();
            Ok(ScriptVal::new(WebviewInstance(entity)))
        })?
    }
}

#[derive(ScriptArgs)]
struct OpenOptions {
    transparent: Option<bool>,
    show_toolbar: Option<bool>,
    shadow: Option<bool>,
    position: Option<IVec2>,
    resolution: Option<Vec2>,
}

impl TypePath for WebviewInstance {
    fn type_path() -> &'static str {
        "webview"
    }

    fn short_type_path() -> &'static str {
        "webview"
    }

    fn type_ident() -> Option<&'static str> {
        Some("Webview")
    }
}
