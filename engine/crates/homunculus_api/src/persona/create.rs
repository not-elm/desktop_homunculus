use crate::error::{ApiError, ApiResult};
use crate::persona::{PersonaApi, PersonaSnapshot};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{Gender, Persona, PersonaId, PersonaIndex, PersonaState};
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
    /// Creates a new persona entity and persists it to the database.
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
    mut commands: Commands,
    mut index: ResMut<PersonaIndex>,
    prefs: NonSend<PrefsDatabase>,
) -> ApiResult<PersonaSnapshot> {
    if index.get(&persona_id).is_some() {
        return Err(ApiError::Conflict(format!(
            "Persona already exists: {}",
            persona_id
        )));
    }

    let persona = build_persona(&persona_id, &args);
    let display_name = persona.name.clone().unwrap_or_else(|| persona_id.0.clone());
    let state = PersonaState::default();

    let entity = commands
        .spawn((
            persona.clone(),
            state.clone(),
            Name::new(display_name),
            Transform::default(),
        ))
        .id();

    index.insert(persona_id.clone(), entity);
    persist_persona(&prefs, &persona);

    Ok(PersonaSnapshot {
        persona,
        state: state.0,
        spawned: true,
    })
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

/// Persists a newly created persona to the SQLite database, logging on failure.
fn persist_persona(prefs: &PrefsDatabase, persona: &Persona) {
    if let Err(e) = prefs.insert_persona(persona) {
        warn!("Failed to persist persona: {e}");
    }
}
