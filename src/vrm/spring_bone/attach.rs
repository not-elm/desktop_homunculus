use crate::system_param::child_searcher::ChildSearcher;
use crate::vrm::spring_bone::registry::{SpringColliderRegistry, SpringJointRegistry, SpringNodeRegistry};
use crate::vrm::spring_bone::{SpringBoneJointState, SpringRoot};
use bevy::app::{App, Update};
use bevy::math::NormedVectorSpace;
use bevy::prelude::{Added, Children, Entity, ParallelCommands, Plugin, Query, Transform};

pub struct SpringBoneAttachPlugin;

impl Plugin for SpringBoneAttachPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            attach_joints,
            attach_collider_shapes,
            attach_spring_roots,
            init_spring_joint_states,
        ));
    }
}

fn attach_joints(
    par_commands: ParallelCommands,
    child_searcher: ChildSearcher,
    mascots: Query<(Entity, &SpringJointRegistry), Added<Children>>,
) {
    mascots.par_iter().for_each(|(entity, nodes)| {
        for (name, props) in nodes.iter() {
            let Some(joint_entity) = child_searcher.find_from_name(entity, name.as_str()) else {
                continue;
            };
            par_commands.command_scope(|mut commands| {
                commands.entity(joint_entity).insert(*props);
            });
        }
    });
}

fn attach_collider_shapes(
    par_commands: ParallelCommands,
    child_searcher: ChildSearcher,
    mascots: Query<(Entity, &SpringColliderRegistry), Added<Children>>,
) {
    mascots.par_iter().for_each(|(entity, nodes)| {
        for (name, shape) in nodes.iter() {
            let Some(collider_entity) = child_searcher.find_from_name(entity, name) else {
                continue;
            };
            par_commands.command_scope(|mut commands| {
                commands.entity(collider_entity).insert(*shape);
            });
        }
    });
}

fn attach_spring_roots(
    par_commands: ParallelCommands,
    child_searcher: ChildSearcher,
    mascots: Query<(Entity, &SpringNodeRegistry), Added<Children>>,
) {
    mascots.par_iter().for_each(|(entity, registry)| {
        for mut spring_root in registry.0
            .iter()
            .map(|spring| SpringRoot {
                center_node: spring.center.as_ref().and_then(|center| {
                    child_searcher.find_from_name(entity, center.as_str())
                }),
                joints: spring.joints.iter().filter_map(|joint| {
                    child_searcher.find_from_name(entity, joint.as_str())
                }).collect(),
            })
        {
            if spring_root.joints.is_empty() {
                continue;
            }
            let root_entity = spring_root.joints.remove(0);
            par_commands.command_scope(|mut commands| {
                commands.entity(root_entity).insert(spring_root);
            });
        }
    });
}

fn init_spring_joint_states(
    par_commands: ParallelCommands,
    spring_roots: Query<&SpringRoot, Added<SpringRoot>>,
    joints: Query<&Transform>,
) {
    spring_roots.par_iter().for_each(|root| {
        for joint_entity in root.joints.iter() {
            let Ok(tf) = joints.get(*joint_entity) else {
                continue;
            };
            let state = SpringBoneJointState {
                prev_tail: tf.translation,
                current_tail: tf.translation,
                bone_axis: tf.rotation.normalize(),
                bone_length: tf.translation.norm(),
                initial_local_matrix: tf.compute_matrix(),
                initial_local_rotation: tf.rotation,
            };
            par_commands.command_scope(|mut commands| {
                commands.entity(*joint_entity).insert(state);
            });
        }
    });
}

#[cfg(test)]
mod tests {
    use crate::success;
    use crate::tests::{test_app, TestResult};
    use crate::vrm::spring_bone::attach::{attach_spring_roots, init_spring_joint_states};
    use crate::vrm::spring_bone::registry::{SpringNode, SpringNodeRegistry};
    use crate::vrm::spring_bone::{SpringBoneJointState, SpringRoot};
    use bevy::core::Name;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{BuildChildren, Commands, Entity, Transform};

    #[test]
    fn test_attach_spring_root() -> TestResult {
        let mut app = test_app();
        let child: Entity = app.world_mut().run_system_once(|mut commands: Commands| {
            let child = commands.spawn(Name::new("child1")).id();
            commands.spawn(SpringNodeRegistry(vec![
                SpringNode {
                    center: None,
                    joints: vec![
                        Name::new("child1"),
                    ]
                }
            ])).add_child(child);
            child
        })?;
        app.update();

        app.world_mut().run_system_once(attach_spring_roots)?;
        app.update();

        let query = app.world_mut().query::<(Entity, &SpringRoot)>().iter(app.world_mut()).next().unwrap();
        assert_eq!(query, (child, &SpringRoot {
            center_node: None,
            joints: vec![]
        }));
        success!()
    }

    #[test]
    fn set_center_node_spring_root() -> TestResult {
        let mut app = test_app();
        let (center, child): (Entity, Entity) = app.world_mut().run_system_once(|mut commands: Commands| {
            let center = commands.spawn(Name::new("center")).id();
            let child = commands.spawn(Name::new("child1")).id();
            commands.spawn(SpringNodeRegistry(vec![
                SpringNode {
                    center: Some(Name::new("center")),
                    joints: vec![
                        Name::new("child1"),
                    ]
                }
            ]))
                .add_child(child)
                .add_child(center);
            (center, child)
        })?;
        app.update();

        app.world_mut().run_system_once(attach_spring_roots)?;
        app.update();

        let query = app.world_mut().query::<(Entity, &SpringRoot)>().iter(app.world_mut()).next().unwrap();
        assert_eq!(query, (child, &SpringRoot {
            center_node: Some(center),
            joints: vec![]
        }));
        success!()
    }

    #[test]
    fn test_init_spring_joint_state() -> TestResult {
        let mut app = test_app();
        app.world_mut().run_system_once(|mut commands: Commands| {
            commands.spawn(SpringNodeRegistry(vec![
                SpringNode {
                    center: None,
                    joints: vec![
                        Name::new("root_joint"),
                        Name::new("joint2")
                    ]
                }
            ]))
                .with_child((
                    Name::new("root_joint"),
                    Transform::from_xyz(0.0, 2.0, 0.0),
                ))
                .with_child((
                    Name::new("joint2"),
                    Transform::from_xyz(0.0, 2.0, 0.0),
                ));
        })?;
        app.update();

        app.world_mut().run_system_once(attach_spring_roots)?;
        app.update();

        app.world_mut().run_system_once(init_spring_joint_states)?;
        app.update();

        let states = app
            .world_mut()
            .query::<&SpringBoneJointState>()
            .iter(app.world_mut())
            .collect::<Vec<_>>();
        assert_eq!(states.len(), 1);
        success!()
    }
}