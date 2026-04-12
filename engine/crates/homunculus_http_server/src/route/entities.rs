pub mod move_to;
pub mod name;
pub mod transform;
pub mod tween;

use axum::extract::{Query, State};
use bevy::prelude::{Entity, Name};
use homunculus_api::prelude::EntitiesApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Find an entity by its name.
///
/// If `root` entity is specified, it will search recursively from that entity's children.
#[utoipa::path(
    get,
    path = "/",
    tag = "entities",
    params(
        ("name" = String, Query, description = "Entity name to search for"),
        ("root" = Option<String>, Query, description = "Root entity to search from recursively"),
    ),
    responses(
        (status = 200, description = "Found entity ID", body = String),
        (status = 404, description = "Entity not found"),
    ),
)]
pub async fn find_entity(
    State(api): State<EntitiesApi>,
    Query(query): Query<EntitiesFindQuery>,
) -> HttpResult<Entity> {
    api.find_by_name(query.name, query.root)
        .await
        .into_http_result()
}

/// Query parameters for finding an entity by name.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, ToSchema)]
pub struct EntitiesFindQuery {
    /// The name of the entity to find.
    #[schema(value_type = String)]
    pub name: Name,
    /// The root entity to search from.
    ///
    /// - If not specified, it will search globally.
    /// - If specified, it will search recursively from this entity's children.
    #[schema(value_type = Option<String>)]
    pub root: Option<Entity>,
}

#[cfg(test)]
mod tests {
    use crate::tests::{assert_response, test_app};
    use bevy::prelude::Name;

    #[tokio::test]
    async fn find_by_name() {
        let (mut app, router) = test_app();
        let entity = app.world_mut().spawn(Name::new("VRM")).id();
        app.update();
        let request = axum::http::Request::get("/entities?name=VRM")
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, entity).await;
    }

    #[tokio::test]
    async fn find_recursive_by_name() {
        let (mut app, router) = test_app();
        let root = app.world_mut().spawn(Name::new("VRM")).id();
        for _ in 0..10 {
            app.world_mut().spawn(Name::new("Child"));
        }
        let child = app.world_mut().spawn(Name::new("Child")).id();
        app.world_mut().entity_mut(root).add_child(child);
        for _ in 0..10 {
            app.world_mut().spawn(Name::new("Child"));
        }
        app.update();

        let request =
            axum::http::Request::get(format!("/entities?name=Child&root={}", root.to_bits()))
                .body(axum::body::Body::empty())
                .unwrap();
        assert_response(&mut app, router, request, child).await;
    }
}
