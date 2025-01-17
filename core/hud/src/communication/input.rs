use bevy::{
    prelude::{warn, Children, EventWriter, Input, KeyCode, Query, Res},
    text::Text,
};
use chat::net::ChatClientMessage;
use console_commands::net::ClientSideConsoleInput;
use networking::client::OutgoingReliableClientMessage;
use resources::{binds::KeyBinds, hud::HudState, ui::TextInput};
use ui::text_input::{FocusTextInput, TextInputNode, UnfocusTextInput};

use crate::{
    input::binds::{SUBMIT_CONSOLE_BIND, TOGGLE_CHAT},
    inventory::build::OpenHud,
};

use super::build::HudCommunicationState;

pub(crate) fn text_input(
    keyboard: Res<Input<KeyCode>>,
    text_input_state: Res<TextInput>,
    mut text_node_query: Query<(&mut TextInputNode, &Children)>,
    mut text_node_input_query: Query<&mut Text>,
    mut net: EventWriter<OutgoingReliableClientMessage<ChatClientMessage>>,
    state: Res<HudCommunicationState>,
    mut console: EventWriter<ClientSideConsoleInput>,
    binds: Res<KeyBinds>,
) {
    match text_input_state.focused_input {
        Some(focused_input_entity) => {
            if keyboard.just_pressed(binds.bind(SUBMIT_CONSOLE_BIND)) {
                match text_node_query.get_mut(focused_input_entity) {
                    Ok((mut text_input_component, children)) => {
                        for child in children {
                            match text_node_input_query.get_mut(*child) {
                                Ok(mut text) => {
                                    let input_text = text_input_component.input.trim().to_string();
                                    if input_text.is_empty() {
                                        continue;
                                    }
                                    for section in text.sections.iter_mut() {
                                        section.value = "".to_string();
                                        text_input_component.input = "".to_string();
                                    }
                                    if state.is_displaying_console {
                                        console
                                            .send(ClientSideConsoleInput::from_string(input_text));
                                    } else {
                                        net.send(OutgoingReliableClientMessage {
                                            message: ChatClientMessage::InputChatMessage(
                                                input_text,
                                            ),
                                        });
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                    }
                    Err(_) => {
                        warn!("Could not get text input entity");
                    }
                }
            }
        }
        None => {}
    }
}

pub(crate) fn tab_communication_input_toggle(
    keys: Res<Input<KeyCode>>,
    state: Res<HudCommunicationState>,
    mut open_hud: EventWriter<OpenHud>,

    mut focus_event: EventWriter<FocusTextInput>,
    mut unfocus_event: EventWriter<UnfocusTextInput>,
    hud_state: Res<HudState>,
    binds: Res<KeyBinds>,
) {
    if keys.just_pressed(binds.bind(TOGGLE_CHAT)) {
        let is_focused = hud_state.expanded;

        if is_focused {
            unfocus_event.send(UnfocusTextInput {
                entity_option: Some(state.communication_input_node),
            });
        } else {
            focus_event.send(FocusTextInput {
                entity: state.communication_input_node,
            });
        }

        open_hud.send(OpenHud { open: !is_focused });
    }
}
