use bevy::prelude::{Camera3dBundle, Commands, EventReader, ResMut, Vec3};

use bevy_atmosphere::prelude::AtmosphereCamera;
use cameras::controllers::fps::{ActiveCamera, FpsCameraBundle, FpsCameraController};
use networking::client::IncomingReliableServerMessage;

use crate::net::PlayerServerMessage;
use bevy::prelude::Local;

/// Spawn 3D debug camera on boarding.

pub(crate) fn spawn_debug_camera(
    mut commands: Commands,
    mut messages: EventReader<IncomingReliableServerMessage<PlayerServerMessage>>,
    mut spawning: Local<bool>,
    mut state: ResMut<ActiveCamera>,
) {
    // Skip one frame to prevent camera ambiguity.
    if *spawning {
        *spawning = false;
        let id = commands
            .spawn(Camera3dBundle::default())
            .insert(FpsCameraBundle::new(
                FpsCameraController::default(),
                Vec3::new(0., 1.8, 0.),
                Vec3::new(0., 1.8, -2.),
                Vec3::Y,
            ))
            .insert(AtmosphereCamera::default())
            .id();
        state.option = Some(id);
    }

    for message in messages.iter() {
        match message.message {
            PlayerServerMessage::Boarded => {
                *spawning = true;
            }
            _ => {}
        }
    }
}
