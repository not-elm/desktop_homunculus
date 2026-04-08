use crate::error::{ApiError, ApiResult};
use crate::persona::{PersonaApi, PersonaSnapshot};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{Gender, Persona, PersonaId};
use homunculus_prefs::prelude::PrefsDatabase;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Arguments for creating a new persona.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct CreatePersona {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub age: Option<u32>,
    #[serde(default)]
    pub gender: Option<Gender>,
    #[serde(default)]
    pub first_person_pronoun: Option<String>,
    #[serde(default)]
    pub profile: Option<String>,
    #[serde(default)]
    pub personality: Option<String>,
    #[serde(default)]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<std::collections::HashMap<String, Object>>))]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl PersonaApi {
    /// Creates a new persona as a DB record only.
    ///
    /// The persona is **not** spawned into the ECS world. Use
    /// `POST /personas/{id}/spawn` to bring it into the scene.
    pub async fn create(&self, args: CreatePersona) -> ApiResult<PersonaSnapshot> {
        let persona_id = PersonaId::validate(&args.id).map_err(ApiError::InvalidInput)?;

        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(create).with((persona_id, args)))
                    .await
            })
            .await?
    }
}

fn create(
    In((persona_id, args)): In<(PersonaId, CreatePersona)>,
    prefs: NonSend<PrefsDatabase>,
) -> ApiResult<PersonaSnapshot> {
    reject_if_exists_in_db(&prefs, &persona_id)?;

    let persona = build_persona(&persona_id, &args);
    persist_persona(&prefs, &persona)?;

    Ok(PersonaSnapshot {
        persona,
        state: String::new(),
        spawned: false,
    })
}

/// Returns `Err(409 Conflict)` if a persona with the given ID already exists in the database.
fn reject_if_exists_in_db(prefs: &PrefsDatabase, persona_id: &PersonaId) -> ApiResult<()> {
    let exists = prefs
        .load_persona(&persona_id.0)
        .map_err(|e| ApiError::Sql(e.to_string()))?
        .is_some();

    if exists {
        return Err(ApiError::Conflict(format!(
            "Persona already exists: {}",
            persona_id
        )));
    }
    Ok(())
}

/// Builds a [`Persona`] component from the validated ID and creation arguments.
fn build_persona(persona_id: &PersonaId, args: &CreatePersona) -> Persona {
    Persona {
        id: persona_id.clone(),
        name: args.name.clone(),
        age: args.age,
        gender: args.gender.clone().unwrap_or_default(),
        first_person_pronoun: args.first_person_pronoun.clone(),
        profile: args.profile.clone().unwrap_or_default(),
        personality: args.personality.clone(),
        vrm_asset_id: None,
        metadata: args.metadata.clone().unwrap_or_default(),
    }
}

/// Inserts a newly created persona into the SQLite database.
fn persist_persona(prefs: &PrefsDatabase, persona: &Persona) -> ApiResult<()> {
    prefs
        .insert_persona(persona)
        .map_err(|e| ApiError::Sql(e.to_string()))
}
