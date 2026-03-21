use crate::extract::character::VrmGuard;
use axum::Json;
use axum::extract::{Path, State};
use bevy::prelude::Entity;
use homunculus_api::character::CharacterApi;
use homunculus_api::prelude::ApiError;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::{
    SpringBoneChain, SpringBoneChainsResponse, SpringBonePropsUpdate, VrmApi,
};
use homunculus_core::prelude::CharacterId;

/// List all spring bone chains for a character's VRM model.
#[utoipa::path(
    get,
    path = "/spring-bones",
    tag = "vrm",
    params(("id" = String, Path, description = "Character ID")),
    responses(
        (status = 200, description = "Spring bone chains", body = SpringBoneChainsResponse),
        (status = 404, description = "Character not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn list(
    State(api): State<VrmApi>,
    VrmGuard { entity, .. }: VrmGuard,
) -> HttpResult<SpringBoneChainsResponse> {
    api.list_spring_bones(entity).await.into_http_result()
}

/// Get a specific spring bone chain by ID.
#[utoipa::path(
    get,
    path = "/spring-bones/{chain_id}",
    tag = "vrm",
    params(
        ("id" = String, Path, description = "Character ID"),
        ("chain_id" = String, Path, description = "Spring bone chain entity ID"),
    ),
    responses(
        (status = 200, description = "Spring bone chain details", body = Option<SpringBoneChain>),
        (status = 404, description = "Character or chain not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn get(
    State(vrm_api): State<VrmApi>,
    State(char_api): State<CharacterApi>,
    Path((id_str, chain_id)): Path<(String, Entity)>,
) -> HttpResult<serde_json::Value> {
    let id = CharacterId::new(&id_str).map_err(|e| ApiError::InvalidCharacterId(e.to_string()))?;
    let entity = char_api.resolve_with_vrm(id).await?;
    let result = vrm_api.get_spring_bone(entity, chain_id).await?;
    Ok(Json(serde_json::to_value(result).unwrap()))
}

/// Update properties of a spring bone chain.
#[utoipa::path(
    put,
    path = "/spring-bones/{chain_id}",
    tag = "vrm",
    params(
        ("id" = String, Path, description = "Character ID"),
        ("chain_id" = String, Path, description = "Spring bone chain entity ID"),
    ),
    request_body = SpringBonePropsUpdate,
    responses(
        (status = 200, description = "Spring bone properties updated"),
        (status = 404, description = "Character or chain not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn put(
    State(vrm_api): State<VrmApi>,
    State(char_api): State<CharacterApi>,
    Path((id_str, chain_id)): Path<(String, Entity)>,
    Json(body): Json<SpringBonePropsUpdate>,
) -> HttpResult {
    let id = CharacterId::new(&id_str).map_err(|e| ApiError::InvalidCharacterId(e.to_string()))?;
    let entity = char_api.resolve_with_vrm(id).await?;
    vrm_api
        .set_spring_bone_props(entity, chain_id, body)
        .await
        .into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::tests::{assert_response, call, spawn_character_with_vrm, test_app};
    use bevy::prelude::*;
    use bevy_vrm1::prelude::{Initialized, SpringJointProps, SpringJoints, SpringRoot};
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
        let vrm = spawn_character_with_vrm(&mut app, "test-char");
        app.world_mut().entity_mut(vrm).insert(Initialized);

        let request = axum::http::Request::get("/characters/test-char/vrm/spring-bones")
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
        let vrm = spawn_character_with_vrm(&mut app, "test-char");
        app.world_mut().entity_mut(vrm).insert(Initialized);
        let (chain_root, _joint) = spawn_spring_chain(&mut app, vrm);

        let request = axum::http::Request::get(format!(
            "/characters/test-char/vrm/spring-bones/{}",
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
        let vrm = spawn_character_with_vrm(&mut app, "test-char");
        app.world_mut().entity_mut(vrm).insert(Initialized);
        let (chain_root, joint) = spawn_spring_chain(&mut app, vrm);

        let request = axum::http::Request::put(format!(
            "/characters/test-char/vrm/spring-bones/{}",
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
