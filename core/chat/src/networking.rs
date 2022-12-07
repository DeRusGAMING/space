use serde::{Deserialize, Serialize};
use typename::TypeName;

use crate::chat::NewChatMessage;
use bevy::prelude::warn;
use networking::server::HandleToEntity;

use bevy::prelude::{EventWriter, Res};

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ChatClientMessage {
    InputChatMessage(String),
}
use networking::typenames::IncomingReliableClientMessage;

use bevy::prelude::EventReader;
/// Manage incoming network messages from clients.
#[cfg(feature = "server")]
pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage<ChatClientMessage>>,
    handle_to_entity: Res<HandleToEntity>,
    mut input_chat_message_event: EventWriter<NewChatMessage>,
) {
    for message in server.iter() {
        let client_message = message.message.clone();

        match client_message {
            ChatClientMessage::InputChatMessage(i_message) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        input_chat_message_event.send(NewChatMessage {
                            messenger_entity_option: Some(*player_entity),
                            messenger_name_option: None,
                            raw_message: i_message,
                            exclusive_radio: false,
                            position_option: None,
                            send_entity_update: true,
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to SelectBodyPart sender handle.");
                    }
                }
            }
        }
    }
}
