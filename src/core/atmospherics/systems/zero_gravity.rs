use bevy_ecs::{
    entity::Entity,
    system::{Commands, Query, Res},
};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Friction};
use bevy_transform::prelude::Transform;

use crate::core::{
    atmospherics::components::ZeroGravity,
    gridmap::{functions::gridmap_functions::world_to_cell_id, resources::GridmapMain},
    rigid_body::components::RigidBodyData,
};

pub fn zero_gravity(
    mut rigid_bodies: Query<(
        Entity,
        &Transform,
        Option<&ZeroGravity>,
        &mut Friction,
        &RigidBodyData,
    )>,
    gridmap_main: Res<GridmapMain>,
    mut commands: Commands,
) {
    for (
        rigidbody_entity,
        rigidbody_position_component,
        zero_gravity_component_option,
        mut collider_material_component,
        rigidbody_data_component,
    ) in rigid_bodies.iter_mut()
    {
        let mut cell_id = world_to_cell_id(rigidbody_position_component.translation.into());

        cell_id.y = -1;

        match gridmap_main.grid_data.get(&cell_id) {
            Some(_) => {
                if zero_gravity_component_option.is_some() {
                    collider_material_component.coefficient = rigidbody_data_component.friction;
                    collider_material_component.combine_rule =
                        rigidbody_data_component.friction_combine_rule;
                    commands.entity(rigidbody_entity).remove::<ZeroGravity>();
                }
            }
            None => {
                if zero_gravity_component_option.is_none() {
                    collider_material_component.coefficient = 0.;
                    collider_material_component.combine_rule = CoefficientCombineRule::Min;
                    commands.entity(rigidbody_entity).insert(ZeroGravity);
                }
            }
        }
    }
}
