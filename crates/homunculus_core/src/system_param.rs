pub mod bone_offsets;
mod camera_2ds;
pub mod coordinate;
pub mod mascot_tracker;
pub mod mesh_aabb;
pub mod monitors;
pub mod windows;

pub mod prelude {
    pub use crate::system_param::{
        bone_offsets::BoneOffsets, coordinate::Coordinate, mascot_tracker::MascotTracker,
        mesh_aabb::VrmAabb, monitors::Monitors, windows::*,
    };
}
