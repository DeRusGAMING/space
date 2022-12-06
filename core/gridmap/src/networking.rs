use bevy::prelude::warn;
use bevy::prelude::Vec3;
use networking::server::GridMapLayer;
use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

use crate::examine::InputExamineMap;
use bevy::prelude::EventWriter;
use bevy::prelude::Res;
use math::grid::Vec3Int;
use networking::server::HandleToEntity;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum GridmapClientMessage {
    ExamineMap(GridMapLayer, i16, i16, i16),
}

use bevy::prelude::EventReader;
use networking::typenames::get_reliable_message;
use networking::typenames::IncomingReliableClientMessage;
use networking::typenames::Typenames;

/// Manage incoming network messages from clients.
#[cfg(feature = "server")]
pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage>,
    handle_to_entity: Res<HandleToEntity>,
    mut input_examine_map: EventWriter<InputExamineMap>,
    typenames: Res<Typenames>,
) {
    for message in server.iter() {
        let client_message;

        match get_reliable_message::<GridmapClientMessage>(
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
            GridmapClientMessage::ExamineMap(grid_map_type, cell_id_x, cell_id_y, cell_id_z) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        input_examine_map.send(InputExamineMap {
                            handle: message.handle,
                            entity: *player_entity,
                            gridmap_type: grid_map_type,
                            gridmap_cell_id: Vec3Int {
                                x: cell_id_x,
                                y: cell_id_y,
                                z: cell_id_z,
                            },
                            ..Default::default()
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to ExamineMap sender handle.");
                    }
                }
            }
        }
    }
}
/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum GridmapServerMessage {
    RemoveCell(i16, i16, i16, GridMapLayer),
    AddCell(i16, i16, i16, i64, i64, GridMapLayer),
    FireProjectile(ProjectileData),
    ConfigBlackCellID(i64, i64),
    ConfigOrderedCellsMain(Vec<String>),
    ConfigOrderedCellsDetails1(Vec<String>),
    ConfigPlaceableItemsSurfaces(Vec<i64>),
    ConfigNonBlockingCells(Vec<i64>),
}

/// Contains information about the projectile and its visual graphics.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ProjectileData {
    Laser((f32, f32, f32, f32), f32, f32, Vec3, Vec3),
    Ballistic,
}
