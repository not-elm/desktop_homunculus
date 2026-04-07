use axum::extract::State;
use axum::response::Sse;
use axum::response::sse::{Event, KeepAlive};
use bevy::prelude::{Entity, In, Res, Update};
use bevy::tasks::futures_lite::Stream;
use bevy_flurx::action::once;
use futures::stream::select_all;
use homunculus_api::prelude::{ApiError, ApiReactor};
use homunculus_core::prelude::{
    ExpressionChangeEvent, OnClickEvent, OnDragEndEvent, OnDragEvent, OnDragStartEvent,
    OnPointerCancelEvent, OnPointerMoveEvent, OnPointerOutEvent, OnPointerOverEvent,
    OnPointerPressedEvent, OnPointerReleasedEvent, PersonaStateChangeEvent, VrmEventReceiver,
    VrmaFinishEvent, VrmaPlayEvent,
};
use serde::Serialize;
use std::convert::Infallible;
use std::pin::Pin;
use std::time::Duration;

use super::PersonaPath;

/// Subscribe to persona events via SSE.
///
/// Events include: drag-start, drag, drag-end, pointer-press, pointer-click,
/// pointer-move, pointer-release, pointer-over, pointer-out, pointer-cancel,
/// state-change, expression-change, vrma-play, vrma-finish.
#[utoipa::path(
    get,
    path = "/events",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 200, description = "SSE stream of persona events", content_type = "text/event-stream"),
    ),
)]
pub async fn events(
    State(reactor): State<ApiReactor>,
    path: PersonaPath,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>> + Send + Sync>, ApiError> {
    let entity = path.entity;

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
                    once::run(observe_stream::<PersonaStateChangeEvent>)
                        .with(("state-change", entity)),
                )
                .await;
            let expression_change = task
                .will(
                    Update,
                    once::run(observe_stream::<ExpressionChangeEvent>)
                        .with(("expression-change", entity)),
                )
                .await;
            let vrma_play = task
                .will(
                    Update,
                    once::run(observe_stream::<VrmaPlayEvent>).with(("vrma-play", entity)),
                )
                .await;
            let vrma_finish = task
                .will(
                    Update,
                    once::run(observe_stream::<VrmaFinishEvent>).with(("vrma-finish", entity)),
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
                expression_change,
                vrma_play,
                vrma_finish,
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
