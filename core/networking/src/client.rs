use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::prelude::{info, Resource};
use bevy_renet::renet::{
    ChannelConfig, ClientAuthentication, ReliableChannelConfig, RenetClient, RenetConnectionConfig,
};

use crate::server::PROTOCOL_ID;

#[cfg(feature = "client")]
pub const CLIENT_PORT: u16 = 56613;

/// Resource containing needed for the server.
#[cfg(feature = "client")]
#[derive(Default, Resource)]
pub struct ConnectionPreferences {
    pub account_name: String,
    pub server_address: String,
}

/// Event that triggers a new server connection.
#[cfg(feature = "client")]
pub struct ConnectToServer;

use crate::server::SERVER_PORT;
use bevy::prelude::warn;
use bevy::prelude::Commands;
use bevy::prelude::EventReader;
use bevy::prelude::Res;
use std::net::IpAddr;

use crate::plugin::RENET_RELIABLE_CHANNEL_ID;
use crate::server::NetworkingClientServerMessage;
use bevy::prelude::ResMut;

#[cfg(feature = "client")]
pub(crate) fn connect_to_server(
    mut event: EventReader<ConnectToServer>,
    mut commands: Commands,
    preferences: Res<ConnectionPreferences>,
    mut connection_state: ResMut<Connection>,
) {
    for _ in event.iter() {
        match connection_state.status {
            ConnectionStatus::None => {
                let address;
                let port;

                match preferences.server_address.split_once(":") {
                    Some((ip, port_str)) => {
                        address = ip;
                        match port_str.parse::<u16>() {
                            Ok(p) => {
                                port = p;
                            }
                            Err(_rr) => {
                                warn!("Couldn't connect: couldn't parse port.");
                                continue;
                            }
                        };
                    }
                    None => {
                        address = &preferences.server_address;
                        port = SERVER_PORT
                    }
                }

                let ip_address;

                match address.parse::<IpAddr>() {
                    Ok(add) => {
                        ip_address = add;
                    }
                    Err(_) => {
                        warn!("Couldn't connect: invalid server address.");
                        continue;
                    }
                }

                let socket_address: SocketAddr = SocketAddr::new(ip_address, port as u16);
                let socket = UdpSocket::bind(
                    local_ipaddress::get().unwrap_or_default() + ":" + &CLIENT_PORT.to_string(),
                )
                .unwrap();

                let channels_config = vec![
                    ChannelConfig::Reliable(ReliableChannelConfig {
                        packet_budget: 6000,
                        max_message_size: 5900,
                        ..Default::default()
                    }),
                    ChannelConfig::Unreliable(Default::default()),
                    ChannelConfig::Chunk(Default::default()),
                ];

                let connection_config = RenetConnectionConfig {
                    send_channels_config: channels_config.clone(),
                    receive_channels_config: channels_config,

                    ..Default::default()
                };
                let current_time = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap();
                let client_id = current_time.as_millis() as u64;

                info!("Establishing connection with [{}]", socket_address);

                let mut client = RenetClient::new(
                    current_time,
                    socket,
                    connection_config,
                    ClientAuthentication::Unsecure {
                        protocol_id: PROTOCOL_ID,
                        client_id: client_id,
                        server_addr: socket_address,
                        user_data: None,
                    },
                )
                .unwrap();

                client.send_message(
                    RENET_RELIABLE_CHANNEL_ID,
                    bincode::serialize(&NetworkingClientServerMessage::Awoo).unwrap(),
                );

                commands.insert_resource(client);

                connection_state.status = ConnectionStatus::Connecting;
            }
            ConnectionStatus::Connecting => {
                continue;
            }
            ConnectionStatus::Connected => {
                continue;
            }
        }
    }
}

#[cfg(feature = "client")]
#[derive(Default, Resource)]
pub struct Connection {
    pub status: ConnectionStatus,
}

#[cfg(feature = "client")]
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum ConnectionStatus {
    #[default]
    None,
    Connecting,
    Connected,
}

/// Event containing in-bound reliable server messages.
pub struct InboundReliableServerMessages {
    pub message: Vec<u8>,
}
/// Event containing in-bound unreliable server messages.
pub struct InboundUnreliableServerMessages {
    pub message: Vec<u8>,
}
use crate::plugin::RENET_UNRELIABLE_CHANNEL_ID;
use bevy::prelude::EventWriter;

/// RenetClient messages get turned into events so they can be consumed by multiple systems.
#[cfg(feature = "client")]
pub(crate) fn messages_to_event(
    mut client: ResMut<RenetClient>,
    mut r_events: EventWriter<InboundReliableServerMessages>,
    mut u_events: EventWriter<InboundUnreliableServerMessages>,
) {
    while let Some(message) = client.receive_message(RENET_RELIABLE_CHANNEL_ID) {
        r_events.send(InboundReliableServerMessages { message });
    }

    while let Some(message) = client.receive_message(RENET_UNRELIABLE_CHANNEL_ID) {
        u_events.send(InboundUnreliableServerMessages { message });
    }
}

/// System run run_if with iyes_loopless
#[cfg(feature = "client")]
pub fn connected(connection: Res<Connection>) -> bool {
    matches!(connection.status, ConnectionStatus::Connected)
}
/// System run run_if with iyes_loopless. The earliest server messages (for setup_ui, boarding etc.)
/// come in while in the connecting stage.
#[cfg(feature = "client")]
pub fn connecting(connection: Res<Connection>) -> bool {
    matches!(connection.status, ConnectionStatus::Connecting)
        || matches!(connection.status, ConnectionStatus::Connected)
}
