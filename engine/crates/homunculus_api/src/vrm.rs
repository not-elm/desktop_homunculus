mod bone;
pub(crate) mod expressions;
mod look;
mod position;
pub(crate) mod snapshot;
mod spring_bones;
mod vrma;

pub use crate::entities::transform::*;
pub use expressions::{ExpressionInfo, ExpressionsResponse};
pub use position::PositionResponse;
pub use snapshot::{LookAtState, VrmSnapshot};
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
