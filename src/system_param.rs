use bevy::math::Vec2;
use bevy::prelude::Deref;

pub mod monitors;
pub mod mesh_aabb;
pub mod mascot_tracker;
pub mod bone_offsets;
pub mod coordinate;
pub mod windows;

/// Represents the global screen coordinates.
/// If there are multiple screens, the coordinates of the leftmost screen are used as the origin.
#[derive(Debug, Copy, Clone, PartialEq, Deref)]
pub struct GlobalScreenPos(pub Vec2);