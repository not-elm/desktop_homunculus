use crate::extract::EntityId;
use axum::Json;
use axum::extract::{Path, State};
use bevy::prelude::Entity;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::{
    SpringBoneChain, SpringBoneChainsResponse, SpringBonePropsUpdate, VrmApi,
};

/// List all spring bone chains for a VRM model.
#[utoipa::path(
    get,
    path = "/spring-bones",
    tag = "vrm",
    params(("entity" = String, Path, description = "Entity ID")),
    responses(
        (status = 200, description = "Spring bone chains", body = SpringBoneChainsResponse),
        (status = 404, description = "Entity not found"),
    ),
)]
pub async fn list(
    State(api): State<VrmApi>,
    EntityId(entity): EntityId,
) -> HttpResult<SpringBoneChainsResponse> {
    api.list_spring_bones(entity).await.into_http_result()
}

/// Get a specific spring bone chain by ID.
#[utoipa::path(
    get,
    path = "/spring-bones/{chain_id}",
    tag = "vrm",
    params(
        ("entity" = String, Path, description = "VRM entity ID"),
        ("chain_id" = String, Path, description = "Spring bone chain entity ID"),
    ),
    responses(
        (status = 200, description = "Spring bone chain details", body = Option<SpringBoneChain>),
        (status = 404, description = "Entity or chain not found"),
    ),
)]
pub async fn get(
    State(api): State<VrmApi>,
    Path((entity, chain_id)): Path<(Entity, Entity)>,
) -> HttpResult<serde_json::Value> {
    let result = api.get_spring_bone(entity, chain_id).await?;
    Ok(Json(serde_json::to_value(result).unwrap()))
}

/// Update properties of a spring bone chain.
#[utoipa::path(
    put,
    path = "/spring-bones/{chain_id}",
    tag = "vrm",
    params(
        ("entity" = String, Path, description = "VRM entity ID"),
        ("chain_id" = String, Path, description = "Spring bone chain entity ID"),
    ),
    request_body = SpringBonePropsUpdate,
    responses(
        (status = 200, description = "Spring bone properties updated"),
        (status = 404, description = "Entity or chain not found"),
    ),
)]
pub async fn put(
    State(api): State<VrmApi>,
    Path((entity, chain_id)): Path<(Entity, Entity)>,
    Json(body): Json<SpringBonePropsUpdate>,
) -> HttpResult {
    api.set_spring_bone_props(entity, chain_id, body)
        .await
        .into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::tests::{assert_response, call, test_app};
    use bevy::prelude::*;
    use bevy_vrm1::prelude::{Initialized, SpringJointProps, SpringJoints, SpringRoot, Vrm};
    use homunculus_api::prelude::{SpringBoneChain, SpringBoneChainsResponse, SpringBoneProps};
    use homunculus_effects::Name;

    fn spawn_spring_chain(app: &mut App, vrm: Entity) -> (Entity, Entity) {
        let joint = app
            .world_mut()
            .spawn((
                Name::new("joint0"),
                SpringJointProps {
                    stiffness: 1.0,
                    drag_force: 0.4,
                    gravity_power: 0.0,
                    gravity_dir: Vec3::new(0.0, -1.0, 0.0),
                    hit_radius: 0.02,
                },
            ))
            .id();
        let chain_root = app
            .world_mut()
            .spawn((
                Name::new("chain_root"),
                SpringRoot {
                    joints: SpringJoints(vec![joint]),
                    ..default()
                },
            ))
            .id();
        app.world_mut().entity_mut(vrm).add_child(chain_root);
        (chain_root, joint)
    }

    #[tokio::test]
    async fn test_empty_spring_chain_list() {
        let (mut app, router) = test_app();

        let vrm = app
            .world_mut()
            .spawn((Name::new("Test1"), Vrm, Initialized))
            .id();
        app.update();

        let request = axum::http::Request::get(format!("/vrm/{}/spring-bones", vrm.to_bits()))
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(
            &mut app,
            router,
            request,
            SpringBoneChainsResponse { chains: vec![] },
        )
        .await;
    }

    #[tokio::test]
    async fn test_get_spring_bone() {
        let (mut app, router) = test_app();

        let vrm = app
            .world_mut()
            .spawn((Name::new("Test1"), Vrm, Initialized))
            .id();
        let (chain_root, _joint) = spawn_spring_chain(&mut app, vrm);
        app.update();

        let request = axum::http::Request::get(format!(
            "/vrm/{}/spring-bones/{}",
            vrm.to_bits(),
            chain_root.to_bits()
        ))
        .body(axum::body::Body::empty())
        .unwrap();

        let expected = SpringBoneChain {
            entity: chain_root,
            joints: vec!["joint0".to_string()],
            props: SpringBoneProps {
                stiffness: 1.0,
                drag_force: 0.4,
                gravity_power: 0.0,
                gravity_dir: [0.0, -1.0, 0.0],
                hit_radius: 0.02,
            },
        };
        assert_response(&mut app, router, request, Some(expected)).await;
    }

    #[tokio::test]
    async fn test_put_spring_bone() {
        let (mut app, router) = test_app();

        let vrm = app
            .world_mut()
            .spawn((Name::new("Test1"), Vrm, Initialized))
            .id();
        let (chain_root, joint) = spawn_spring_chain(&mut app, vrm);
        app.update();

        let request = axum::http::Request::put(format!(
            "/vrm/{}/spring-bones/{}",
            vrm.to_bits(),
            chain_root.to_bits()
        ))
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(
            serde_json::json!({
                "stiffness": 2.0,
                "dragForce": 0.8
            })
            .to_string(),
        ))
        .unwrap();

        call(&mut app, router, request).await;

        let props = app.world().entity(joint).get::<SpringJointProps>().unwrap();
        assert_eq!(props.stiffness, 2.0);
        assert_eq!(props.drag_force, 0.8);
        // unchanged fields
        assert_eq!(props.gravity_power, 0.0);
        assert_eq!(props.hit_radius, 0.02);
    }
}
