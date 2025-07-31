//! `/gpt` provides methods for interacting with GPT models.

mod available_models;
mod chat;
pub mod model;
pub mod speaker;
pub mod system_prompt;
pub mod use_web_search;

pub use available_models::available_models;
pub use chat::chat;
use serde::{Deserialize, Serialize};

/// Query parameters for the GPT route
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct GptQuery {
    /// If you set some VRM entity ID, it will use the VRM's specific options.
    pub vrm: Option<u64>,
}

impl GptQuery {
    pub fn vrm_entity(&self) -> Option<bevy::prelude::Entity> {
        self.vrm.map(bevy::prelude::Entity::from_bits)
    }
}

fn to_entity(entity_id: Option<u64>) -> Option<bevy::prelude::Entity> {
    entity_id.map(bevy::prelude::Entity::from_bits)
}
