//! Provides generics APIs for [`Entity`](bevy::prelude::Entity) in Bevy.

use crate::api;

mod find;
mod name;
pub mod transform;

api!(EntitiesApi);
