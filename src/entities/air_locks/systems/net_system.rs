use bevy_ecs::{
    event::EventReader,
    system::{Query, Res, ResMut},
};
use bevy_renet::renet::RenetServer;

use crate::{
    core::{
        connected_player::{components::ConnectedPlayer, resources::HandleToEntity},
        networking::{send_net, NetEvent, RENET_RELIABLE_CHANNEL_ID},
    },
    entities::air_locks::events::NetAirLock,
};

pub fn net_system(
    mut net: ResMut<RenetServer>,
    connected_players: Query<&ConnectedPlayer>,
    handle_to_entity: Res<HandleToEntity>,

    mut net1: EventReader<NetAirLock>,
) {
    for new_event in net1.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
            RENET_RELIABLE_CHANNEL_ID,
        );
    }
}
