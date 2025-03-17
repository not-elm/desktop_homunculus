use bevy::ecs::system::SystemParam;
use bevy::hierarchy::Children;
use bevy::math::Vec3;
use bevy::prelude::{Entity, GlobalTransform, Query};
use bevy::render::primitives::Aabb;

#[derive(SystemParam)]
pub struct MascotAabb<'w, 's> {
    meshes: Query<
        'w,
        's,
        (
            &'static GlobalTransform,
            Option<&'static Aabb>,
            Option<&'static Children>,
        ),
    >,
}

impl MascotAabb<'_, '_> {
    // pub fn calculate_as_rect(&self, entity: Entity) -> Rect {
    //     let (min, max) = self.calculate(entity);
    //     Rect::from_corners(min.truncate(), max.truncate())
    // }

    pub fn calculate(&self, mesh_root: Entity) -> (Vec3, Vec3) {
        calculate_aabb(&[mesh_root], true, &self.meshes)
    }
}

fn calculate_aabb(
    entities: &[Entity],
    include_children: bool,
    entities_query: &Query<(&GlobalTransform, Option<&Aabb>, Option<&Children>)>,
) -> (Vec3, Vec3) {
    let combine_bounds = |(a_min, a_max): (Vec3, Vec3), (b_min, b_max): (Vec3, Vec3)| {
        (a_min.min(b_min), a_max.max(b_max))
    };
    let default_bounds = (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY));
    entities
        .iter()
        .filter_map(|&entity| {
            entities_query
                .get(entity)
                .map(|(&tf, bounds, children)| {
                    let mut entity_bounds = bounds.map_or(default_bounds, |bounds| {
                        (tf * Vec3::from(bounds.min()), tf * Vec3::from(bounds.max()))
                    });
                    if include_children {
                        if let Some(children) = children {
                            let children_bounds =
                                calculate_aabb(children, include_children, entities_query);
                            entity_bounds = combine_bounds(entity_bounds, children_bounds);
                        }
                    }
                    entity_bounds
                })
                .ok()
        })
        .fold(default_bounds, combine_bounds)
}
