use crate::vrm::extensions::VrmNode;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct Expressions {
    pub preset: HashMap<String, VrmPreset>,
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
pub struct MorphTargetBind {
    pub index: usize,
    pub node: usize,
    pub weight: f32,
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