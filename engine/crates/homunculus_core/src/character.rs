use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

/// Marker component identifying an entity as an Character.
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Character;

/// Unique, URL-safe identifier for a character.
///
/// Must match the pattern `^[a-z0-9][a-z0-9._-]{0,62}$` and must not be
/// a reserved ID.
///
/// # Example
///
/// ```
/// use homunculus_core::character::CharacterId;
///
/// let id = CharacterId::new("elmer").unwrap();
/// assert_eq!(&*id, "elmer");
/// ```
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CharacterId(String);

/// IDs reserved for internal routing — cannot be used as character identifiers.
const RESERVED_IDS: &[&str] = &["vrm", "api", "stream", "events"];

impl CharacterId {
    /// Creates a new `CharacterId` after validation.
    ///
    /// The ID must:
    /// - Start with a lowercase ASCII letter or digit
    /// - Contain only lowercase ASCII letters, digits, `.`, `_`, or `-`
    /// - Be between 1 and 63 characters long
    /// - Not be a reserved ID (`vrm`, `api`, `stream`, `events`)
    pub fn new(id: &str) -> Result<Self, InvalidCharacterId> {
        if !is_valid_character_id(id) {
            return Err(InvalidCharacterId(id.to_string()));
        }
        if RESERVED_IDS.contains(&id) {
            return Err(InvalidCharacterId(id.to_string()));
        }
        Ok(Self(id.to_string()))
    }
}

impl Deref for CharacterId {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CharacterId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Validates that `id` matches `^[a-z0-9][a-z0-9._-]{0,62}$`.
fn is_valid_character_id(id: &str) -> bool {
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

/// The display name of a character — source of truth for the `Name` component.
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Deref)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CharacterName(pub String);

/// Represents the current behavioral state of a character (e.g. "idle", "sitting", "drag").
#[repr(transparent)]
#[derive(Debug, Component, Eq, PartialEq, Clone, Reflect, Serialize, Deserialize, Deref)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[reflect(Component, Serialize, Deserialize)]
pub struct CharacterState(pub String);

impl CharacterState {
    /// The sitting state constant.
    pub const SITTING: &'static str = "sitting";
}

impl From<&str> for CharacterState {
    fn from(state: &str) -> Self {
        Self(state.to_string())
    }
}

impl Default for CharacterState {
    fn default() -> Self {
        Self("idle".to_string())
    }
}

/// Error returned when a character ID fails validation.
#[derive(Debug, thiserror::Error)]
#[error("Invalid character ID: {0}")]
pub struct InvalidCharacterId(pub String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_id_valid() {
        assert!(CharacterId::new("elmer").is_ok());
        assert!(CharacterId::new("my-character.01").is_ok());
        assert!(CharacterId::new("a").is_ok());
        assert!(CharacterId::new("0abc").is_ok());
    }

    #[test]
    fn test_character_id_uppercase_rejected() {
        assert!(CharacterId::new("Elmer").is_err());
        assert!(CharacterId::new("aBc").is_err());
    }

    #[test]
    fn test_character_id_reserved_rejected() {
        assert!(CharacterId::new("vrm").is_err());
        assert!(CharacterId::new("api").is_err());
        assert!(CharacterId::new("stream").is_err());
        assert!(CharacterId::new("events").is_err());
    }

    #[test]
    fn test_character_id_empty_rejected() {
        assert!(CharacterId::new("").is_err());
    }

    #[test]
    fn test_character_id_too_long_rejected() {
        let long_id = "a".repeat(64);
        assert!(CharacterId::new(&long_id).is_err());

        let max_id = "a".repeat(63);
        assert!(CharacterId::new(&max_id).is_ok());
    }
}
