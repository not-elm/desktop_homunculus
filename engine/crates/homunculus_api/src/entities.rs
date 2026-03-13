//! Provides generics APIs for [`Entity`](bevy::prelude::Entity) in Bevy.

use crate::api;

mod find;
mod move_to;
mod name;
pub mod transform;
pub mod tween;

pub use move_to::MoveTarget;
pub use tween::{
    EasingFunction, TweenPositionArgs, TweenPositionViewportArgs, TweenRotationArgs,
    TweenScaleArgs,
};

api!(EntitiesApi);
