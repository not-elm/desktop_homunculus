use crate::ScriptVal;
use crate::util::{WorldEvents, with_static_guard_with_guard};
use crate::vrm::pointer::{
    OnPointerDrag, OnPointerDragEnd, OnPointerDragEndHandler, OnPointerDragHandler,
    OnPointerDragStart, OnPointerDragStartHandler, OnPointerMoved, OnPointerMovedHandler,
    OnPointerPressed, OnPointerPressedHandler, OnPointerReleased, OnPointerReleasedHandler,
    PointerParams,
};
use crate::vrm::{VrmHandlerBase, VrmHandlers};
use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::{DynamicScriptFunction, IntoScript};
use bevy_mod_scripting::lua::bindings::script_value::LUA_CALLER_CONTEXT;
use homunculus_core::prelude::{OutputLog, VrmState};
use std::ops::Deref;

pub struct VrmPointerHandlerPlugin;

impl Plugin for VrmPointerHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                call_pointer_handlers_system::<OnPointerPressed, OnPointerPressedHandler>
                    .run_if(on_event::<OnPointerPressed>),
                call_pointer_handlers_system::<OnPointerMoved, OnPointerMovedHandler>
                    .run_if(on_event::<OnPointerMoved>),
                call_pointer_handlers_system::<OnPointerReleased, OnPointerReleasedHandler>
                    .run_if(on_event::<OnPointerReleased>),
                call_pointer_handlers_system::<OnPointerDragStart, OnPointerDragStartHandler>
                    .run_if(on_event::<OnPointerDragStart>),
                call_pointer_handlers_system::<OnPointerDrag, OnPointerDragHandler>
                    .run_if(on_event::<OnPointerDrag>),
                call_pointer_handlers_system::<OnPointerDragEnd, OnPointerDragEndHandler>
                    .run_if(on_event::<OnPointerDragEnd>),
            ),
        );
    }
}

fn call_pointer_handlers_system<E, H>(world: &mut World)
where
    E: Event + Deref<Target = PointerParams> + Clone,
    H: Deref<Target = VrmHandlerBase> + Send + Sync + 'static,
{
    let handlers = collect_handlers::<E, H>(world);
    with_static_guard_with_guard(world, |guard| {
        for (f, params) in handlers {
            let Ok(args) = ScriptVal::new(params).into_script(guard.clone()) else {
                continue;
            };
            f.call(vec![args], LUA_CALLER_CONTEXT)
                .output_log_if_error("PointerHandler");
        }
    });
}

fn collect_handlers<E, H>(world: &mut World) -> Vec<(DynamicScriptFunction, PointerParams)>
where
    E: Event + Deref<Target = PointerParams> + Clone,
    H: Deref<Target = VrmHandlerBase> + Send + Sync + 'static,
{
    let events = world.read_all_events::<E>();
    world
        .query::<(Entity, &VrmHandlers<H>, &VrmState)>()
        .iter(world)
        .flat_map(|(entity, handlers, state)| {
            let event = events.iter().find(|e| e.target == entity)?;
            Some((handlers, event.deref(), state))
        })
        .flat_map(|(handlers, params, state)| {
            handlers.iter().flat_map(move |h| {
                let h = h.deref();
                let is_target = h.observer_on.as_ref().is_none_or(|s| s == state);
                is_target.then_some((h.f.clone(), params.clone()))
            })
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::vrm::pointer::handler::collect_handlers;
    use crate::vrm::pointer::{OnPointerDragStart, OnPointerDragStartHandler, PointerParams};
    use crate::vrm::{VrmHandlerBase, VrmHandlers};
    use bevy::prelude::*;
    use homunculus_core::prelude::VrmState;

    #[test]
    fn test_collect_handlers() {
        let mut app = App::new();
        app.add_event::<OnPointerDragStart>();

        let vrm = app
            .world_mut()
            .spawn((
                Name::default(),
                VrmState::default(),
                VrmHandlers(vec![OnPointerDragStartHandler(VrmHandlerBase::default())]),
            ))
            .id();
        app.world_mut()
            .send_event(OnPointerDragStart(PointerParams {
                target: vrm,
                ..default()
            }));
        app.update();

        let handlers =
            collect_handlers::<OnPointerDragStart, OnPointerDragStartHandler>(app.world_mut());
        assert_eq!(handlers.len(), 1);
    }

    #[test]
    fn test_collect_handlers_with_observe_state() {
        let mut app = App::new();
        app.add_event::<OnPointerDragStart>();

        let vrm = app
            .world_mut()
            .spawn((
                Name::default(),
                VrmState::default(),
                VrmHandlers(vec![OnPointerDragStartHandler(VrmHandlerBase {
                    observer_on: Some(VrmState::default()),
                    ..default()
                })]),
            ))
            .id();
        app.world_mut()
            .send_event(OnPointerDragStart(PointerParams {
                target: vrm,
                ..default()
            }));
        app.update();

        let handlers =
            collect_handlers::<OnPointerDragStart, OnPointerDragStartHandler>(app.world_mut());
        assert_eq!(handlers.len(), 1);
    }

    #[test]
    fn test_collect_handlers_with_differ_observe_state() {
        let mut app = App::new();
        app.add_event::<OnPointerDragStart>();

        let vrm = app
            .world_mut()
            .spawn((
                Name::default(),
                VrmState::from("drag"),
                VrmHandlers(vec![OnPointerDragStartHandler(VrmHandlerBase {
                    observer_on: Some(VrmState::default()),
                    ..default()
                })]),
            ))
            .id();
        app.world_mut()
            .send_event(OnPointerDragStart(PointerParams {
                target: vrm,
                ..default()
            }));
        app.update();

        let handlers =
            collect_handlers::<OnPointerDragStart, OnPointerDragStartHandler>(app.world_mut());
        assert_eq!(handlers.len(), 0);
    }
}
