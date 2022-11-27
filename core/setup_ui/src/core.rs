use std::collections::HashMap;

use bevy::prelude::{Added, Commands, EventReader, EventWriter, Query, Res, Resource};
use networking::server::HandleToEntity;
use networking::server::{EntityUpdateData, UIInputAction};
use player::connections::PlayerAwaitingBoarding;
use resources::core::ServerId;

use controller::networking::InputUIInput;
use controller::networking::UIInputNodeClass;
use player::boarding::SoftPlayer;
use player::connection::Boarding;

/// Process player requesting board.
#[cfg(feature = "server")]
pub(crate) fn register_ui_input_boarding(
    mut event: EventReader<InputUIInput>,
    handle_to_entity: Res<HandleToEntity>,
    criteria_query: Query<&SoftPlayer>,
    mut commands: Commands,
) {
    for new_event in event.iter() {
        let player_entity = handle_to_entity.map.get(&new_event.handle).expect(
            "ui_input_event.rs could not find components for player that just got done boarding.",
        );

        // Safety check.
        match criteria_query.get(*player_entity) {
            Ok(_) => {}
            Err(_rr) => {
                continue;
            }
        }

        if new_event.ui_type == "setupUI" {
            if new_event.node_name == "board"
                && matches!(new_event.node_class, UIInputNodeClass::Button)
                && matches!(new_event.action, UIInputAction::Pressed)
            {
                commands.entity(*player_entity).insert(Boarding);
            }
        }
    }
}

/// Godot NodePath.
pub const INPUT_NAME_PATH_FULL : &str = "setupUI::ColorRect/background/VBoxContainer/HBoxContainer/characterSettingsPopup/Control/TabContainer/Boarding Configuration/VBoxContainer/vBoxNameInput/Control/inputName";
/// Godot NodePath.
pub const INPUT_NAME_PATH : &str = "ColorRect/background/VBoxContainer/HBoxContainer/characterSettingsPopup/Control/TabContainer/Boarding Configuration/VBoxContainer/vBoxNameInput/Control/inputName";

use bevy_renet::renet::RenetServer;
use motd::motd::MOTD;
use player::name_generator::get_full_name;
use player::{connection::SetupPhase, names::UsedNames};

use networking::server::ConnectedPlayer;
/// Initialize the setup UI by spawning in showcase entities etc.
#[cfg(feature = "server")]
pub(crate) fn initialize_setupui(
    used_names: Res<UsedNames>,
    server_id: Res<ServerId>,
    query: Query<&ConnectedPlayer, Added<SetupPhase>>,
    mut server: ResMut<RenetServer>,
    motd: Res<MOTD>,
) {
    use entity::networking::{EntityServerMessage, EntityWorldType};
    use networking::{plugin::RENET_RELIABLE_CHANNEL_ID, server::NetworkingChatServerMessage};

    for connected_player_component in query.iter() {
        let suggested_name = get_full_name(true, true, &used_names);

        let mut hash_map_data = HashMap::new();

        hash_map_data.insert(
            "label_text".to_string(),
            EntityUpdateData::String(suggested_name),
        );

        let mut hash_map_path = HashMap::new();

        hash_map_path.insert(INPUT_NAME_PATH_FULL.to_string(), hash_map_data);

        server.send_message(
            connected_player_component.handle,
            RENET_RELIABLE_CHANNEL_ID,
            bincode::serialize(&EntityServerMessage::EntityUpdate(
                server_id.id.to_bits(),
                hash_map_path,
                false,
                EntityWorldType::Main,
            ))
            .unwrap(),
        );

        server.send_message(
            connected_player_component.handle,
            RENET_RELIABLE_CHANNEL_ID,
            bincode::serialize(&NetworkingChatServerMessage::ChatMessage(
                motd.message.clone(),
            ))
            .unwrap(),
        );
    }
}
use bevy::prelude::warn;
use player::boarding::{BoardingPlayer, InputUIInputTransmitText, PersistentPlayerData};
use text_api::core::CONSOLE_ERROR_COLOR;

use bevy::prelude::ResMut;
use text_api::core::escape_bb;
/// Recieve boarding UI input.
#[cfg(feature = "server")]
pub(crate) fn ui_input_boarding(
    mut event: EventReader<InputUIInputTransmitText>,
    mut boarding_player_event: EventWriter<BoardingPlayer>,
    handle_to_entity: Res<HandleToEntity>,
    used_names: ResMut<UsedNames>,
    mut query: Query<(&mut PersistentPlayerData, &Boarding, &ConnectedPlayer)>,
    mut commands: Commands,
    mut server: ResMut<RenetServer>,
) {
    use console_commands::networking::ConsoleCommandsServerMessage;
    use networking::plugin::RENET_RELIABLE_CHANNEL_ID;

    for new_event in event.iter() {
        let player_entity = handle_to_entity.map.get(&new_event.handle).expect(
            "ui_input_transmit_text_event.rs could not find entity belonging to player handle.",
        );

        let player_components;

        match query.get_mut(*player_entity) {
            Ok(s) => {
                player_components = s;
            }
            Err(_rr) => {
                warn!("ui_input_transmit_text_event.rs could not find components belonging to player.");
                continue;
            }
        }

        let mut persistent_player_data = player_components.0;
        let connected_player_component = player_components.2;

        if new_event.ui_type == "setupUI" {
            if new_event.node_path == INPUT_NAME_PATH {
                // In the future check if we have recieved all requested data sets and THEN remove Boarding component.

                persistent_player_data.character_name =
                    escape_bb(new_event.input_text.to_string(), true, true);

                if persistent_player_data.character_name.len() > 26 {
                    persistent_player_data.character_name =
                        persistent_player_data.character_name[..26].to_string();
                }

                let mut name_in_use = false;

                for name in used_names.names.keys() {
                    if name.to_lowercase() == persistent_player_data.character_name.to_lowercase() {
                        // Character name of player is already in-use.
                        name_in_use = true;
                        break;
                    }
                }

                if name_in_use {
                    // Character name of player is already in-use.
                    server.send_message(
                        new_event.handle,
                        RENET_RELIABLE_CHANNEL_ID,
                        bincode::serialize(&ConsoleCommandsServerMessage::ConsoleWriteLine(
                            "[color=".to_string()
                                + CONSOLE_ERROR_COLOR
                                + "]Character name is already in-use.[/color]",
                        ))
                        .unwrap(),
                    );
                    continue;
                }

                if persistent_player_data.character_name.len() < 3 {
                    server.send_message(
                        new_event.handle,
                        RENET_RELIABLE_CHANNEL_ID,
                        bincode::serialize(&ConsoleCommandsServerMessage::ConsoleWriteLine(
                            "[color=".to_string()
                                + CONSOLE_ERROR_COLOR
                                + "]Character name is too short.[/color]",
                        ))
                        .unwrap(),
                    );
                    continue;
                }

                commands.entity(*player_entity).remove::<Boarding>();

                boarding_player_event.send(BoardingPlayer {
                    entity: *player_entity,
                    player_handle: connected_player_component.handle,
                    player_character_name: persistent_player_data.character_name.clone(),
                });
            }
        }
    }
}

/// Sets radio channel list for clients in setup UI to only show global chat availability as a function.
#[cfg(feature = "server")]
pub fn get_talk_spaces_setupui() -> Vec<(String, String)> {
    use text_api::core::TALK_SPACE_GLOBAL_CHATPREFIX;

    vec![(
        "Global".to_string(),
        TALK_SPACE_GLOBAL_CHATPREFIX.to_string(),
    )]
}

use player::connection::SendServerConfiguration;

#[cfg(feature = "server")]
pub(crate) fn configure(
    mut config_events: EventReader<SendServerConfiguration>,
    mut server: ResMut<RenetServer>,
) {
    use networking::plugin::RENET_RELIABLE_CHANNEL_ID;
    use player::connections::PlayerServerMessage;

    for event in config_events.iter() {
        let talk_spaces = get_talk_spaces_setupui();

        server.send_message(
            event.handle,
            RENET_RELIABLE_CHANNEL_ID,
            bincode::serialize(&PlayerServerMessage::ConfigTalkSpaces(talk_spaces)).unwrap(),
        );
    }
}

/// Setup ui state resource.
#[cfg(feature = "server")]
#[derive(Resource, Default)]
pub struct SetupUiState {
    pub enabled: HashMap<u64, bool>,
}

/// Show setup_ui to newly connected clients.
#[cfg(feature = "server")]
pub(crate) fn new_clients_enable_setupui(
    mut player_awaiting_boarding: EventReader<PlayerAwaitingBoarding>,
    mut state: ResMut<SetupUiState>,
    //mut net : EventWriter<NetConfigure>,
) {
    for awaiting in player_awaiting_boarding.iter() {
        if !state.enabled.contains_key(&awaiting.handle) {
            state.enabled.insert(awaiting.handle, true);
        }
    }
}
