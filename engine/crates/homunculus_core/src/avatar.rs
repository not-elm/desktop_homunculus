use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

/// Marker component identifying an entity as an Avatar.
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Avatar;

/// Unique, URL-safe identifier for an avatar.
///
/// Must match the pattern `^[a-z0-9][a-z0-9._-]{0,62}$` and must not be
/// a reserved ID.
///
/// # Example
///
/// ```
/// use homunculus_core::avatar::AvatarId;
///
/// let id = AvatarId::new("elmer").unwrap();
/// assert_eq!(&*id, "elmer");
/// ```
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[reflect(Component, Serialize, Deserialize)]
pub struct AvatarId(String);

/// IDs reserved for internal routing — cannot be used as avatar identifiers.
const RESERVED_IDS: &[&str] = &["vrm", "api", "stream", "events"];

impl AvatarId {
    /// Creates a new `AvatarId` after validation.
    ///
    /// The ID must:
    /// - Start with a lowercase ASCII letter or digit
    /// - Contain only lowercase ASCII letters, digits, `.`, `_`, or `-`
    /// - Be between 1 and 63 characters long
    /// - Not be a reserved ID (`vrm`, `api`, `stream`, `events`)
    pub fn new(id: &str) -> Result<Self, InvalidAvatarId> {
        if !is_valid_avatar_id(id) {
            return Err(InvalidAvatarId(id.to_string()));
        }
        if RESERVED_IDS.contains(&id) {
            return Err(InvalidAvatarId(id.to_string()));
        }
        Ok(Self(id.to_string()))
    }
}

impl Deref for AvatarId {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AvatarId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Validates that `id` matches `^[a-z0-9][a-z0-9._-]{0,62}$`.
fn is_valid_avatar_id(id: &str) -> bool {
    if id.is_empty() || id.len() > 63 {
        return false;
    }
    let bytes = id.as_bytes();
    if !bytes[0].is_ascii_lowercase() && !bytes[0].is_ascii_digit() {
        return false;
    }
    bytes[1..]
        .iter()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'.' | b'_' | b'-'))
}

/// The display name of an avatar — source of truth for the `Name` component.
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Deref)]
#[reflect(Component, Serialize, Deserialize)]
pub struct AvatarName(pub String);

/// Represents the current behavioral state of an avatar (e.g. "idle", "sitting", "drag").
#[repr(transparent)]
#[derive(Debug, Component, Eq, PartialEq, Clone, Reflect, Serialize, Deserialize, Deref)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[reflect(Component, Serialize, Deserialize)]
pub struct AvatarState(pub String);

impl AvatarState {
    /// The sitting state constant.
    pub const SITTING: &'static str = "sitting";
}

impl From<&str> for AvatarState {
    fn from(state: &str) -> Self {
        Self(state.to_string())
    }
}

impl Default for AvatarState {
    fn default() -> Self {
        Self("idle".to_string())
    }
}

/// Error returned when an avatar ID fails validation.
#[derive(Debug, thiserror::Error)]
#[error("Invalid avatar ID: {0}")]
pub struct InvalidAvatarId(pub String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avatar_id_valid() {
        assert!(AvatarId::new("elmer").is_ok());
        assert!(AvatarId::new("my-avatar.01").is_ok());
        assert!(AvatarId::new("a").is_ok());
        assert!(AvatarId::new("0abc").is_ok());
    }

    #[test]
    fn test_avatar_id_uppercase_rejected() {
        assert!(AvatarId::new("Elmer").is_err());
        assert!(AvatarId::new("aBc").is_err());
    }

    #[test]
    fn test_avatar_id_reserved_rejected() {
        assert!(AvatarId::new("vrm").is_err());
        assert!(AvatarId::new("api").is_err());
        assert!(AvatarId::new("stream").is_err());
        assert!(AvatarId::new("events").is_err());
    }

    #[test]
    fn test_avatar_id_empty_rejected() {
        assert!(AvatarId::new("").is_err());
    }

    #[test]
    fn test_avatar_id_too_long_rejected() {
        let long_id = "a".repeat(64);
        assert!(AvatarId::new(&long_id).is_err());

        let max_id = "a".repeat(63);
        assert!(AvatarId::new(&max_id).is_ok());
    }
}
