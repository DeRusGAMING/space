use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::{
        AssetServer, BuildChildren, Commands, EventReader, EventWriter, Label, NodeBundle, Res,
        TextBundle,
    },
    text::{TextSection, TextStyle},
    ui::{FlexDirection, Size, Style, Val},
};
use chat::net::ChatServerMessage;
use networking::client::IncomingReliableServerMessage;
use ui::fonts::Fonts;

use super::build::{HudCommunicationState, MESSAGES_DEFAULT_MAX_WIDTH};

pub(crate) fn receive_chat_message(
    mut net: EventReader<IncomingReliableServerMessage<ChatServerMessage>>,
    fonts: Res<Fonts>,
    asset_server: Res<AssetServer>,
    mut events: EventWriter<DisplayChatMessage>,
) {
    for message in net.iter() {
        match &message.message {
            ChatServerMessage::ChatMessage(message) => {
                let mut sections = vec![];

                for net_section in message.sections.iter() {
                    sections.push(TextSection::new(
                        net_section.text.clone(),
                        TextStyle {
                            font: asset_server
                                .load(fonts.map.get(&net_section.font).expect("Font not loaded")),
                            font_size: net_section.font_size,
                            color: net_section.color,
                        },
                    ));
                }
                events.send(DisplayChatMessage { sections });
            }
        }
    }
}

pub struct DisplayChatMessage {
    pub sections: Vec<TextSection>,
}

pub(crate) fn display_chat_message(
    mut events: EventReader<DisplayChatMessage>,
    mut commands: Commands,
    chat_state: Res<HudCommunicationState>,
) {
    for event in events.iter() {
        let text_section = commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    flex_direction: FlexDirection::Row,

                    ..Default::default()
                },
                ..Default::default()
            })
            .insert((Label, AccessibilityNode(NodeBuilder::new(Role::ListItem))))
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_sections(event.sections.clone()).with_style(Style {
                        max_size: Size {
                            width: Val::Px(MESSAGES_DEFAULT_MAX_WIDTH),
                            height: Val::Undefined,
                        },
                        ..Default::default()
                    }),
                );
            })
            .id();

        commands
            .entity(chat_state.chat_messages_node)
            .insert_children(0, &[text_section]);
    }
}
