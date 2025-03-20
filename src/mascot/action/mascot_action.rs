use crate::mascot::action::animation::{AnimationActionParams, AnimationActionPlugin};
use crate::mascot::action::transition::{TransitionActionParams, TransitionActionPlugin};
use crate::mascot::action::wait_animation::WaitAnimationPlugin;
use crate::settings::preferences::action::ActionName;
use bevy_vrma::vrma::VrmaPath;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct MascotAction {
    pub id: String,
    pub params: String,
}

impl MascotAction {
    pub fn new<P: Serialize>(
        id: impl Into<String>,
        params: P,
    ) -> MascotAction {
        MascotAction {
            id: id.into(),
            params: serde_json::to_string(&params).unwrap(),
        }
    }

    pub fn animation(
        vrma_name: &str,
        repeat: bool,
    ) -> Self {
        Self::new(
            AnimationActionPlugin::ID,
            AnimationActionParams {
                vrma_path: VrmaPath(PathBuf::from("vrma").join(vrma_name)),
                repeat,
            },
        )
    }

    pub fn wait_animation() -> Self {
        Self::new(WaitAnimationPlugin::ID, ())
    }

    pub fn transition(next: ActionName) -> Self {
        Self::new(TransitionActionPlugin::ID, TransitionActionParams { next })
    }
}

impl Default for MascotAction {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            params: serde_json::to_string(&()).unwrap(),
        }
    }
}
