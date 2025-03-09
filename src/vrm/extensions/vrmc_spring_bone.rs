use bevy::math::NormedVectorSpace;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VRMCSpringBone {
    /// Represents the specification version of the VRMC_springBone extension.
    #[serde(rename = "specVersion")]
    pub spec_version: String,

    /// [Collider]
    pub colliders: Vec<Collider>,

    /// [ColliderGroup]
    #[serde(rename = "colliderGroups")]
    pub collider_groups: Vec<ColliderGroup>,

    /// [Spring]
    pub springs: Vec<Spring>,
}

impl VRMCSpringBone {
    pub fn all_joints(&self) -> Vec<SpringJoint> {
        self
            .springs
            .iter()
            .flat_map(|spring| spring.joints.clone())
            .collect()
    }

    pub fn spring_colliders(&self, collider_group_indices: &[usize]) -> Vec<Collider> {
        collider_group_indices
            .iter()
            .flat_map(|index| self.collider_groups[*index].colliders.clone())
            .flat_map(|index| self.colliders.get(index as usize).cloned())
            .collect()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ColliderGroup {
    /// Group name
    pub name: String,

    /// The list of colliders belonging to this group.
    /// Each value is an index of `VRMCSpringBone::colliders`.
    pub colliders: Vec<u64>,
}

/// Represents the collision detection for SpringBone.
/// It consists of the target node index and the collider shape.
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Collider {
    pub node: usize,
    pub shape: ColliderShape,
}

#[derive(Serialize, Deserialize)]
pub struct Spring {
    /// Spring name
    pub name: String,

    /// The list of joints that make up the springBone.
    pub joints: Vec<SpringJoint>,

    /// Each value is an index of `VRMCSpringBone::colliderGroups`.
    #[serde(rename = "colliderGroups")]
    pub collider_groups: Option<Vec<usize>>,

    pub center: Option<usize>,
}

/// The node of a single glTF with SpringBone settings.
/// The node of a single glTF with SpringBone settings.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct SpringJoint {
    pub node: usize,
    #[serde(rename = "dragForce")]
    pub drag_force: Option<f32>,
    #[serde(rename = "gravityDir")]
    pub gravity_dir: Option<[f32; 3]>,
    #[serde(rename = "gravityPower")]
    pub gravity_power: Option<f32>,
    #[serde(rename = "hitRadius")]
    pub hit_radius: Option<f32>,
    pub stiffness: Option<f32>,
}

/// The shape of the collision detection for [Collider]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Component, Reflect)]
#[reflect(Component, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ColliderShape {
    Sphere(Sphere),
    Capsule(Capsule),
}

impl ColliderShape {
    /// Returns the collision vector from the collider to the target position.
    pub fn calc_collision(
        &self,
        next_tail: Vec3,
        collider: &GlobalTransform,
        joint_radius: f32,
    ) -> (Vec3, f32) {
        match self {
            Self::Sphere(sphere) => {
                let offset = collider.compute_matrix().transform_point3(Vec3::from(sphere.offset));
                let delta = next_tail - offset;
                let distance = delta.norm() - sphere.radius - joint_radius;
                (delta.normalize(), distance)
            }
            Self::Capsule(_) => {
                //TODO: In UniVRM, only SphereCollider is implemented, so it seems to be postponed
                (Vec3::ZERO, 1.)
            }
        }
    }


    #[inline]
    pub const fn radius(&self) -> f32 {
        match self {
            Self::Sphere(sphere) => sphere.radius,
            Self::Capsule(capsule) => capsule.radius,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Component, Reflect)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Sphere {
    /// Local coordinate of the sphere center
    pub offset: [f32; 3],
    /// Radius of the sphere
    pub radius: f32,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Component, Reflect)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Capsule {
    /// Local coordinate of the center of the half sphere at the start point of the capsule
    pub offset: [f32; 3],
    /// Radius of the half sphere and cylinder part of the capsule
    pub radius: f32,
    /// Local coordinate of the center of the half sphere at the end point of the capsule
    pub tail: [f32; 3],
}

#[cfg(test)]
mod tests {
    use crate::success;
    use crate::tests::TestResult;
    use crate::vrm::extensions::vrmc_spring_bone::VRMCSpringBone;

    #[test]
    fn deserialize_vrmc_spring_bone() -> TestResult {
        let _spring_bone: VRMCSpringBone = serde_json::from_str(include_str!("./vrmc_spring_bone.json"))?;
        success!()
    }
}