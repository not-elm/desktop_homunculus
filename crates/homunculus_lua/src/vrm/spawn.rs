use crate::lua_args::GetArgs;
use crate::vrm::on_range_timer::OnRangeTimerHandlers;
use crate::vrm::on_state_enter::OnStateEnterHandler;
use crate::vrm::on_timer::OnTimerHandlers;
use crate::vrm::pointer::{
    OnPointerDragEndHandler, OnPointerDragHandler, OnPointerDragStartHandler,
    OnPointerMovedHandler, OnPointerPressedHandler, OnPointerReleasedHandler,
};
use crate::vrm::{VrmHandlers, VrmInstance};
use crate::{ScriptResult, ScriptVal, ScriptValueResult};
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::{
    FunctionCallContext, GlobalNamespace, IntoScript, ScriptValue,
};
use bevy_mod_scripting::script_bindings;
use bevy_vrm1::prelude::{Cameras, VrmHandle};
use homunculus_core::prelude::{Loading, VrmState, app_data_dir, vrm_settings_path};
use std::collections::HashMap;
use std::path::PathBuf;

pub(super) struct VrmSpawnPlugin;

impl Plugin for VrmSpawnPlugin {
    fn build(&self, app: &mut App) {
        register_vrm_spawn(app.world_mut());
        register_settings_functions(app.world_mut());
    }
}

#[script_bindings(name = "vrm_spawn")]
#[allow(unused)]
impl VrmInstance {
    fn spawn(
        ctx: FunctionCallContext,
        vrm_path: PathBuf,
        options: Option<HashMap<String, ScriptValue>>,
    ) -> ScriptValueResult<VrmInstance> {
        let options = options.unwrap_or_default();
        let world = ctx.world()?;
        let scale = options
            .get_reflect::<Vec3>("scale", world.clone())
            .unwrap_or(Vec3::ONE);
        let rotation = options
            .get_reflect::<Quat>("rotation", world.clone())
            .unwrap_or_default();
        let position = options
            .get_reflect::<Vec3>("position", world.clone())
            .unwrap_or_default();

        world.with_global_access(|world| {
            let mut system_state = SystemState::<(Cameras, Res<AssetServer>)>::new(world);
            let (cameras, asset_server) = system_state.get(world);
            let vrm_entity = world
                .spawn((
                    VrmHandle(asset_server.load(PathBuf::from("plugins").join(vrm_path))),
                    cameras.all_layers(),
                    VrmState::default(),
                    OnTimerHandlers::default(),
                    OnRangeTimerHandlers::default(),
                    VrmHandlers::<OnPointerPressedHandler>::default(),
                    VrmHandlers::<OnPointerMovedHandler>::default(),
                    VrmHandlers::<OnPointerReleasedHandler>::default(),
                    VrmHandlers::<OnPointerDragStartHandler>::default(),
                    VrmHandlers::<OnPointerDragHandler>::default(),
                    VrmHandlers::<OnPointerDragEndHandler>::default(),
                    VrmHandlers::<OnStateEnterHandler>::default(),
                    Transform {
                        translation: position,
                        rotation,
                        scale,
                    },
                ))
                .remove::<Loading>()
                .id();
            ScriptVal::new(VrmInstance {
                entity: vrm_entity,
                observe_on: None,
            })
        })
    }
}

#[script_bindings(name = "settings_functions", remote, unregistered)]
#[allow(unused)]
impl GlobalNamespace {
    fn load_settings(
        ctx: FunctionCallContext,
        vrm_path: PathBuf,
    ) -> ScriptResult<HashMap<String, ScriptValue>> {
        let Some(transform) = vrm_settings_path(&app_data_dir(), &vrm_path)
            .and_then(|path| std::fs::read(&path).ok())
            .and_then(|buf| serde_json::from_slice::<Transform>(&buf).ok())
        else {
            return Ok(HashMap::new());
        };
        let world = ctx.world()?;
        Ok(HashMap::from([
            (
                "position".to_string(),
                ScriptVal::new(transform.translation).into_script(world.clone())?,
            ),
            (
                "rotation".to_string(),
                ScriptVal::new(transform.rotation).into_script(world.clone())?,
            ),
            (
                "scale".to_string(),
                ScriptVal::new(transform.scale).into_script(world.clone())?,
            ),
        ]))
    }
}
