use axum::extract::State;
use axum::response::Sse;
use axum::response::sse::{Event, KeepAlive};
use bevy::prelude::{Entity, Res, Update};
use bevy::tasks::futures_lite::Stream;
use bevy_flurx::action::once;
use futures::stream::select_all;
use homunculus_api::prelude::ApiReactor;
use homunculus_core::prelude::{
    PersonaChangeEvent, PersonaDeletedEvent, PersonaEvent, PersonaId, PersonaIndex,
    PersonaStateChangeEvent, VrmAttachedEvent, VrmDetachedEvent, VrmEventReceiver,
};
use std::collections::HashMap;
use std::convert::Infallible;
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use std::time::Duration;

/// Reverse lookup from Entity to PersonaId, shared across stream tasks.
type ReverseIndex = Arc<RwLock<HashMap<Entity, PersonaId>>>;

/// Subscribe to a combined SSE stream of events from ALL personas.
///
/// Each event is wrapped in `PersonaEvent<E>` containing the source `personaId`.
/// Includes `persona-deleted` events that are not available on per-persona streams.
#[utoipa::path(
    get,
    path = "/stream",
    tag = "personas",
    responses(
        (status = 200, description = "Combined SSE stream of all persona events", content_type = "text/event-stream"),
    ),
)]
pub async fn stream(
    State(reactor): State<ApiReactor>,
) -> Result<
    Sse<impl Stream<Item = Result<Event, Infallible>> + Send + Sync>,
    homunculus_api::prelude::ApiError,
> {
    let stream = reactor
        .schedule(
            move |task| async move { task.will(Update, once::run(build_combined_stream)).await },
        )
        .await?;

    Ok(Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(30))))
}

/// Builds the combined SSE stream by subscribing to all persona-level event channels.
fn build_combined_stream(
    index: Res<PersonaIndex>,
    rx_change: Res<VrmEventReceiver<PersonaChangeEvent>>,
    rx_state: Res<VrmEventReceiver<PersonaStateChangeEvent>>,
    rx_attached: Res<VrmEventReceiver<VrmAttachedEvent>>,
    rx_detached: Res<VrmEventReceiver<VrmDetachedEvent>>,
    rx_deleted: Res<VrmEventReceiver<PersonaDeletedEvent>>,
) -> Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send + Sync + 'static>> {
    let reverse: ReverseIndex = Arc::new(RwLock::new(build_reverse_index(&index)));

    let streams: Vec<Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send + Sync>>> = vec![
        persona_change_stream(rx_change.clone(), Arc::clone(&reverse)),
        entity_event_stream("state-change", rx_state.clone(), Arc::clone(&reverse)),
        entity_event_stream("vrm-attached", rx_attached.clone(), Arc::clone(&reverse)),
        entity_event_stream("vrm-detached", rx_detached.clone(), Arc::clone(&reverse)),
        persona_deleted_stream(rx_deleted.clone(), Arc::clone(&reverse)),
    ];

    Box::pin(select_all(streams))
}

/// Builds an Entity -> PersonaId reverse lookup from the current [`PersonaIndex`].
fn build_reverse_index(index: &PersonaIndex) -> HashMap<Entity, PersonaId> {
    index.0.iter().map(|(id, &e)| (e, id.clone())).collect()
}

/// Stream for `persona-change` events.
///
/// The persona_id is extracted from the payload and the reverse index is updated
/// so that subsequent events for the same entity can be resolved.
fn persona_change_stream(
    rx: VrmEventReceiver<PersonaChangeEvent>,
    reverse: ReverseIndex,
) -> Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send + Sync + 'static>> {
    let stream = futures::stream::unfold((rx, reverse), |(mut rx, reverse)| async move {
        let event = rx.recv().await.ok()?;
        let persona_id = event.payload.persona.id.clone();

        if let Ok(mut map) = reverse.write() {
            map.insert(event.vrm, persona_id.clone());
        }

        let wrapped = PersonaEvent {
            persona_id,
            payload: event.payload,
        };
        let sse = Event::default()
            .event("persona-change")
            .data(serde_json::to_string(&wrapped).unwrap());
        Some((Ok(sse), (rx, reverse)))
    });
    Box::pin(stream)
}

/// Stream for `persona-deleted` events.
///
/// The persona_id is already embedded in the payload. The entity is removed
/// from the reverse index since it is no longer valid.
fn persona_deleted_stream(
    rx: VrmEventReceiver<PersonaDeletedEvent>,
    reverse: ReverseIndex,
) -> Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send + Sync + 'static>> {
    let stream = futures::stream::unfold((rx, reverse), |(mut rx, reverse)| async move {
        let event = rx.recv().await.ok()?;
        let persona_id = event.payload.persona_id.clone();

        if let Ok(mut map) = reverse.write() {
            map.remove(&event.vrm);
        }

        let wrapped = PersonaEvent {
            persona_id,
            payload: event.payload,
        };
        let sse = Event::default()
            .event("persona-deleted")
            .data(serde_json::to_string(&wrapped).unwrap());
        Some((Ok(sse), (rx, reverse)))
    });
    Box::pin(stream)
}

/// Stream for entity-keyed events (state-change, vrm-attached, vrm-detached).
///
/// Resolves the entity to a [`PersonaId`] via the shared reverse index.
/// Events for entities not found in the index are silently skipped.
fn entity_event_stream<E>(
    event_name: &'static str,
    rx: VrmEventReceiver<E>,
    reverse: ReverseIndex,
) -> Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send + Sync + 'static>>
where
    E: serde::Serialize + Clone + Send + Sync + 'static,
{
    let stream = futures::stream::unfold((rx, reverse), move |(mut rx, reverse)| async move {
        loop {
            let event = rx.recv().await.ok()?;

            let persona_id = reverse
                .read()
                .ok()
                .and_then(|map| map.get(&event.vrm).cloned());
            let Some(persona_id) = persona_id else {
                continue;
            };

            let wrapped = PersonaEvent {
                persona_id,
                payload: event.payload,
            };
            let sse = Event::default()
                .event(event_name)
                .data(serde_json::to_string(&wrapped).unwrap());
            return Some((Ok(sse), (rx, reverse)));
        }
    });
    Box::pin(stream)
}
