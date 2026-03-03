pub mod asset;
pub mod mods;

pub mod prelude {
    pub use crate::schema::{asset::*, mods::*};
}
