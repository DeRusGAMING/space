use bevy::prelude::{warn, Color, EventReader, EventWriter, Query, Res};
use networking::server::{
    ConnectedPlayer, IncomingReliableClientMessage, OutgoingReliableServerMessage,
};
use player::account::Accounts;
use ui::{
    fonts::{Fonts, SOURCECODE_REGULAR_FONT},
    text::{NetTextSection, COMMUNICATION_FONT_SIZE},
};

use crate::net::{ChatClientMessage, ChatMessage, ChatServerMessage};

pub struct GlobalChatMessage {
    pub message: String,
    pub sender: u64,
}

pub(crate) fn chat_net_input(
    mut net: EventReader<IncomingReliableClientMessage<ChatClientMessage>>,
    mut events: EventWriter<GlobalChatMessage>,
) {
    for message in net.iter() {
        match &message.message {
            ChatClientMessage::InputChatMessage(input) => {
                events.send(GlobalChatMessage {
                    message: input.clone(),
                    sender: message.handle,
                });
            }
        }
    }
}

pub(crate) fn broadcast_global_chat_message(
    mut events: EventReader<GlobalChatMessage>,
    mut net: EventWriter<OutgoingReliableServerMessage<ChatServerMessage>>,
    accounts: Res<Accounts>,
    fonts: Res<Fonts>,
    connected_players: Query<&ConnectedPlayer>,
) {
    for event in events.iter() {
        let sender_name;

        match accounts.list.get(&event.sender) {
            Some(n) => {
                sender_name = n;
            }
            None => {
                warn!("Couldnt find sender account.");
                continue;
            }
        }

        let sourcecode_regular = *fonts
            .inv_map
            .get(SOURCECODE_REGULAR_FONT)
            .expect("Could not get font.");

        let sender_name_section = NetTextSection {
            text: sender_name.to_string() + ": ",
            font: sourcecode_regular,
            font_size: COMMUNICATION_FONT_SIZE,
            color: Color::BLUE,
        };

        let message_section = NetTextSection {
            text: event.message.clone(),
            font: sourcecode_regular,
            font_size: COMMUNICATION_FONT_SIZE,
            color: Color::WHITE,
        };

        for connected in connected_players.iter() {
            if connected.connected {
                net.send(OutgoingReliableServerMessage {
                    handle: connected.handle,
                    message: ChatServerMessage::ChatMessage(ChatMessage {
                        sections: vec![sender_name_section.clone(), message_section.clone()],
                    }),
                });
            }
        }
    }
}
