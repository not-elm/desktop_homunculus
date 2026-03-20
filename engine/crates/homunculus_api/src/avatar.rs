//! Domain API for avatar lifecycle and state management.
//!
//! [`AvatarApi`] bridges async HTTP/MCP handlers with Bevy's single-threaded
//! ECS using the [`ApiReactor`](crate::prelude::ApiReactor) pattern.

mod attach;
mod create;
mod destroy;
mod extensions;
mod list;
mod name;
mod persona;
mod resolve;
mod state;

pub use list::AvatarInfo;

use crate::api;
use bevy::prelude::*;
use bevy_vrm1::prelude::Initialized;
use homunculus_core::prelude::AvatarName;

api!(
    /// API for managing avatar entities.
    ///
    /// Provides CRUD operations for avatars and bridges VRM attachment/detachment
    /// with persistence through the avatar database.
    AvatarApi
);

/// Plugin that registers avatar-related observers.
pub(crate) struct AvatarApiPlugin;

impl Plugin for AvatarApiPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(restore_avatar_name);
    }
}

/// Restores the Bevy `Name` component from `AvatarName` when a VRM finishes
/// loading (i.e. when `Initialized` is inserted).
///
/// VRM loading overwrites the `Name` component with the VRM model name.
/// This observer re-applies the avatar's display name.
fn restore_avatar_name(
    trigger: On<Insert, Initialized>,
    mut commands: Commands,
    avatars: Query<&AvatarName>,
) {
    let entity = trigger.event_target();
    if let Ok(avatar_name) = avatars.get(entity) {
        commands
            .entity(entity)
            .try_insert(Name::new(avatar_name.0.clone()));
    }
}
