mod bone;
mod despawn;
mod expressions;
mod fetch_all;
mod find_by_name;
mod look;
mod observer;
mod persona;
mod position;
mod snapshot;
mod spawn;
mod spring_bones;
mod state;
mod vrma;
mod wait_load_by_name;

pub use crate::entities::transform::*;
pub use expressions::{ExpressionInfo, ExpressionsResponse};
pub use position::PositionResponse;
pub use snapshot::{LookAtState, VrmSnapshot};
pub use spawn::*;
pub use spring_bones::{
    SpringBoneChain, SpringBoneChainsResponse, SpringBoneProps, SpringBonePropsUpdate,
};

use crate::api;
use bevy::prelude::*;
use bevy_vrm1::prelude::Initialized;

api!(VrmApi);

pub fn initialized(In(entity): In<Entity>, vrmas: Query<&Initialized>) -> bool {
    vrmas.get(entity).is_ok()
}
