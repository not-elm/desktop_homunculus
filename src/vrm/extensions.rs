pub mod vrmc_spring_bone;
pub mod vrmc_vrm;

use crate::error::AppResult;
use crate::vrm::extensions::vrmc_spring_bone::VRMCSpringBone;
use crate::vrm::extensions::vrmc_vrm::VrmcVrm;
use anyhow::Context;
use bevy::gltf::Gltf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VrmExtensions {
    #[serde(rename = "VRMC_vrm")]
    pub vrmc_vrm: VrmcVrm,

    #[serde(rename = "VRMC_springBone")]
    pub vrmc_spring_bone: Option<VRMCSpringBone>,
}

impl VrmExtensions {
    pub fn new(
        json: &serde_json::map::Map<String, serde_json::Value>,
    ) -> AppResult<Self> {
        Ok(Self {
            vrmc_vrm: serde_json::from_value(obtain_vrmc_vrm(json)?)?,
            vrmc_spring_bone: obtain_vrmc_springs(json)
                .ok()
                .and_then(|v| serde_json::from_value(v).ok()),
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

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct VrmNode {
    pub node: usize,
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

pub fn obtain_vrmc_springs(json: &serde_json::map::Map<String, serde_json::Value>) -> AppResult<serde_json::Value> {
    Ok(json
        .get("VRMC_springBone")
        .context("Not found VRMC_springBone")?
        .clone())
}

