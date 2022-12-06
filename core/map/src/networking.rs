use bevy::prelude::warn;
use bevy::prelude::Vec2;

use bevy::prelude::EventWriter;
use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

use crate::map_input::InputMap;
use crate::map_input::InputMapChangeDisplayMode;
use crate::map_input::MapInput;
use bevy::prelude::Res;
use networking::server::HandleToEntity;

use crate::map_input::InputMapRequestOverlay;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum MapReliableClientMessage {
    MapChangeDisplayMode(String),
    MapRequestDisplayModes,
    MapCameraPosition(Vec2),
}

/// This message gets sent at high intervals.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum MapUnreliableClientMessage {
    MapViewRange(f32),
    MapOverlayMouseHoverCell(i16, i16),
}
use networking::typenames::get_unreliable_message;
use networking::typenames::IncomingUnreliableClientMessage;
use networking::typenames::{get_reliable_message, Typenames};

use bevy::prelude::EventReader;
use networking::typenames::IncomingReliableClientMessage;
/// Manage incoming network messages from clients.
#[cfg(feature = "server")]
pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage>,
    mut u_server: EventReader<IncomingUnreliableClientMessage>,
    mut input_map_change_display_mode: EventWriter<InputMapChangeDisplayMode>,
    handle_to_entity: Res<HandleToEntity>,
    mut input_map_request_display_modes: EventWriter<InputMapRequestOverlay>,
    mut input_map_view_range: EventWriter<InputMap>,
    typenames: Res<Typenames>,
) {
    for message in server.iter() {
        let client_message;
        match get_reliable_message::<MapReliableClientMessage>(
            &typenames,
            message.message.typename_net,
            &message.message.serialized,
        ) {
            Some(x) => {
                client_message = x;
            }
            None => {
                continue;
            }
        }

        match client_message {
            MapReliableClientMessage::MapChangeDisplayMode(display_mode) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        input_map_change_display_mode.send(InputMapChangeDisplayMode {
                            handle: message.handle,
                            entity: *player_entity,
                            display_mode,
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to MapChangeDisplayMode sender handle.");
                    }
                }
            }

            MapReliableClientMessage::MapRequestDisplayModes => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        input_map_request_display_modes.send(InputMapRequestOverlay {
                            handle: message.handle,
                            entity: *player_entity,
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to input_map_request_display_modes sender handle.");
                    }
                }
            }

            MapReliableClientMessage::MapCameraPosition(position) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        input_map_view_range.send(InputMap {
                            handle: message.handle,
                            entity: *player_entity,
                            input: MapInput::Position(position),
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to MapCameraPosition sender handle.");
                    }
                }
            }
        }

        for message in u_server.iter() {
            let client_message;
            match get_unreliable_message::<MapUnreliableClientMessage>(
                &typenames,
                message.message.typename_net,
                &message.message.serialized,
            ) {
                Some(x) => {
                    client_message = x;
                }
                None => {
                    continue;
                }
            }
            match client_message {
                MapUnreliableClientMessage::MapViewRange(range_x) => {
                    match handle_to_entity.map.get(&message.handle) {
                        Some(player_entity) => {
                            input_map_view_range.send(InputMap {
                                handle: message.handle,
                                entity: *player_entity,
                                input: MapInput::Range(range_x),
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to MapViewRange sender handle.");
                        }
                    }
                }
                MapUnreliableClientMessage::MapOverlayMouseHoverCell(idx, idy) => {
                    match handle_to_entity.map.get(&message.handle) {
                        Some(player_entity) => {
                            input_map_view_range.send(InputMap {
                                handle: message.handle,
                                entity: *player_entity,
                                input: MapInput::MouseCell(idx, idy),
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to MapMouseHoverCell sender handle.");
                        }
                    }
                }
            }
        }
    }
}

/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum MapServerMessage {
    MapSendDisplayModes(Vec<(String, String)>),
    MapOverlayUpdate(Vec<(i16, i16, i16)>),
    MapOverlayHoverData(String),
    MapDefaultAddition(i16, i16, i16),
}
