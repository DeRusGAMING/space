use bevy::prelude::{App, CoreSet, IntoSystemConfig, Plugin};
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};
use resources::is_server::is_server;

use crate::{
    broadcast_interpolation_transforms::broadcast_interpolation_transforms,
    physics::disable_rigidbodies, rigidbody_link_transform::rigidbody_link_transform,
};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(rigidbody_link_transform)
                .add_system(broadcast_interpolation_transforms);
        }
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_system(disable_rigidbodies.in_base_set(CoreSet::PostUpdate));
    }
}
