//! Persona CRUD API.
//!
//! Provides async methods for creating, reading, updating, and deleting
//! persona entities in the Bevy ECS, with persistence via [`PrefsDatabase`].

mod create;
mod delete;
mod fetch;
mod full_snapshot;
mod startup;
mod state;
mod update;
mod vrm_attach;
mod vrm_detach;

pub use create::CreatePersona;
pub use full_snapshot::PersonaFullSnapshot;
pub use update::PatchPersona;

use crate::api;
use bevy::app::{Plugin, Startup};
use homunculus_core::prelude::Persona;
use serde::{Deserialize, Serialize};

/// Persona data combined with ephemeral state, returned by all CRUD endpoints.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct PersonaSnapshot {
    #[serde(flatten)]
    pub persona: Persona,
    /// Current ephemeral state (e.g. "idle", "sitting", "drag").
    pub state: String,
}

api!(
    /// API for managing persona entities (CRUD, state, VRM attachment).
    PersonaApi
);

/// Plugin that restores persisted personas at startup.
pub struct PersonaApiPlugin;

impl Plugin for PersonaApiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, startup::load_personas);
    }
}
