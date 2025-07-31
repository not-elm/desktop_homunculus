use crate::ScriptResult;
use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::{FunctionCallContext, ScriptValue};
use bevy_mod_scripting::script_bindings;
use homunculus_core::prelude::OutputLog;
use homunculus_shadow_panel::ShadowPanelMaterial;

pub struct ShadowPanelScriptsPlugin;

impl Plugin for ShadowPanelScriptsPlugin {
    fn build(&self, app: &mut App) {
        register_shadow_panel_functions(app.world_mut());
    }
}

#[derive(Reflect)]
pub struct ShadowPanel;

#[script_bindings(name = "shadow_panel_functions")]
#[allow(unused)]
impl ShadowPanel {
    fn set_alpha(ctx: FunctionCallContext, alpha: f32) -> ScriptResult<ScriptValue> {
        ctx.world()?.with_global_access(|world| {
            world
                .run_system_once_with(set_alpha, alpha)
                .output_log_if_error("set_alpha");
        })?;
        Ok(ScriptValue::Unit)
    }
}

fn set_alpha(
    In(alpha_factor): In<f32>,
    mut materials: ResMut<Assets<ShadowPanelMaterial>>,
    handles: Query<&MeshMaterial3d<ShadowPanelMaterial>>,
) {
    for handle in handles.iter() {
        if let Some(material) = materials.get_mut(handle.id()) {
            material.alpha_factor = alpha_factor;
        }
    }
}
