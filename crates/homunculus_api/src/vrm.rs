mod despawn;
mod fetch_all;
mod find_by_name;
mod look;
mod observer;
mod spawn;
mod state;
mod vrma;
mod wait_load_by_name;

pub use crate::entities::transform::*;
pub use spawn::*;

use crate::prelude::ApiReactor;
use bevy::prelude::*;
use bevy_vrm1::prelude::Initialized;

#[derive(Clone, Resource)]
pub struct VrmApi(pub(crate) ApiReactor);

impl From<ApiReactor> for VrmApi {
    fn from(reactor: ApiReactor) -> Self {
        Self(reactor)
    }
}

pub fn initialized(In(entity): In<Entity>, vrmas: Query<&Initialized>) -> bool {
    vrmas.get(entity).is_ok()
}
