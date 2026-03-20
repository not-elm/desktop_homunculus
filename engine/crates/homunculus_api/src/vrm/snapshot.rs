use crate::error::ApiResult;
use crate::vrm::VrmApi;
use crate::vrm::expressions::{ExpressionInfo, ExpressionsResponse};
use crate::vrma::VrmaInfo;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::*;
use homunculus_core::prelude::{
    AssetIdComponent, AvatarState, Coordinate, GlobalViewport, LinkedAvatar, Persona,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct VrmSnapshot {
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    pub entity: Entity,
    pub name: String,
    pub state: String,
    #[cfg_attr(feature = "openapi", schema(value_type = Object))]
    pub transform: Transform,
    #[cfg_attr(feature = "openapi", schema(value_type = Option<Object>))]
    pub global_viewport: Option<GlobalViewport>,
    pub expressions: ExpressionsResponse,
    pub animations: Vec<VrmaInfo>,
    pub look_at: Option<LookAtState>,
    #[cfg_attr(feature = "openapi", schema(value_type = Vec<String>))]
    pub linked_webviews: Vec<Entity>,
    #[cfg_attr(feature = "openapi", schema(value_type = Object))]
    pub persona: Persona,
    pub asset_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum LookAtState {
    Cursor,
    Target {
        #[cfg_attr(feature = "openapi", schema(value_type = String))]
        entity: Entity,
    },
}

impl VrmApi {
    pub async fn snapshot(&self) -> ApiResult<Vec<VrmSnapshot>> {
        self.0
            .schedule(
                move |task| async move { task.will(Update, once::run(snapshot_all_vrms)).await },
            )
            .await
    }
}

#[allow(clippy::too_many_arguments)]
fn snapshot_all_vrms(
    vrms: Query<
        (
            Entity,
            &Name,
            Option<&AvatarState>,
            &Transform,
            Option<&LookAt>,
            Option<&Persona>,
            Option<&ExpressionEntityMap>,
            Option<&AssetIdComponent>,
        ),
        With<Vrm>,
    >,
    expr_components: Query<(
        &Transform,
        Option<&ExpressionOverride>,
        Option<&BinaryExpression>,
        Option<&ExpressionOverrideSettings>,
    )>,
    children_query: Query<&Children>,
    vrma_query: Query<(Entity, &Name, &VrmaAnimationPlayers), With<Vrma>>,
    players: Query<&AnimationPlayer>,
    coordinate: Coordinate,
    registry: Res<homunculus_core::prelude::AvatarRegistry>,
    linked_avatars: Query<(Entity, &LinkedAvatar)>,
) -> Vec<VrmSnapshot> {
    vrms.iter()
        .map(
            |(entity, name, state, transform, look_at, persona, expr_map, asset_id_comp)| {
                let expressions = collect_expressions(expr_map, &expr_components);
                let animations =
                    collect_playing_animations(entity, &children_query, &vrma_query, &players);
                let global_viewport = coordinate.to_global_by_world(transform.translation);
                let look_at_state = look_at.map(|la| match la {
                    LookAt::Cursor => LookAtState::Cursor,
                    LookAt::Target(target) => LookAtState::Target { entity: *target },
                });
                let linked_webviews = collect_linked_webviews(
                    entity,
                    &linked_avatars,
                    &registry,
                );

                VrmSnapshot {
                    entity,
                    name: name.to_string(),
                    state: state
                        .map(|s| s.0.clone())
                        .unwrap_or_else(|| "idle".to_string()),
                    transform: *transform,
                    global_viewport,
                    expressions,
                    animations,
                    look_at: look_at_state,
                    linked_webviews,
                    persona: persona.cloned().unwrap_or_default(),
                    asset_id: asset_id_comp.map(|c| c.0.to_string()),
                }
            },
        )
        .collect()
}

fn collect_expressions(
    expr_map: Option<&ExpressionEntityMap>,
    expr_components: &Query<(
        &Transform,
        Option<&ExpressionOverride>,
        Option<&BinaryExpression>,
        Option<&ExpressionOverrideSettings>,
    )>,
) -> ExpressionsResponse {
    let Some(map) = expr_map else {
        return ExpressionsResponse {
            expressions: Vec::new(),
        };
    };

    let mut expressions: Vec<ExpressionInfo> = map
        .iter()
        .filter_map(|(expr_name, &expr_entity)| {
            let (tf, maybe_override, maybe_binary, maybe_settings) =
                expr_components.get(expr_entity).ok()?;

            let weight = match maybe_override {
                Some(ExpressionOverride(w)) => *w,
                None => tf.translation.x,
            };

            let (override_blink, override_look_at, override_mouth) = match maybe_settings {
                Some(settings) => (
                    super::expressions::override_type_to_string(&settings.override_blink).into(),
                    super::expressions::override_type_to_string(&settings.override_look_at).into(),
                    super::expressions::override_type_to_string(&settings.override_mouth).into(),
                ),
                None => ("none".into(), "none".into(), "none".into()),
            };

            Some(ExpressionInfo {
                name: expr_name.to_string(),
                weight: weight.clamp(0.0, 1.0),
                is_binary: maybe_binary.is_some(),
                override_blink,
                override_look_at,
                override_mouth,
            })
        })
        .collect();

    expressions.sort_by(|a, b| a.name.cmp(&b.name));

    ExpressionsResponse { expressions }
}

/// Collects webview entities linked to a given VRM entity via LinkedAvatar.
fn collect_linked_webviews(
    vrm_entity: Entity,
    linked_avatars: &Query<(Entity, &LinkedAvatar)>,
    registry: &homunculus_core::prelude::AvatarRegistry,
) -> Vec<Entity> {
    linked_avatars
        .iter()
        .filter(|(_, linked)| {
            homunculus_core::avatar::AvatarId::new(&linked.0)
                .ok()
                .and_then(|id| registry.get(&id))
                == Some(vrm_entity)
        })
        .map(|(webview_entity, _)| webview_entity)
        .collect()
}

fn collect_playing_animations(
    vrm: Entity,
    children: &Query<&Children>,
    vrma_query: &Query<(Entity, &Name, &VrmaAnimationPlayers), With<Vrma>>,
    players: &Query<&AnimationPlayer>,
) -> Vec<VrmaInfo> {
    let mut result = Vec::new();
    let Ok(vrm_children) = children.get(vrm) else {
        return result;
    };
    for child in vrm_children.iter() {
        if let Ok((entity, name, animation_players)) = vrma_query.get(child) {
            let playing = animation_players
                .0
                .iter()
                .any(|&pe| players.get(pe).map(|p| !p.all_finished()).unwrap_or(false));
            if playing {
                result.push(VrmaInfo {
                    entity,
                    name: name.to_string(),
                    playing: true,
                });
            }
        }
    }
    result
}
