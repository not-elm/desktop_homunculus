use crate::error::ApiResult;
use crate::vrm::VrmApi;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::*;
use homunculus_core::prelude::{ExpressionChangeEvent, OutputLog, VrmEvent, VrmEventSender};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ExpressionInfo {
    pub name: String,
    pub weight: f32,
    pub is_binary: bool,
    pub override_blink: String,
    pub override_look_at: String,
    pub override_mouth: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ExpressionsResponse {
    pub expressions: Vec<ExpressionInfo>,
}

impl VrmApi {
    pub async fn list_expressions(&self, vrm: Entity) -> ApiResult<ExpressionsResponse> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(list_expressions).with(vrm))
                    .await
            })
            .await
    }

    pub async fn set_expressions(&self, vrm: Entity, weights: HashMap<String, f32>) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(do_set_expressions).with((vrm, weights)))
                    .await;
            })
            .await
    }

    pub async fn modify_expressions(
        &self,
        vrm: Entity,
        weights: HashMap<String, f32>,
    ) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(
                    Update,
                    once::run(do_modify_expressions).with((vrm, weights)),
                )
                .await;
            })
            .await
    }

    pub async fn modify_mouth(&self, vrm: Entity, weights: HashMap<String, f32>) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(do_modify_mouth).with((vrm, weights)))
                    .await;
            })
            .await
    }

    pub async fn clear_expressions(&self, vrm: Entity) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(do_clear_expressions).with(vrm))
                    .await;
            })
            .await
    }
}

pub(crate) fn override_type_to_string(ty: &ExpressionOverrideType) -> &'static str {
    match ty {
        ExpressionOverrideType::None => "none",
        ExpressionOverrideType::Block => "block",
        ExpressionOverrideType::Blend => "blend",
    }
}

fn list_expressions(
    In(vrm): In<Entity>,
    entity_map: Query<&ExpressionEntityMap>,
    overrides: Query<(
        &Transform,
        Option<&ExpressionOverride>,
        Option<&BinaryExpression>,
        Option<&ExpressionOverrideSettings>,
    )>,
) -> ExpressionsResponse {
    let Ok(map) = entity_map.get(vrm) else {
        return ExpressionsResponse {
            expressions: Vec::new(),
        };
    };

    let mut expressions: Vec<ExpressionInfo> = map
        .iter()
        .filter_map(|(expr_name, &expr_entity)| {
            let (tf, maybe_override, maybe_binary, maybe_settings) =
                overrides.get(expr_entity).ok()?;

            let weight = match maybe_override {
                Some(ExpressionOverride(w)) => *w,
                None => tf.translation.x,
            };

            let (override_blink, override_look_at, override_mouth) = match maybe_settings {
                Some(settings) => (
                    override_type_to_string(&settings.override_blink).into(),
                    override_type_to_string(&settings.override_look_at).into(),
                    override_type_to_string(&settings.override_mouth).into(),
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

fn do_set_expressions(
    In((vrm, weights)): In<(Entity, HashMap<String, f32>)>,
    mut commands: Commands,
    tx: Option<Res<VrmEventSender<ExpressionChangeEvent>>>,
) {
    let clamped: HashMap<String, f32> = weights
        .into_iter()
        .map(|(k, v)| (k, v.clamp(0.0, 1.0)))
        .collect();

    commands.trigger(SetExpressions::from_iter(
        vrm,
        clamped.iter().map(|(k, &v)| (k.as_str(), v)),
    ));

    if let Some(tx) = tx {
        tx.try_broadcast(VrmEvent {
            vrm,
            payload: ExpressionChangeEvent {
                expressions: clamped,
            },
        })
        .output_log_if_error("Failed to broadcast ExpressionChangeEvent");
    }
}

fn do_modify_expressions(
    In((vrm, weights)): In<(Entity, HashMap<String, f32>)>,
    mut commands: Commands,
    tx: Option<Res<VrmEventSender<ExpressionChangeEvent>>>,
) {
    let clamped: HashMap<String, f32> = weights
        .into_iter()
        .map(|(k, v)| (k, v.clamp(0.0, 1.0)))
        .collect();

    commands.trigger(ModifyExpressions::from_iter(
        vrm,
        clamped.iter().map(|(k, &v)| (k.as_str(), v)),
    ));

    if let Some(tx) = tx {
        tx.try_broadcast(VrmEvent {
            vrm,
            payload: ExpressionChangeEvent {
                expressions: clamped,
            },
        })
        .output_log_if_error("Failed to broadcast ExpressionChangeEvent");
    }
}

fn do_modify_mouth(
    In((vrm, weights)): In<(Entity, HashMap<String, f32>)>,
    mut commands: Commands,
    tx: Option<Res<VrmEventSender<ExpressionChangeEvent>>>,
) {
    let clamped: HashMap<String, f32> = weights
        .into_iter()
        .map(|(k, v)| (k, v.clamp(0.0, 1.0)))
        .collect();

    commands.trigger(ModifyExpressions::mouth_weights(
        vrm,
        clamped.iter().map(|(k, &v)| (k.as_str(), v)),
    ));

    if let Some(tx) = tx {
        tx.try_broadcast(VrmEvent {
            vrm,
            payload: ExpressionChangeEvent {
                expressions: clamped,
            },
        })
        .output_log_if_error("Failed to broadcast ExpressionChangeEvent");
    }
}

fn do_clear_expressions(
    In(vrm): In<Entity>,
    mut commands: Commands,
    tx: Option<Res<VrmEventSender<ExpressionChangeEvent>>>,
) {
    commands.trigger(ClearExpressions { entity: vrm });

    if let Some(tx) = tx {
        tx.try_broadcast(VrmEvent {
            vrm,
            payload: ExpressionChangeEvent {
                expressions: HashMap::new(),
            },
        })
        .output_log_if_error("Failed to broadcast ExpressionChangeEvent");
    }
}
