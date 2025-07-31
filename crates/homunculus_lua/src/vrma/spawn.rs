use crate::lua_args::GetArgs;
use crate::vrm::VrmInstance;
use crate::vrma::{VrmaInstance, find_vrma};
use crate::{ScriptResult, ScriptVal, ScriptValueResult};
use bevy::animation::RepeatAnimation;
use bevy::asset::AssetServer;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::{FunctionCallContext, ScriptValue};
use bevy_mod_scripting::core::error::InteropError;
use bevy_mod_scripting::script_bindings;
use bevy_vrm1::prelude::{PlayVrma, Vrm, VrmaHandle, VrmaPath};
use homunculus_macros::ScriptArgs;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

pub(super) struct VrmaSpawnPlugin;

impl Plugin for VrmaSpawnPlugin {
    fn build(&self, app: &mut App) {
        register_vrma_play_functions(app.world_mut());
    }
}

#[script_bindings(name = "vrma_play_functions")]
#[allow(unused)]
impl VrmaInstance {
    pub fn spawn(
        ctx: FunctionCallContext,
        vrm: ScriptVal<VrmInstance>,
        path: String,
        options: Option<HashMap<String, ScriptValue>>,
    ) -> ScriptValueResult<VrmaInstance> {
        let vrm_entity = vrm.into_inner().entity;
        let (vrma, play) = spawn_vrma(ctx, options, vrm_entity, path)?;
        Ok(ScriptVal::new(VrmaInstance {
            vrm: vrm_entity,
            vrmas: vec![(vrma, play)],
        }))
    }
}

pub(crate) fn spawn_vrma(
    ctx: FunctionCallContext,
    options: Option<HashMap<String, ScriptValue>>,
    vrm_entity: Entity,
    path: String,
) -> ScriptResult<(Entity, PlayVrma)> {
    let world = ctx.world()?;
    let options = PlayOptions::from_args(options.unwrap_or_default(), world.clone());
    let play = options.to_play_vrma();
    let result = world.with_global_access(|world| {
        let mut system_state = SystemState::<(
            Res<AssetServer>,
            Query<&Children, With<Vrm>>,
            Query<(Entity, &VrmaPath)>,
        )>::new(world);
        let (asset_server, vrms, vrmas) = system_state.get(world);
        let Ok(vrm_children) = vrms.get(vrm_entity) else {
            return Err(InteropError::missing_entity(vrm_entity));
        };
        let vrma = if let Some(vrma) = find_vrma(vrm_children, &vrmas, &path) {
            vrma
        } else {
            let vrma_handle = VrmaHandle(asset_server.load(PathBuf::from("plugins").join(path)));
            let vrma = world.spawn(vrma_handle).id();
            world.commands().entity(vrm_entity).add_child(vrma);
            vrma
        };
        Ok((vrma, play))
    })??;
    Ok(result)
}

#[derive(ScriptArgs)]
struct PlayOptions {
    repeating: Option<RepeatAnimation>,
    transition: Option<Duration>,
}

impl PlayOptions {
    fn to_play_vrma(&self) -> PlayVrma {
        let mut p = PlayVrma::default();
        if let Some(repeat_animation) = self.repeating {
            p.repeat = repeat_animation;
        }
        if let Some(transition_duration) = self.transition {
            p.transition_duration = transition_duration;
        }
        p
    }
}
