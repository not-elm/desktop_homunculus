use crate::error::OutputLog;
use crate::prelude::{
    AppWindows, GlobalViewport, OnClickEvent, OnDragEndEvent, OnDragEvent, OnDragStartEvent,
    OnPointerCancelEvent, OnPointerMoveEvent, OnPointerOutEvent, OnPointerOverEvent,
    OnPointerPressedEvent, OnPointerReleasedEvent, VrmState, global_cursor_pos,
};
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_vrm1::prelude::{Initialized, ParentSearcher};
use bevy_vrm1::vrm::{Vrm, VrmBone};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

mod vrm;

pub mod prelude {
    pub use crate::events::{
        VrmEventReceiver, VrmEventSender, VrmMetadata, VrmStateChangeEvent, vrm::*,
    };
}

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct VrmEventSender<E>(async_broadcast::Sender<VrmEvent<E>>);

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct VrmEventReceiver<E>(pub async_broadcast::Receiver<VrmEvent<E>>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VrmEvent<E> {
    pub vrm: Entity,
    #[serde(flatten)]
    pub payload: E,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VrmStateChangeEvent {
    pub state: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct VrmMetadata {
    pub name: String,
    pub entity: Entity,
}

pub struct HomunculusEventsPlugin;

impl Plugin for HomunculusEventsPlugin {
    fn build(&self, app: &mut App) {
        setup_channel::<OnDragStartEvent>(app);
        setup_channel::<OnDragEvent>(app);
        setup_channel::<OnDragEndEvent>(app);
        setup_channel::<OnPointerPressedEvent>(app);
        setup_channel::<OnClickEvent>(app);
        setup_channel::<OnPointerMoveEvent>(app);
        setup_channel::<OnPointerReleasedEvent>(app);
        setup_channel::<OnPointerOverEvent>(app);
        setup_channel::<OnPointerOutEvent>(app);
        setup_channel::<OnPointerCancelEvent>(app);
        setup_channel::<VrmStateChangeEvent>(app);
        setup_channel::<VrmMetadata>(app);

        app.add_systems(Update, (start_observe_vrm, state_change, vrm_metadata));
    }
}

fn setup_channel<E: Send + Sync + 'static>(app: &mut App) {
    let (mut sender, receiver) = async_broadcast::broadcast::<VrmEvent<E>>(256);
    sender.set_overflow(true);
    app.insert_resource(VrmEventSender(sender))
        .insert_resource(VrmEventReceiver(receiver));
}

fn start_observe_vrm(mut commands: Commands, vrms: Query<Entity, (With<Vrm>, Added<Initialized>)>) {
    for vrm in vrms.iter() {
        commands
            .entity(vrm)
            .observe(pointer::<DragStart, OnDragStartEvent>)
            .observe(pointer::<Drag, OnDragEvent>)
            .observe(pointer::<DragEnd, OnDragEndEvent>)
            .observe(pointer::<Pressed, OnPointerPressedEvent>)
            .observe(pointer::<Click, OnClickEvent>)
            .observe(pointer::<Move, OnPointerMoveEvent>)
            .observe(pointer::<Released, OnPointerReleasedEvent>)
            .observe(pointer::<Over, OnPointerOverEvent>)
            .observe(pointer::<Out, OnPointerOutEvent>)
            .observe(pointer::<Cancel, OnPointerCancelEvent>);
    }
}

fn pointer<E1, E2>(
    trigger: Trigger<Pointer<E1>>,
    tx: Res<VrmEventSender<E2>>,
    parent_searcher: ParentSearcher,
    windows: AppWindows,
) where
    E1: Clone + Reflect + Debug,
    E2: From<(GlobalViewport, Option<VrmBone>, E1)> + Send + Sync + 'static,
    E2: Clone,
{
    let Some(vrm) = parent_searcher.find_vrm(trigger.target) else {
        return;
    };
    let Some(global_viewport) = global_cursor_pos(&trigger, &windows) else {
        return;
    };
    let _ = tx.try_broadcast(VrmEvent {
        vrm,
        payload: E2::from((global_viewport, None, trigger.event.clone())),
    });
}

fn state_change(
    tx: Res<VrmEventSender<VrmStateChangeEvent>>,
    vrms: Query<(Entity, &VrmState), Changed<VrmState>>,
) {
    for (vrm, state) in vrms.iter() {
        tx.try_broadcast(VrmEvent {
            vrm,
            payload: VrmStateChangeEvent {
                state: state.to_string(),
            },
        })
        .output_log_if_error("Failed to broadcast VrmStateChangeEvent");
    }
}

fn vrm_metadata(
    tx: Res<VrmEventSender<VrmMetadata>>,
    vrms: Query<(Entity, &Name), (Added<Initialized>, With<Vrm>)>,
) {
    for (entity, name) in vrms.iter() {
        tx.broadcast_blocking(VrmEvent {
            vrm: entity,
            payload: VrmMetadata {
                name: name.to_string(),
                entity,
            },
        })
        .output_log_if_error("Failed to broadcast VrmMetadata");
    }
}
