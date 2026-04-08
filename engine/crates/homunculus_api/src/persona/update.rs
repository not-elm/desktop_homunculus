use crate::error::{ApiError, ApiResult};
use crate::persona::{PersonaApi, PersonaSnapshot};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{
    Gender, Persona, PersonaChangeEvent, PersonaId, PersonaIndex, PersonaState, VrmEvent,
    VrmEventSender,
};
use homunculus_prefs::prelude::PrefsDatabase;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Partial update payload for a persona.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct PatchPersona {
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
    /// Applies a partial update to a persona.
    pub async fn patch(
        &self,
        persona_id: PersonaId,
        patch: PatchPersona,
    ) -> ApiResult<PersonaSnapshot> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(patch_persona).with((persona_id, patch)))
                    .await
            })
            .await?
    }

    /// Updates the display name of a persona.
    pub async fn set_name(
        &self,
        persona_id: PersonaId,
        name: String,
    ) -> ApiResult<PersonaSnapshot> {
        self.patch(
            persona_id,
            PatchPersona {
                name: Some(name),
                ..Default::default()
            },
        )
        .await
    }

    /// Updates the age of a persona.
    pub async fn set_age(&self, persona_id: PersonaId, age: u32) -> ApiResult<PersonaSnapshot> {
        self.patch(
            persona_id,
            PatchPersona {
                age: Some(age),
                ..Default::default()
            },
        )
        .await
    }

    /// Updates the gender of a persona.
    pub async fn set_gender(
        &self,
        persona_id: PersonaId,
        gender: Gender,
    ) -> ApiResult<PersonaSnapshot> {
        self.patch(
            persona_id,
            PatchPersona {
                gender: Some(gender),
                ..Default::default()
            },
        )
        .await
    }

    /// Updates the first-person pronoun of a persona.
    pub async fn set_first_person_pronoun(
        &self,
        persona_id: PersonaId,
        pronoun: String,
    ) -> ApiResult<PersonaSnapshot> {
        self.patch(
            persona_id,
            PatchPersona {
                first_person_pronoun: Some(pronoun),
                ..Default::default()
            },
        )
        .await
    }

    /// Updates the profile of a persona.
    pub async fn set_profile(
        &self,
        persona_id: PersonaId,
        profile: String,
    ) -> ApiResult<PersonaSnapshot> {
        self.patch(
            persona_id,
            PatchPersona {
                profile: Some(profile),
                ..Default::default()
            },
        )
        .await
    }

    /// Updates the personality of a persona.
    pub async fn set_personality(
        &self,
        persona_id: PersonaId,
        personality: String,
    ) -> ApiResult<PersonaSnapshot> {
        self.patch(
            persona_id,
            PatchPersona {
                personality: Some(personality),
                ..Default::default()
            },
        )
        .await
    }

    /// Replaces all metadata of a persona.
    pub async fn set_metadata(
        &self,
        persona_id: PersonaId,
        metadata: HashMap<String, serde_json::Value>,
    ) -> ApiResult<PersonaSnapshot> {
        self.patch(
            persona_id,
            PatchPersona {
                metadata: Some(metadata),
                ..Default::default()
            },
        )
        .await
    }
}

fn patch_persona(
    In((persona_id, patch)): In<(PersonaId, PatchPersona)>,
    mut commands: Commands,
    index: Res<PersonaIndex>,
    mut personas: Query<(&mut Persona, &PersonaState)>,
    prefs: NonSend<PrefsDatabase>,
    tx: Option<Res<VrmEventSender<PersonaChangeEvent>>>,
) -> ApiResult<PersonaSnapshot> {
    let entity = index.get(&persona_id).ok_or(ApiError::EntityNotFound)?;

    let (mut persona, state) = personas
        .get_mut(entity)
        .map_err(|_| ApiError::EntityNotFound)?;

    apply_patch(&mut persona, &patch);

    let display_name = persona.name.clone().unwrap_or_else(|| persona.id.0.clone());
    commands.entity(entity).try_insert(Name::new(display_name));

    let updated = persona.clone();
    let state_str = state.0.clone();
    persist_and_broadcast(&prefs, &tx, entity, &updated);

    Ok(PersonaSnapshot {
        persona: updated,
        state: state_str,
    })
}

/// Merges non-`None` patch fields into the existing persona.
fn apply_patch(persona: &mut Mut<'_, Persona>, patch: &PatchPersona) {
    if let Some(name) = &patch.name {
        persona.name = Some(name.clone());
    }
    if let Some(age) = patch.age {
        persona.age = Some(age);
    }
    if let Some(gender) = &patch.gender {
        persona.gender = gender.clone();
    }
    if let Some(pronoun) = &patch.first_person_pronoun {
        persona.first_person_pronoun = Some(pronoun.clone());
    }
    if let Some(profile) = &patch.profile {
        persona.profile = profile.clone();
    }
    if let Some(personality) = &patch.personality {
        persona.personality = Some(personality.clone());
    }
    if let Some(metadata) = &patch.metadata {
        persona.metadata = metadata.clone();
    }
}

/// Saves to database and broadcasts the change event.
fn persist_and_broadcast(
    prefs: &PrefsDatabase,
    tx: &Option<Res<VrmEventSender<PersonaChangeEvent>>>,
    entity: Entity,
    persona: &Persona,
) {
    if let Err(e) = prefs.update_persona(persona) {
        warn!("Failed to persist persona: {e}");
    }
    if let Some(tx) = tx {
        let _ = tx.try_broadcast(VrmEvent {
            vrm: entity,
            payload: PersonaChangeEvent {
                persona: persona.clone(),
            },
        });
    }
}
