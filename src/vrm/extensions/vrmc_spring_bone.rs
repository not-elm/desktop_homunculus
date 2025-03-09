use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VRMCSpringBone {
    #[serde(rename = "colliderGroups")]
    pub collider_groups: Vec<ColliderGroup>,
    pub colliders: Vec<Collider>,
    #[serde(rename = "specVersion")]
    pub spec_version: String,
    pub springs: Vec<Spring>,
}

#[derive(Serialize, Deserialize)]
pub struct ColliderGroup {
    pub colliders: Vec<u64>,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Collider {
    pub node: usize,
    pub shape: Shape,
}

#[derive(Serialize, Deserialize)]
pub struct Spring {
    pub center: i64,
    pub joints: Vec<SpringJoint>,
    pub name: String,
    #[serde(rename = "colliderGroups")]
    pub collider_groups: Option<Vec<i64>>,
}

/// The node of a single glTF with SpringBone settings.
#[derive(Serialize, Deserialize)]
pub struct SpringJoint {
    pub node: usize,
    #[serde(rename = "dragForce")]
    pub drag_force: f32,
    #[serde(rename = "gravityDir")]
    pub gravity_dir: [f32; 3],
    #[serde(rename = "gravityPower")]
    pub gravity_power: f32,
    #[serde(rename = "hitRadius")]
    pub hit_radius: f32,
    pub stiffness: f32,
}

/// The shape of the collision detection for [Collider]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Shape {
    Sphere(Sphere),
    Capsule(Capsule),
}

#[derive(Serialize, Deserialize)]
pub struct Sphere {
    /// Local coordinate of the sphere center
    pub offset: [f32; 3],
    /// Radius of the sphere
    pub radius: f32,
}

#[derive(Serialize, Deserialize)]
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