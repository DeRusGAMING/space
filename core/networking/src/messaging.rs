use std::collections::HashMap;

use bevy::prelude::{App, CoreSet, IntoSystemConfig, SystemSet};
use bevy::prelude::{ResMut, Resource};
use typename::TypeName;

/// Resource containing typenames and smaller 16-bit netcode representations. Needed to identify Rust types sent over the net.

#[derive(Resource, Default)]
pub struct Typenames {
    pub reliable_incremental_id: u16,
    pub unreliable_incremental_id: u8,
    pub reliable_types: Vec<String>,
    pub unreliable_types: Vec<String>,
    pub reliable_net_types: HashMap<String, u16>,
    pub unreliable_net_types: HashMap<String, u8>,
}

use bevy::prelude::warn;

/// Generic startup system that registers reliable netcode message types. All reliable netcode types sent over the net must be registered with this system.

pub(crate) fn reliable_message<T: TypeName>(mut typenames: ResMut<Typenames>) {
    typenames.reliable_types.push(T::type_name());
}
/// Generic startup system that registers unreliable netcode message types. All unreliable netcode types sent over the net must be registered with this system.

pub(crate) fn unreliable_message<T: TypeName>(mut typenames: ResMut<Typenames>) {
    typenames.unreliable_types.push(T::type_name());
}
use bevy::prelude::info;

/// Order and generate typenames.

pub fn generate_typenames(mut typenames: ResMut<Typenames>) {
    let mut r_iter = typenames.reliable_types.clone();
    r_iter.sort();
    for typename in r_iter {
        typenames.reliable_types.push(typename.clone());
        let i = typenames.reliable_incremental_id;
        typenames.reliable_net_types.insert(typename, i);
        typenames.reliable_incremental_id += 1;

        if typenames.reliable_incremental_id >= u16::MAX {
            warn!("Reached maximum number of reliable serializable netcode messages.");
        }
    }
    let mut u_iter = typenames.unreliable_types.clone();
    u_iter.sort();
    for typename in u_iter {
        typenames.unreliable_types.push(typename.clone());
        let i = typenames.unreliable_incremental_id;
        typenames.unreliable_net_types.insert(typename, i);
        typenames.unreliable_incremental_id += 1;

        if typenames.unreliable_incremental_id >= u8::MAX {
            warn!("Reached maximum number of unreliable serializable netcode messages.");
        }
    }
    info!(
        "Loaded {} serializable messages.",
        typenames.reliable_net_types.len() + typenames.unreliable_net_types.len()
    );
}

pub enum MessageSender {
    Client,
    Server,
    Both,
}

use crate::client::is_client_connected;
use crate::{
    client::{
        deserialize_incoming_reliable_server_message, send_outgoing_reliable_client_messages,
        IncomingReliableServerMessage, OutgoingReliableClientMessage,
    },
    server::{
        deserialize_incoming_reliable_client_message, send_outgoing_reliable_server_messages,
        IncomingReliableClientMessage, OutgoingReliableServerMessage,
    },
};
/// All reliable networking messages must be registered with this system.

pub fn register_reliable_message<
    T: TypeName + Send + Sync + Serialize + for<'a> Deserialize<'a> + 'static,
>(
    app: &mut App,
    sender: MessageSender,
) {
    app.add_startup_system(reliable_message::<T>.in_set(TypenamesLabel::Generate));

    let mut client_is_sender = false;
    let mut server_is_sender = false;

    match sender {
        MessageSender::Client => {
            client_is_sender = true;
        }
        MessageSender::Server => {
            server_is_sender = true;
        }
        MessageSender::Both => {
            client_is_sender = true;
            server_is_sender = true;
        }
    }

    app.add_event::<OutgoingReliableServerMessage<T>>();
    if server_is_sender && is_server() {
        app.add_system(
            send_outgoing_reliable_server_messages::<T>.in_base_set(CoreSet::PostUpdate),
        );
    }
    app.add_event::<IncomingReliableServerMessage<T>>();
    if server_is_sender && !is_server() {
        app.add_system(
            deserialize_incoming_reliable_server_message::<T>
                .in_base_set(CoreSet::PreUpdate)
                .after(TypenamesLabel::SendRawEvents),
        );
    }
    app.add_event::<OutgoingReliableClientMessage<T>>();

    if client_is_sender && !is_server() {
        app.add_system(
            send_outgoing_reliable_client_messages::<T>
                .in_base_set(CoreSet::PostUpdate)
                .run_if(is_client_connected),
        );
    }
    app.add_event::<IncomingReliableClientMessage<T>>();

    if client_is_sender && is_server() {
        app.add_system(
            deserialize_incoming_reliable_client_message::<T>
                .in_base_set(CoreSet::PreUpdate)
                .after(TypenamesLabel::SendRawEvents),
        );
    }
}
use resources::is_server::is_server;

/// All unreliable networking messages must be registered with this system.
pub fn register_unreliable_message<
    T: TypeName + Send + Sync + Serialize + for<'a> Deserialize<'a> + 'static,
>(
    app: &mut App,
    sender: MessageSender,
) {
    use crate::{
        client::{
            deserialize_incoming_unreliable_server_message,
            send_outgoing_unreliable_client_messages, IncomingUnreliableServerMessage,
            OutgoingUnreliableClientMessage,
        },
        server::{
            deserialize_incoming_unreliable_client_message,
            send_outgoing_unreliable_server_messages, IncomingUnreliableClientMessage,
            OutgoingUnreliableServerMessage,
        },
    };

    app.add_startup_system(unreliable_message::<T>.in_set(TypenamesLabel::Generate));
    let mut client_is_sender = false;
    let mut server_is_sender = false;

    match sender {
        MessageSender::Client => {
            client_is_sender = true;
        }
        MessageSender::Server => {
            server_is_sender = true;
        }
        MessageSender::Both => {
            client_is_sender = true;
            server_is_sender = true;
        }
    }
    if server_is_sender && is_server() {
        app.add_event::<OutgoingUnreliableServerMessage<T>>()
            .add_system(
                send_outgoing_unreliable_server_messages::<T>.in_base_set(CoreSet::PostUpdate),
            );
    }
    if server_is_sender && !is_server() {
        app.add_event::<IncomingUnreliableServerMessage<T>>()
            .add_system(
                deserialize_incoming_unreliable_server_message::<T>
                    .in_base_set(CoreSet::PreUpdate)
                    .after(TypenamesLabel::SendRawEvents),
            );
    }
    if client_is_sender && !is_server() {
        app.add_event::<OutgoingUnreliableClientMessage<T>>()
            .add_system(
                send_outgoing_unreliable_client_messages::<T>
                    .in_base_set(CoreSet::PostUpdate)
                    .run_if(is_client_connected),
            );
    }
    if client_is_sender && is_server() {
        app.add_event::<IncomingUnreliableClientMessage<T>>()
            .add_system(
                deserialize_incoming_unreliable_client_message::<T>
                    .in_base_set(CoreSet::PreUpdate)
                    .after(TypenamesLabel::SendRawEvents),
            );
    }
}

/// Wrapper for reliable messages.
#[derive(Serialize, Deserialize)]

pub(crate) struct ReliableMessage {
    pub serialized: Vec<u8>,
    pub typename_net: u16,
}
/// Wrapper for unreliable messages.
#[derive(Serialize, Deserialize)]

pub(crate) struct UnreliableMessage {
    pub serialized: Vec<u8>,
    pub typename_net: u8,
}

/// Returns an option containing the desired reliable netcode message.
pub(crate) fn get_reliable_message<T: TypeName + Serialize + for<'a> Deserialize<'a>>(
    typenames: &Res<Typenames>,
    identifier: u16,
    message: &[u8],
) -> Option<T> {
    match typenames.reliable_net_types.get(&T::type_name()) {
        Some(i) => {
            if &identifier == i {
                match bincode::deserialize::<T>(message) {
                    Ok(t) => Some(t),
                    Err(_) => {
                        warn!("Couldnt serialize message!");
                        None
                    }
                }
            } else {
                None
            }
        }
        None => {
            warn!("Couldnt find reliable net type.");
            None
        }
    }
}
use bevy::prelude::Res;
use serde::{Deserialize, Serialize};

/// Typenames systems ordering label.

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum TypenamesLabel {
    Generate,
    SendRawEvents,
}
