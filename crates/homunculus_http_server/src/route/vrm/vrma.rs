use crate::extract::EntityId;
use crate::route::ModuleSourceRequest;
use axum::extract::{Query, State};
use bevy::prelude::Entity;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::VrmApi;

/// Fetch or load a VRM animation.
///
/// ### Path
///
/// `POST /vrm/:entity_id/vrma?source=<VRMA_SOURCE>`
pub async fn vrma(
    State(api): State<VrmApi>,
    EntityId(vrm_entity): EntityId,
    Query(query): Query<ModuleSourceRequest>,
) -> HttpResult<Entity> {
    api.vrma(vrm_entity, query.source).await.into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::tests::{assert_response, test_app};
    use bevy::prelude::Name;
    use bevy_vrm1::prelude::Initialized;
    use homunculus_core::prelude::ModModuleSource;

    #[tokio::test]
    async fn test_get_exists_vrma() {
        let (mut app, router) = test_app();
        let entity = app.world_mut().spawn(Name::new("VRM")).id();
        let vrma_entity = app
            .world_mut()
            .spawn((ModModuleSource("test".to_string()), Initialized))
            .id();
        app.world_mut()
            .commands()
            .entity(entity)
            .add_child(vrma_entity);
        app.update();
        let request =
            axum::http::Request::get(format!("/vrm/{}/vrma?source=test", entity.to_bits()))
                .body(axum::body::Body::empty())
                .unwrap();
        assert_response(&mut app, router, request, vrma_entity).await;
    }
}
