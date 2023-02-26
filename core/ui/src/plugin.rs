use bevy::prelude::{App, Plugin};
use networking::messaging::{register_reliable_message, MessageSender};

use crate::{
    button::button_hover_visuals,
    fonts::{init_fonts, Fonts},
    net::{UiClientMessage, UiServerMessage},
    text_input::{
        focus_events, incoming_messages, input_characters, input_mouse_press_unfocus,
        set_text_input_node_text, ui_events, FocusTextInput, SetText, TextInputLabel,
        TextTreeInputSelection, UnfocusTextInput,
    },
};
use bevy::app::CoreStage::PreUpdate;
use bevy::prelude::IntoSystemDescriptor;
use resources::{is_server::is_server, ui::TextInput};
pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system_to_stage(PreUpdate, incoming_messages)
                .add_event::<TextTreeInputSelection>();
        } else {
            app.add_system(ui_events.label(TextInputLabel::UiEvents))
                .add_system(
                    focus_events
                        .before(TextInputLabel::UiEvents)
                        .label(TextInputLabel::MousePressUnfocus),
                )
                .add_system(input_mouse_press_unfocus.before(TextInputLabel::MousePressUnfocus))
                .init_resource::<TextInput>()
                .add_system(input_characters)
                .add_event::<UnfocusTextInput>()
                .add_event::<FocusTextInput>()
                .add_system(button_hover_visuals)
                .add_event::<SetText>()
                .add_system(set_text_input_node_text);
        }
        app.init_resource::<Fonts>().add_startup_system(init_fonts);
        register_reliable_message::<UiClientMessage>(app, MessageSender::Client);
        register_reliable_message::<UiServerMessage>(app, MessageSender::Server);
    }
}
