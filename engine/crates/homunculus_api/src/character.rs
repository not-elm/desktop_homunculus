//! Domain API for character lifecycle and state management.
//!
//! [`CharacterApi`] bridges async HTTP/MCP handlers with Bevy's single-threaded
//! ECS using the [`ApiReactor`](crate::prelude::ApiReactor) pattern.

mod attach;
mod create;
mod destroy;
mod extensions;
mod list;
mod name;
mod persona;
mod resolve;

pub use list::CharacterInfo;

use crate::api;
use bevy::prelude::*;
use bevy_vrm1::prelude::Initialized;
use homunculus_core::prelude::CharacterName;

api!(
    /// API for managing character entities.
    ///
    /// Provides CRUD operations for characters and bridges VRM attachment/detachment
    /// with persistence through the character database.
    CharacterApi
);

/// Plugin that registers character-related observers.
pub(crate) struct CharacterApiPlugin;

impl Plugin for CharacterApiPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(restore_character_name);
    }
}

/// Restores the Bevy `Name` component from `CharacterName` when a VRM finishes
/// loading (i.e. when `Initialized` is inserted).
///
/// VRM loading overwrites the `Name` component with the VRM model name.
/// This observer re-applies the character's display name.
fn restore_character_name(
    trigger: On<Insert, Initialized>,
    mut commands: Commands,
    characters: Query<&CharacterName>,
) {
    let entity = trigger.event_target();
    if let Ok(character_name) = characters.get(entity) {
        commands
            .entity(entity)
            .try_insert(Name::new(character_name.0.clone()));
    }
}
