use bevy::math::Vec3;
use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

/// Constrains how parent Transform propagation affects a child entity's GlobalTransform.
///
/// Designed for webview entities that are `ChildOf` a persona entity.
/// The PostUpdate correction system reads this component and overrides the propagated
/// GlobalTransform to keep the webview upright and correctly positioned.
///
/// # Architectural constraint
///
/// This component is only valid on **leaf entities** (entities with no children).
/// Applying it to a non-leaf entity produces incorrect GlobalTransform for descendants
/// because the PostUpdate correction does not cascade.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct TransformConstraint {
    /// How much rotation to inherit from parent. 0.0 = billboard (always upright), 1.0 = full inherit.
    pub rotation_follow: f32,
    /// Maximum tilt angle from upright in degrees (swing clamp ceiling). 0.0 = no tilt allowed.
    pub max_tilt_degrees: f32,
    /// Whether to lock scale at Vec3::ONE regardless of parent scale.
    pub lock_scale: bool,
    /// User-specified offset in unscaled parent-local space.
    /// Stored separately from local Transform to prevent scale-coupling feedback loops.
    pub intended_offset: Vec3,
}

impl Default for TransformConstraint {
    fn default() -> Self {
        Self {
            rotation_follow: 0.0,
            max_tilt_degrees: 0.0,
            lock_scale: true,
            intended_offset: Vec3::ZERO,
        }
    }
}

/// API-facing constraint parameters for HTTP/MCP.
/// Maps to the non-offset fields of `TransformConstraint`.
/// `intended_offset` is set via the `transform.translation` field instead.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct WebviewConstraints {
    /// How much rotation to inherit from parent. 0.0 = billboard, 1.0 = full inherit.
    #[serde(default)]
    pub rotation_follow: Option<f32>,
    /// Maximum tilt angle from upright in degrees. 0.0 = no tilt allowed.
    #[serde(default)]
    pub max_tilt_degrees: Option<f32>,
    /// Whether to lock scale at 1.0 regardless of parent scale.
    #[serde(default)]
    pub lock_scale: Option<bool>,
}
