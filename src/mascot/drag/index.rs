use crate::mascot::{Mascot, MascotEntity};
use crate::settings::state::MascotState;
use crate::system_param::mascot_controller::MascotTracker;
use crate::system_param::mouse_position::MousePosition;
use crate::vrma::VrmaDuration;
use bevy::app::App;
use bevy::prelude::{Changed, Children, Entity, In, Local, ParallelCommands, Plugin, Query, Res, Time, Timer, TimerMode, Update, With, Without};
use bevy::render::view::RenderLayers;
use bevy_flurx::action::wait;
use bevy_flurx::prelude::Reactor;
use std::time::Duration;

/// When the user starts dragging the mascot, the [`MascotState`] transitions to `drag:index`.
/// Its animation is intended to raise the mascot to the cursor position, so the mascot is placed below and raised in parallel with the animation.
pub struct MascotDragIndexPlugin;

impl Plugin for MascotDragIndexPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, start_rise);
    }
}

fn start_rise(
    commands: ParallelCommands,
    mascots: Query<(Entity, &MascotState, &Children), (Changed<MascotState>, With<Mascot>)>,
    vrma: Query<(&MascotState, &VrmaDuration), Without<Mascot>>,
) {
    mascots.par_iter().for_each(|(entity, state, children)| {
        if !state.is_grab_index() {
            return;
        }
        let mascot = MascotEntity(entity);
        let Some(duration) = children.iter().find_map(|entity| {
            let (vrma_state, duration) = vrma.get(*entity).ok()?;
            vrma_state.is_grab_index().then_some(duration.0.as_secs_f32())
        }) else {
            return;
        };
        commands.command_scope(move |mut commands| {
            commands.spawn(Reactor::schedule(move |task| async move {
                task.will(Update, wait::until(rise).with((mascot, duration))).await;
            }));
        });
    });
}

fn rise(
    In((mascot, duration)): In<(MascotEntity, f32)>,
    mut controller: MascotTracker,
    mut timer: Local<Option<Timer>>,
    layers: Query<&RenderLayers>,
    time: Res<Time>,
    mouse_position: MousePosition,
) -> bool {
    let timer = timer
        .get_or_insert_with(|| Timer::new(Duration::from_secs_f32(duration), TimerMode::Once))
        .tick(time.delta());
    let t = duration - timer.elapsed_secs() / duration;
    let layers = layers.get(mascot.0).unwrap_or_default();
    if let Some(viewport_pos) = mouse_position.local(layers) {
        controller.rise(mascot, viewport_pos, t);
    }
    timer.just_finished()
}