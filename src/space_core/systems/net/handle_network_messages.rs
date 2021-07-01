use bevy::{ecs::system::{ResMut}, prelude::{EventWriter}};
use bevy_networking_turbulence::NetworkResource;

use crate::space_core::{events::general::{build_graphics::BuildGraphics, examine_entity::ExamineEntity, examine_map::ExamineMap, input_chat_message::InputChatMessage, input_sprinting::InputSprinting, movement_input::MovementInput, scene_ready::SceneReady, ui_input::UIInput, ui_input_transmit_text::UIInputTransmitText}, resources::precalculated_fov_data::Vec3Int, structs::network_messages::{ReliableClientMessage, ReliableServerMessage, UnreliableServerMessage}};

pub fn handle_network_messages(
    mut net: ResMut<NetworkResource>,
    mut ui_input_event : EventWriter<UIInput>,
    mut scene_ready_event : EventWriter<SceneReady>,
    mut ui_input_transmit_text : EventWriter<UIInputTransmitText>,
    mut movement_input_event : EventWriter<MovementInput>,
    mut build_graphics_event : EventWriter<BuildGraphics>,
    mut input_chat_message_event : EventWriter<InputChatMessage>,
    mut input_sprinting_event : EventWriter<InputSprinting>,
    mut examine_entity : EventWriter<ExamineEntity>,
    mut examine_map : EventWriter<ExamineMap>,
) {



    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();

        

        while let Some(client_message) = channels.recv::<ReliableClientMessage>() {
            //info!("ReliableClientMessage received on [{}]: {:?}",handle, client_message);

            match client_message {
                ReliableClientMessage::Awoo => {},
                ReliableClientMessage::UIInput(
                    node_class,
                    action,
                    node_name,
                    ui_type
                ) => {
                    ui_input_event.send(UIInput{
                        handle : *handle,
                        node_class: node_class,
                        action: action,
                        node_name : node_name,
                        ui_type : ui_type
                    });
                }
                ReliableClientMessage::SceneReady(scene_type) => {
                    scene_ready_event.send(SceneReady{
                        handle: *handle,
                        scene_type: scene_type
                    });
                }
                ReliableClientMessage::UIInputTransmitData(ui_type, node_path, input_text) => {
                    ui_input_transmit_text.send(UIInputTransmitText{
                        handle: *handle,
                        ui_type:ui_type,
                        node_path:node_path,
                        input_text:input_text
                    });
                }
                ReliableClientMessage::MovementInput(movement_input) => {
                    movement_input_event.send(MovementInput{
                        handle: *handle,
                        vector: movement_input
                    });
                }
                ReliableClientMessage::BuildGraphics => {
                    build_graphics_event.send(BuildGraphics{
                        handle: *handle
                    });
                }
                ReliableClientMessage::InputChatMessage(message) => {
                    input_chat_message_event.send(InputChatMessage{
                        handle: *handle,
                        message: message
                    });
                },
                ReliableClientMessage::SprintInput(is_sprinting) => {

                    input_sprinting_event.send(InputSprinting {
                        handle: *handle,
                        is_sprinting: is_sprinting,
                    });

                },
                ReliableClientMessage::ExamineEntity(entity_id) => {

                    examine_entity.send(ExamineEntity{
                        handle: *handle,
                        examine_entity_id: entity_id,
                    });

                },
                ReliableClientMessage::ExamineMap(grid_map_type, cell_id_x,cell_id_y,cell_id_z) => {

                    examine_map.send(ExamineMap{
                        handle: *handle,
                        gridmap_type: grid_map_type,
                        gridmap_cell_id: Vec3Int {
                            x: cell_id_x,
                            y: cell_id_y,
                            z: cell_id_z,
                        },
                    });

                },
            }

        }

        while let Some(_server_message) = channels.recv::<ReliableServerMessage>() {
            // In case we ever get this from faulty or malicious clients, free it up.
        }
        while let Some(_server_message) = channels.recv::<UnreliableServerMessage>() {
            // In case we ever get this from faulty or malicious clients, free it up.
        }

    }


}
