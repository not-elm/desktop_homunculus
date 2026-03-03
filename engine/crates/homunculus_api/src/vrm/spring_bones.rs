use crate::error::ApiResult;
use crate::vrm::VrmApi;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::{SpringJointProps, SpringRoot};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct SpringBoneChain {
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    pub entity: Entity,
    pub joints: Vec<String>,
    pub props: SpringBoneProps,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct SpringBoneProps {
    pub stiffness: f32,
    pub drag_force: f32,
    pub gravity_power: f32,
    pub gravity_dir: [f32; 3],
    pub hit_radius: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct SpringBoneChainsResponse {
    pub chains: Vec<SpringBoneChain>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct SpringBonePropsUpdate {
    pub stiffness: Option<f32>,
    pub drag_force: Option<f32>,
    pub gravity_power: Option<f32>,
    pub gravity_dir: Option<[f32; 3]>,
    pub hit_radius: Option<f32>,
}

impl VrmApi {
    pub async fn list_spring_bones(&self, vrm: Entity) -> ApiResult<SpringBoneChainsResponse> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(list_spring_bones).with(vrm))
                    .await
            })
            .await
    }

    pub async fn get_spring_bone(
        &self,
        vrm: Entity,
        chain: Entity,
    ) -> ApiResult<Option<SpringBoneChain>> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_spring_bone).with((vrm, chain)))
                    .await
            })
            .await
    }

    pub async fn set_spring_bone_props(
        &self,
        vrm: Entity,
        chain: Entity,
        update: SpringBonePropsUpdate,
    ) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(
                    Update,
                    once::run(set_spring_bone_props).with((vrm, chain, update)),
                )
                .await;
            })
            .await
    }
}

fn props_from(p: &SpringJointProps) -> SpringBoneProps {
    SpringBoneProps {
        stiffness: p.stiffness,
        drag_force: p.drag_force,
        gravity_power: p.gravity_power,
        gravity_dir: p.gravity_dir.to_array(),
        hit_radius: p.hit_radius,
    }
}

fn default_props() -> SpringBoneProps {
    SpringBoneProps {
        stiffness: 1.0,
        drag_force: 0.4,
        gravity_power: 0.0,
        gravity_dir: [0.0, -1.0, 0.0],
        hit_radius: 0.02,
    }
}

fn list_spring_bones(
    In(vrm): In<Entity>,
    spring_roots: Query<&SpringRoot>,
    names: Query<&Name>,
    joint_props: Query<&SpringJointProps>,
    children: Query<&Children>,
) -> SpringBoneChainsResponse {
    SpringBoneChainsResponse {
        chains: find_all_spring_bones(vrm, &spring_roots, &names, &joint_props, &children),
    }
}

fn get_spring_bone(
    In((_vrm, chain)): In<(Entity, Entity)>,
    spring_roots: Query<&SpringRoot>,
    names: Query<&Name>,
    joint_props: Query<&SpringJointProps>,
) -> Option<SpringBoneChain> {
    let root = spring_roots.get(chain).ok()?;
    let joint_names: Vec<String> = root
        .joints
        .iter()
        .map(|&e| {
            names
                .get(e)
                .map(|n| n.to_string())
                .unwrap_or_else(|_| format!("{}", e))
        })
        .collect();
    let props = root
        .joints
        .first()
        .and_then(|&e| joint_props.get(e).ok())
        .map(props_from)
        .unwrap_or_else(default_props);
    Some(SpringBoneChain {
        entity: chain,
        joints: joint_names,
        props,
    })
}

fn find_all_spring_bones(
    entity: Entity,
    spring_roots: &Query<&SpringRoot>,
    names: &Query<&Name>,
    joint_props: &Query<&SpringJointProps>,
    children: &Query<&Children>,
) -> Vec<SpringBoneChain> {
    let mut chains = Vec::new();

    let Ok(vrm_children) = children.get(entity) else {
        return chains;
    };

    for child in vrm_children.iter() {
        if let Ok(root) = spring_roots.get(child) {
            let props = root
                .joints
                .first()
                .and_then(|&e| joint_props.get(e).ok())
                .map(props_from)
                .unwrap_or_else(default_props);
            chains.push(SpringBoneChain {
                entity: child,
                joints: root
                    .joints
                    .iter()
                    .flat_map(|e| names.get(*e).ok())
                    .map(|n| format!("{n}"))
                    .collect(),
                props,
            });
        };
        chains.extend(find_all_spring_bones(
            child,
            spring_roots,
            names,
            joint_props,
            children,
        ));
    }
    chains
}

fn set_spring_bone_props(
    In((_vrm, chain, update)): In<(Entity, Entity, SpringBonePropsUpdate)>,
    spring_roots: Query<&SpringRoot>,
    mut joint_props: Query<&mut SpringJointProps>,
) {
    let Ok(root) = spring_roots.get(chain) else {
        return;
    };
    for &joint_entity in root.joints.iter() {
        if let Ok(mut props) = joint_props.get_mut(joint_entity) {
            if let Some(stiffness) = update.stiffness {
                props.stiffness = stiffness;
            }
            if let Some(drag_force) = update.drag_force {
                props.drag_force = drag_force;
            }
            if let Some(gravity_power) = update.gravity_power {
                props.gravity_power = gravity_power;
            }
            if let Some(gravity_dir) = update.gravity_dir {
                props.gravity_dir = Vec3::from_array(gravity_dir);
            }
            if let Some(hit_radius) = update.hit_radius {
                props.hit_radius = hit_radius;
            }
        }
    }
}
