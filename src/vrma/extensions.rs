use crate::error::AppResult;
use crate::vrm::extensions::{obtain_extensions, obtain_vrmc_vrm, VrmNode};
use bevy::gltf::Gltf;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VrmaExpressions {
    pub preset: HashMap<String, VrmNode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VrmaHumanoid {
    #[serde(rename = "humanBones")]
    pub human_bones: HashMap<String, VrmNode>,
}

#[derive(Serialize, Deserialize)]
pub struct VRMCVrmAnimation {
    pub expressions: Option<VrmaExpressions>,
    pub humanoid: VrmaHumanoid,
    #[serde(rename = "specVersion")]
    pub spec_version: String,
}

#[derive(Serialize, Deserialize)]
pub struct VrmaExtensions {
    #[serde(rename = "VRMC_vrm_animation")]
    pub vrmc_vrm_animation: VRMCVrmAnimation,
}

impl VrmaExtensions {
    pub fn new(
        json: &serde_json::map::Map<String, serde_json::Value>,
    ) -> AppResult<Self> {
        Ok(Self {
            vrmc_vrm_animation: serde_json::from_value(obtain_vrmc_vrm(json)?)?,
        })
    }

    pub fn from_gltf(gltf: &Gltf) -> AppResult<Self> {
        Self::new(obtain_extensions(gltf)?)
    }
}