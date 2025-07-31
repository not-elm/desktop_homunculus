use crate::extract::EntityId;
use axum::extract::State;
use axum::response::Sse;
use axum::response::sse::{Event, KeepAlive};
use bevy::prelude::{Entity, In, Res, Update};
use bevy::tasks::futures_lite::Stream;
use bevy_flurx::action::once;
use futures::stream::select_all;
use homunculus_api::prelude::{ApiError, ApiReactor};
use homunculus_core::prelude::{
    OnClickEvent, OnDragEndEvent, OnDragEvent, OnDragStartEvent, OnPointerCancelEvent,
    OnPointerMoveEvent, OnPointerOutEvent, OnPointerOverEvent, OnPointerPressedEvent,
    OnPointerReleasedEvent, VrmEventReceiver, VrmStateChangeEvent,
};
use serde::Serialize;
use std::convert::Infallible;
use std::pin::Pin;
use std::time::Duration;

/// Subscribe to VRM events.
///
/// ### Path
///
/// `GET /vrm/:entity_id/events`
///
/// ### Events
///
/// - `drag-start`: Fired when a drag starts.
/// - `drag`: Fired when a drag is in progress.
/// - `drag-end`: Fired when a drag ends.
/// - `pointer-press`: Fired when a pointer is pressed.
/// - `pointer-click`: Fired when a click occurs.
/// - `pointer-move`: Fired when a pointer moves.
/// - `pointer-release`: Fired when a pointer is released.
/// - `pointer-over`: Fired when a pointer is over the VRM.
/// - `pointer-out`: Fired when a pointer is out of the VRM.
/// - `pointer-cancel`: Fired when a pointer action is canceled.
/// - `state-change`: Fired when the VRM state changes.
pub async fn events(
    State(reactor): State<ApiReactor>,
    EntityId(entity): EntityId,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>> + Send + Sync>, ApiError> {
    let stream = reactor
        .schedule(move |task| async move {
            let drag_start = task
                .will(
                    Update,
                    once::run(observe_stream::<OnDragStartEvent>).with(("drag-start", entity)),
                )
                .await;
            let drag = task
                .will(
                    Update,
                    once::run(observe_stream::<OnDragEvent>).with(("drag", entity)),
                )
                .await;
            let drag_end = task
                .will(
                    Update,
                    once::run(observe_stream::<OnDragEndEvent>).with(("drag-end", entity)),
                )
                .await;
            let pressed = task
                .will(
                    Update,
                    once::run(observe_stream::<OnPointerPressedEvent>)
                        .with(("pointer-press", entity)),
                )
                .await;
            let clicked = task
                .will(
                    Update,
                    once::run(observe_stream::<OnClickEvent>).with(("pointer-click", entity)),
                )
                .await;
            let moved = task
                .will(
                    Update,
                    once::run(observe_stream::<OnPointerMoveEvent>).with(("pointer-move", entity)),
                )
                .await;
            let released = task
                .will(
                    Update,
                    once::run(observe_stream::<OnPointerReleasedEvent>)
                        .with(("pointer-release", entity)),
                )
                .await;
            let over = task
                .will(
                    Update,
                    once::run(observe_stream::<OnPointerOverEvent>).with(("pointer-over", entity)),
                )
                .await;
            let out = task
                .will(
                    Update,
                    once::run(observe_stream::<OnPointerOutEvent>).with(("pointer-out", entity)),
                )
                .await;
            let cancel = task
                .will(
                    Update,
                    once::run(observe_stream::<OnPointerCancelEvent>)
                        .with(("pointer-cancel", entity)),
                )
                .await;
            let state_change = task
                .will(
                    Update,
                    once::run(observe_stream::<VrmStateChangeEvent>).with(("state-change", entity)),
                )
                .await;
            select_all([
                drag_start,
                drag,
                drag_end,
                pressed,
                clicked,
                moved,
                released,
                over,
                out,
                cancel,
                state_change,
            ])
        })
        .await?;

    Ok(Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(30))))
}

fn observe_stream<E>(
    In((event_name, target_vrm)): In<(&'static str, Entity)>,
    rx: Res<VrmEventReceiver<E>>,
) -> Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send + Sync + 'static>>
where
    E: Serialize + Clone + Send + Sync + 'static,
{
    let stream = futures::stream::unfold(rx.clone(), move |mut rx| async move {
        loop {
            let event = rx.recv().await.ok()?;
            if event.vrm != target_vrm {
                continue;
            }
            let data = Event::default()
                .event(event_name)
                .data(serde_json::to_string(&event).unwrap());
            return Some((Ok(data), rx));
        }
    });
    Box::pin(stream)
}
