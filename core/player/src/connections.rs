use bevy::prelude::{Commands, EventReader, EventWriter, Res, ResMut};
use networking::server::{ConnectedPlayer, HandleToEntity};
use resources::core::{ServerId, TickRate};
use serde::{Deserialize, Serialize};
use world_environment::environment::WorldEnvironment;

use crate::{
    boarding::{PersistentPlayerData, SoftPlayer},
    connection::{AuthidI, SendServerConfiguration},
    names::UsedNames,
};
#[cfg(feature = "server")]
pub(crate) fn configure(
    mut config_events: EventReader<SendServerConfiguration>,
    tick_rate: Res<TickRate>,
    server_id: Res<ServerId>,
    mut auth_id_i: ResMut<AuthidI>,
    mut used_names: ResMut<UsedNames>,
    mut commands: Commands,
    mut handle_to_entity: ResMut<HandleToEntity>,
    mut server: ResMut<RenetServer>,
) {
    use bincode::serialize;
    use networking::{plugin::RENET_RELIABLE_CHANNEL_ID, server::NetworkingClientServerMessage};

    for event in config_events.iter() {
        server.send_message(
            event.handle,
            RENET_RELIABLE_CHANNEL_ID,
            serialize(&NetworkingClientServerMessage::Awoo).unwrap(),
        );

        server.send_message(
            event.handle,
            RENET_RELIABLE_CHANNEL_ID,
            serialize(&PlayerServerMessage::ConfigTickRate(tick_rate.physics_rate)).unwrap(),
        );

        server.send_message(
            event.handle,
            RENET_RELIABLE_CHANNEL_ID,
            serialize(&PlayerServerMessage::ConfigServerEntityId(
                server_id.id.to_bits(),
            ))
            .unwrap(),
        );

        server.send_message(
            event.handle,
            RENET_RELIABLE_CHANNEL_ID,
            serialize(&PlayerServerMessage::ChangeScene(
                false,
                "setupUI".to_string(),
            ))
            .unwrap(),
        );

        server.send_message(
            event.handle,
            RENET_RELIABLE_CHANNEL_ID,
            serialize(&PlayerServerMessage::ConfigRepeatingSFX(
                "concrete_walking_footsteps".to_string(),
                (1..=39)
                    .map(|i| {
                        format!(
                        "/content/audio/footsteps/default/Concrete_Shoes_Walking_step{i}.sample"
                    )
                    })
                    .collect(),
            ))
            .unwrap(),
        );

        server.send_message(
            event.handle,
            RENET_RELIABLE_CHANNEL_ID,
            serialize(&PlayerServerMessage::ConfigRepeatingSFX(
                "concrete_sprinting_footsteps".to_string(),
                [
                    4, 5, 7, 9, 10, 12, 13, 14, 15, 16, 17, 20, 21, 22, 23, 24, 25, 27, 28, 30, 31,
                    32, 34, 35, 36, 38, 40, 41, 42, 43, 44, 45, 46, 47, 49, 50, 51,
                ]
                .iter()
                .map(|i| {
                    format!(
                        "/content/audio/footsteps/default/Concrete_Shoes_Running_step{i}.sample"
                    )
                })
                .collect(),
            ))
            .unwrap(),
        );

        // Create the actual Bevy entity for the player , with its network handle, authid and softConnected components.

        let connected_player_component = ConnectedPlayer {
            handle: event.handle,
            authid: auth_id_i.i,
            rcon: false,
            ..Default::default()
        };

        let soft_connected_component = SoftPlayer;

        let mut default_name = "Wolf".to_string() + &used_names.player_i.to_string();

        used_names.player_i += 1;

        while used_names.account_name.contains_key(&default_name) {
            used_names.player_i += 1;
            default_name = "Wolf".to_string() + &used_names.player_i.to_string();
        }

        let persistent_player_data = PersistentPlayerData {
            character_name: "".to_string(),
            account_name: default_name.clone(),
            ..Default::default()
        };

        auth_id_i.i += 1;

        let player_entity_id = commands
            .spawn((
                connected_player_component,
                soft_connected_component,
                persistent_player_data,
            ))
            .id();

        used_names
            .account_name
            .insert(default_name, player_entity_id);

        handle_to_entity.map.insert(event.handle, player_entity_id);
        handle_to_entity
            .inv_map
            .insert(player_entity_id, event.handle);

        server.send_message(
            event.handle,
            RENET_RELIABLE_CHANNEL_ID,
            serialize(&PlayerServerMessage::ConfigEntityId(
                player_entity_id.to_bits(),
            ))
            .unwrap(),
        );
    }
}

pub struct PlayerAwaitingBoarding {
    pub handle: u64,
}

#[cfg(feature = "server")]
pub(crate) fn finished_configuration(
    mut config_events: EventReader<SendServerConfiguration>,
    mut server: ResMut<RenetServer>,
    mut player_awaiting_event: EventWriter<PlayerAwaitingBoarding>,
) {
    use networking::plugin::RENET_RELIABLE_CHANNEL_ID;

    for event in config_events.iter() {
        server.send_message(
            event.handle,
            RENET_RELIABLE_CHANNEL_ID,
            bincode::serialize(&PlayerServerMessage::ConfigFinished).unwrap(),
        );
        player_awaiting_event.send(PlayerAwaitingBoarding {
            handle: event.handle,
        });
    }
}
use bevy::prelude::info;
use bevy::prelude::warn;
use bevy_renet::renet::RenetServer;
use bevy_renet::renet::ServerEvent;

#[cfg(feature = "server")]
pub(crate) fn server_events(
    mut server_events: EventReader<ServerEvent>,
    server: Res<RenetServer>,
    mut configure: EventWriter<SendServerConfiguration>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(handle, _) => {
                let client_address;

                match server.client_addr(*handle) {
                    Some(ip) => {
                        client_address = ip;
                    }
                    None => {
                        warn!("Couldn't get address from [{}]", handle);
                        continue;
                    }
                };

                info!("Incoming connection [{}] [{:?}]", handle, client_address);
                configure.send(SendServerConfiguration { handle: *handle })
            }
            ServerEvent::ClientDisconnected(handle) => {
                info!("[{}] disconnected", handle);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum PlayerServerMessage {
    ConfigWorldEnvironment(WorldEnvironment),
    ServerTime,
    ConnectedPlayers(u16),
    ConfigTickRate(u8),
    ConfigEntityId(u64),
    ChangeScene(bool, String),
    ConfigServerEntityId(u64),
    ConfigRepeatingSFX(String, Vec<String>),
    ConfigFinished,
    ConfigTalkSpaces(Vec<(String, String)>),
}
