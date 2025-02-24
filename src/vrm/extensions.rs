use crate::error::AppResult;
use anyhow::Context;
use bevy::gltf::Gltf;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "allowAntisocialOrHateUsage")]
    pub allow_antisocial_or_hate_usage: bool,
    #[serde(rename = "allowExcessivelySexualUsage")]
    pub allow_excessively_sexual_usage: bool,
    #[serde(rename = "allowExcessivelyViolentUsage")]
    pub allow_excessively_violent_usage: bool,
    #[serde(rename = "allowPoliticalOrReligiousUsage")]
    pub allow_political_or_religious_usage: bool,
    #[serde(rename = "allowRedistribution")]
    pub allow_redistribution: bool,
    pub authors: Vec<String>,
    #[serde(rename = "avatarPermission")]
    pub avatar_permission: Option<String>,
    #[serde(rename = "commercialUsage")]
    pub commercial_usage: Option<String>,
    #[serde(rename = "creditNotation")]
    pub credit_notation: Option<String>,
    #[serde(rename = "licenseUrl")]
    pub license_url: Option<String>,
    pub modification: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "otherLicenseUrl")]
    pub other_license_url: Option<String>,
    #[serde(rename = "thumbnailImage")]
    pub thumbnail_image: Option<i64>,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Struct8 {
    #[serde(rename = "inputMaxValue")]
    pub input_max_value: i64,
    #[serde(rename = "outputScale")]
    pub output_scale: i64,
}

// #[derive(Serialize, Deserialize)]
// struct LookAt {
//     #[serde(rename = "offsetFromHeadBone")]
//     pub offset_from_head_bone: Vec<_>,
//     #[serde(rename = "rangeMapHorizontalInner")]
//     pub range_map_horizontal_inner: Struct8,
//     #[serde(rename = "rangeMapHorizontalOuter")]
//     pub range_map_horizontal_outer: Struct8,
//     #[serde(rename = "rangeMapVerticalDown")]
//     pub range_map_vertical_down: Struct8,
//     #[serde(rename = "rangeMapVerticalUp")]
//     pub range_map_vertical_up: Struct8,
//     #[serde(rename = "type")]
//     pub r#type: String,
// }

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct VrmNode {
    pub node: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Humanoid {
    #[serde(rename = "humanBones")]
    pub human_bones: HashMap<String, VrmNode>,
}

#[derive(Serialize, Deserialize)]
pub struct Struct6 {
    pub node: i64,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize)]
pub struct FirstPerson {
    #[serde(rename = "meshAnnotations")]
    pub mesh_annotations: Vec<Struct6>,
}

#[derive(Serialize, Deserialize)]
pub struct Struct5 {
    #[serde(rename = "isBinary")]
    pub is_binary: bool,
    #[serde(rename = "overrideBlink")]
    pub override_blink: String,
    #[serde(rename = "overrideLookAt")]
    pub override_look_at: String,
    #[serde(rename = "overrideMouth")]
    pub override_mouth: String,
}

#[derive(Serialize, Deserialize)]
pub struct MorphTargetBind {
    pub index: usize,
    pub node: usize,
    pub weight: f32,
}

#[derive(Serialize, Deserialize)]
pub struct VrmPreset {
    #[serde(rename = "isBinary")]
    pub is_binary: bool,
    #[serde(rename = "morphTargetBinds")]
    pub morph_target_binds: Option<Vec<MorphTargetBind>>,
    #[serde(rename = "overrideBlink")]
    pub override_blink: String,
    #[serde(rename = "overrideLookAt")]
    pub override_look_at: String,
    #[serde(rename = "overrideMouth")]
    pub override_mouth: String,
}
#[derive(Serialize, Deserialize)]
pub struct Expressions {
    pub preset: HashMap<String, VrmPreset>,
}

#[derive(Serialize, Deserialize)]
pub struct VrmcVrm {
    pub expressions: Option<Expressions>,
    // #[serde(rename = "firstPerson")]
    // pub first_person: Option<FirstPerson>,
    pub humanoid: Humanoid,
    // #[serde(rename = "lookAt")]
    // pub look_at: LookAt,
    pub meta: Option<Meta>,
    #[serde(rename = "specVersion")]
    pub spec_version: String,
}

// #[derive(Serialize, Deserialize)]
// struct Struct2 {
//     pub center: i64,
//     pub joints: _,
//     pub name: String,
//     #[serde(rename = "colliderGroups")]
//     pub collider_groups: Option<Vec<i64>>,
// }

#[derive(Serialize, Deserialize)]
pub struct Sphere {
    pub offset: Vec<f64>,
    pub radius: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Shape {
    pub sphere: Sphere,
}

#[derive(Serialize, Deserialize)]
pub struct Struct1 {
    pub node: i64,
    pub shape: Shape,
}

#[derive(Serialize, Deserialize)]
pub struct Struct {
    pub colliders: Vec<i64>,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct VrmExtensions {
    #[serde(rename = "VRMC_vrm")]
    pub vrmc_vrm: VrmcVrm,
}

impl VrmExtensions {
    pub fn new(
        json: &serde_json::map::Map<String, serde_json::Value>,
    ) -> AppResult<Self> {
        Ok(Self {
            vrmc_vrm: serde_json::from_value(obtain_vrmc_vrm(json)?)?,
        })
    }

    pub fn from_gltf(gltf: &Gltf) -> AppResult<Self> {
        Self::new(obtain_extensions(gltf)?)
    }

    pub fn name(&self) -> Option<String> {
        self
            .vrmc_vrm
            .meta
            .as_ref()?
            .name
            .clone()
    }
}

pub fn obtain_extensions(gltf: &Gltf) -> AppResult<&serde_json::map::Map<String, serde_json::Value>> {
    gltf
        .source
        .as_ref()
        .and_then(|source| source.extensions())
        .context("Not found gltf extensions")
}

pub fn obtain_vrmc_vrm(json: &serde_json::map::Map<String, serde_json::Value>) -> AppResult<serde_json::Value> {
    Ok(json
        .get("VRMC_vrm")
        .or_else(|| json.get("VRMC_vrm_animation"))
        .context("Not found VRMC_vrm or VRMC_vrm_animation")?
        .clone())
}