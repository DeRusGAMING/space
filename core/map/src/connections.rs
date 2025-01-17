use crate::map_input::MapData;
use bevy::prelude::{EventReader, Res};
use networking::server::OutgoingReliableServerMessage;
use player::connections::SendServerConfiguration;

use crate::net::MapServerMessage;
use bevy::prelude::EventWriter;

pub(crate) fn configure(
    mut config_events: EventReader<SendServerConfiguration>,
    map_data: Res<MapData>,
    mut server: EventWriter<OutgoingReliableServerMessage<MapServerMessage>>,
) {
    for event in config_events.iter() {
        for add in map_data.to_net() {
            server.send(OutgoingReliableServerMessage {
                handle: event.handle,
                message: MapServerMessage::MapDefaultAddition(add.0, add.1, add.2),
            });
        }
    }
}
