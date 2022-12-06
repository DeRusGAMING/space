use bevy::prelude::warn;
use networking::server::GodotVariant;
use networking::server::GodotVariantValues;
use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

use crate::commands::InputConsoleCommand;
use bevy::prelude::{EventWriter, Res};
use networking::server::HandleToEntity;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ConsoleCommandsClientMessage {
    ConsoleCommand(String, Vec<GodotVariantValues>),
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
    mut console_commands_queue: EventWriter<InputConsoleCommand>,
    typenames: Res<Typenames>,
) {
    for message in server.iter() {
        let client_message;
        match get_reliable_message::<ConsoleCommandsClientMessage>(
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
            ConsoleCommandsClientMessage::ConsoleCommand(command_name, variant_arguments) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        console_commands_queue.send(InputConsoleCommand {
                            handle_option: Some(message.handle),
                            entity: *player_entity,
                            command_name: command_name,
                            command_arguments: variant_arguments,
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to console_command sender handle.");
                    }
                }
            }
        }
    }
}
/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ConsoleCommandsServerMessage {
    ConsoleWriteLine(String),
    ConfigConsoleCommands(Vec<(String, String, Vec<(String, GodotVariant)>)>),
}
