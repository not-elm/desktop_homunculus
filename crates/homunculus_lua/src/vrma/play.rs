use crate::vrma::VrmaInstance;
use crate::{ScriptResult, ScriptVal};
use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::*;
use bevy_flurx::action::{once, wait};
use bevy_flurx::prelude::{Reactor, Then};
use bevy_mod_scripting::core::bindings::FunctionCallContext;
use bevy_mod_scripting::script_bindings;
use bevy_vrm1::prelude::{Initialized, PlayVrma, VrmAnimation};
use bevy_vrm1::vrma::Vrma;
use homunculus_core::prelude::OutputLog;

pub(super) struct VrmaPlayPlugin;

impl Plugin for VrmaPlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangeAnimation>()
            .add_observer(apply_change_animation);

        register_vrma_play_functions(app.world_mut());
    }
}

#[script_bindings(name = "vrma_play_functions")]
#[allow(unused)]
impl VrmaInstance {
    pub fn play(ctx: FunctionCallContext, instance: ScriptVal<VrmaInstance>) -> ScriptResult<()> {
        ctx.world()?.with_global_access(|world| {
            world
                .run_system_once_with(all_play_animations, instance.into_inner())
                .output_log_if_error("Vrma::play");
        })?;
        Ok(())
    }
}

fn all_play_animations(In(instance): In<VrmaInstance>, mut commands: Commands) {
    let vrm = instance.vrm;
    let vrmas = instance.vrmas;
    commands.spawn(Reactor::schedule(move |task| async move {
        for (vrma, args) in vrmas {
            let playing_other = task
                .will(Update, {
                    wait::until(has_been_loaded)
                        .with(vrma)
                        .then(once::run(play).with((vrma, args)))
                        .then(wait::either(
                            wait::until(finished).with(vrm),
                            wait::until(start_play_other).with((vrm, vrma)),
                        ))
                })
                .await
                .is_right();
            if playing_other {
                break;
            }
        }
    }));
}

fn has_been_loaded(In(vrma): In<Entity>, vrmas: Query<&Initialized, With<Vrma>>) -> bool {
    vrmas.get(vrma).is_ok()
}

fn play(In((vrma, args)): In<(Entity, PlayVrma)>, mut commands: Commands) {
    commands.entity(vrma).trigger(args);
}

fn finished(In(vrm): In<Entity>, animation: VrmAnimation) -> bool {
    animation.all_finished(vrm)
}

fn start_play_other(
    In((vrm, vrma)): In<(Entity, Entity)>,
    mut er: EventReader<ChangeAnimation>,
) -> bool {
    er.read().any(|e| e.vrm == vrm && e.vrma != vrma)
}

#[derive(Event)]
struct ChangeAnimation {
    vrm: Entity,
    vrma: Entity,
}

fn apply_change_animation(
    trigger: Trigger<PlayVrma>,
    mut ew: EventWriter<ChangeAnimation>,
    parents: Query<&ChildOf>,
) {
    if let Ok(ChildOf(vrm)) = parents.get(trigger.target()) {
        ew.write(ChangeAnimation {
            vrm: *vrm,
            vrma: trigger.target(),
        });
    }
}
