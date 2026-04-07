use crate::error::ApiResult;
use crate::persona::PersonaApi;
use crate::vrm::expressions::{ExpressionInfo, ExpressionsResponse};
use crate::vrm::snapshot::LookAtState;
use crate::vrma::VrmaInfo;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::*;
use homunculus_core::prelude::{AssetIdComponent, LinkedPersona, Persona, PersonaState};
use serde::{Deserialize, Serialize};

/// Full snapshot of a persona including transform, linked webviews, and VRM state.
///
/// Returned by `GET /personas/snapshot`.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct PersonaFullSnapshot {
    #[serde(flatten)]
    pub persona: Persona,
    /// Current ephemeral state.
    pub state: String,
    /// World-space transform.
    #[cfg_attr(feature = "openapi", schema(value_type = Object))]
    pub transform: Transform,
    /// Entity IDs of linked webviews (as strings to avoid JS 64-bit precision loss).
    #[cfg_attr(feature = "openapi", schema(value_type = Vec<String>))]
    pub linked_webviews: Vec<String>,
    /// VRM-specific data, or `null` if no VRM is attached.
    pub vrm: Option<VrmInfo>,
}

/// VRM-specific rendering state nested within a persona full snapshot.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct VrmInfo {
    pub asset_id: String,
    pub expressions: ExpressionsResponse,
    pub animations: Vec<VrmaInfo>,
    pub look_at: Option<LookAtState>,
    #[cfg_attr(feature = "openapi", schema(value_type = Object))]
    pub spring_bones: serde_json::Value,
}

impl PersonaApi {
    /// Returns a full snapshot of all personas with VRM rendering state.
    pub async fn full_snapshot(&self) -> ApiResult<Vec<PersonaFullSnapshot>> {
        self.0
            .schedule(move |task| async move { task.will(Update, once::run(snapshot_all)).await })
            .await
    }
}

#[allow(clippy::too_many_arguments)]
fn snapshot_all(
    personas: Query<(Entity, &Persona, &PersonaState, &Transform)>,
    vrm_handles: Query<&VrmHandle>,
    asset_ids: Query<&AssetIdComponent>,
    look_ats: Query<&LookAt>,
    expr_maps: Query<&ExpressionEntityMap>,
    expr_components: Query<(
        &Transform,
        Option<&ExpressionOverride>,
        Option<&BinaryExpression>,
        Option<&ExpressionOverrideSettings>,
    )>,
    children_query: Query<&Children>,
    vrma_query: Query<(Entity, &Name, &VrmaAnimationPlayers), With<Vrma>>,
    players: Query<&AnimationPlayer>,
    linked_personas: Query<(Entity, &LinkedPersona)>,
) -> Vec<PersonaFullSnapshot> {
    personas
        .iter()
        .map(|(entity, persona, state, transform)| {
            let linked_webviews = collect_linked_webviews(&persona.id, &linked_personas);
            let vrm = build_vrm_info(
                entity,
                &vrm_handles,
                &asset_ids,
                &look_ats,
                &expr_maps,
                &expr_components,
                &children_query,
                &vrma_query,
                &players,
            );

            PersonaFullSnapshot {
                persona: persona.clone(),
                state: state.0.clone(),
                transform: *transform,
                linked_webviews,
                vrm,
            }
        })
        .collect()
}

/// Collects entity IDs of all webviews linked to the given persona.
fn collect_linked_webviews(
    persona_id: &homunculus_core::prelude::PersonaId,
    linked_personas: &Query<(Entity, &LinkedPersona)>,
) -> Vec<String> {
    linked_personas
        .iter()
        .filter(|(_, linked)| &linked.0 == persona_id)
        .map(|(webview_entity, _)| format!("{}", webview_entity.to_bits()))
        .collect()
}

/// Builds the VRM info section if a VRM is attached to the entity.
#[allow(clippy::too_many_arguments)]
fn build_vrm_info(
    entity: Entity,
    vrm_handles: &Query<&VrmHandle>,
    asset_ids: &Query<&AssetIdComponent>,
    look_ats: &Query<&LookAt>,
    expr_maps: &Query<&ExpressionEntityMap>,
    expr_components: &Query<(
        &Transform,
        Option<&ExpressionOverride>,
        Option<&BinaryExpression>,
        Option<&ExpressionOverrideSettings>,
    )>,
    children_query: &Query<&Children>,
    vrma_query: &Query<(Entity, &Name, &VrmaAnimationPlayers), With<Vrma>>,
    players: &Query<&AnimationPlayer>,
) -> Option<VrmInfo> {
    vrm_handles.get(entity).ok()?;

    let asset_id = asset_ids
        .get(entity)
        .map(|c| c.0.to_string())
        .unwrap_or_default();

    let expressions = collect_expressions(expr_maps.get(entity).ok(), expr_components);
    let animations = collect_playing_animations(entity, children_query, vrma_query, players);
    let look_at = look_ats.get(entity).ok().map(|la| match la {
        LookAt::Cursor => LookAtState::Cursor,
        LookAt::Target(target) => LookAtState::Target { entity: *target },
    });

    Some(VrmInfo {
        asset_id,
        expressions,
        animations,
        look_at,
        spring_bones: serde_json::Value::Object(Default::default()),
    })
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
                    crate::vrm::expressions::override_type_to_string(&settings.override_blink)
                        .into(),
                    crate::vrm::expressions::override_type_to_string(&settings.override_look_at)
                        .into(),
                    crate::vrm::expressions::override_type_to_string(&settings.override_mouth)
                        .into(),
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
