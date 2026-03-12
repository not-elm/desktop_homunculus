//! Reaction presets for the MCP handler.

use std::collections::HashMap;

/// A reaction preset combining expressions, optional animation, and optional sound effect.
pub(crate) struct ReactionPreset {
    pub expressions: HashMap<String, f32>,
    pub vrma: Option<&'static str>,
    pub se: Option<&'static str>,
}

/// All available reaction names.
pub(crate) const REACTION_NAMES: &[&str] = &[
    "happy",
    "sad",
    "confused",
    "error",
    "success",
    "thinking",
    "surprised",
    "neutral",
];

/// Looks up a reaction preset by name.
pub(crate) fn get_preset(name: &str) -> Option<ReactionPreset> {
    match name {
        "happy" => Some(ReactionPreset {
            expressions: HashMap::from([("happy".into(), 1.0)]),
            vrma: Some("idle-happy"),
            se: Some("success"),
        }),
        "sad" => Some(ReactionPreset {
            expressions: HashMap::from([("sad".into(), 0.8)]),
            vrma: None,
            se: None,
        }),
        "confused" => Some(ReactionPreset {
            expressions: HashMap::from([("surprised".into(), 0.4)]),
            vrma: None,
            se: None,
        }),
        "error" => Some(ReactionPreset {
            expressions: HashMap::from([("angry".into(), 0.3), ("sad".into(), 0.4)]),
            vrma: None,
            se: Some("error"),
        }),
        "success" => Some(ReactionPreset {
            expressions: HashMap::from([("happy".into(), 0.9)]),
            vrma: Some("celebrate"),
            se: Some("success"),
        }),
        "thinking" => Some(ReactionPreset {
            expressions: HashMap::from([("neutral".into(), 0.5)]),
            vrma: Some("thinking"),
            se: None,
        }),
        "surprised" => Some(ReactionPreset {
            expressions: HashMap::from([("surprised".into(), 0.9)]),
            vrma: None,
            se: Some("notification"),
        }),
        "neutral" => Some(ReactionPreset {
            expressions: HashMap::new(),
            vrma: None,
            se: None,
        }),
        _ => None,
    }
}
