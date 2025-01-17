use std::fs;

use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::{
        warn, AssetServer, BuildChildren, ButtonBundle, Changed, Color, Commands, Component,
        Entity, EventWriter, Input, KeyCode, NodeBundle, Query, Res, ResMut, Resource, TextBundle,
        With,
    },
    text::{TextSection, TextStyle},
    ui::{Display, FlexDirection, Interaction, Overflow, Size, Style, Val},
};
use cargo_toml::Manifest;
use resources::{binds::KeyBinds, hud::HudState, ui::TextInput};
use ui::{
    button::SFButton,
    fonts::SOURCECODE_REGULAR_FONT,
    scrolling::ScrollingListInverted,
    text::COMMUNICATION_FONT_SIZE,
    text_input::{CharacterFilter, FocusTextInput, TextInputNode},
};

use crate::{input::binds::TOGGLE_CONSOLE_BIND, inventory::build::OpenHud};

use super::console::DisplayConsoleMessage;
#[derive(Component)]
pub struct ChatMessagesNode;
#[derive(Component)]
pub struct ChatMessagesBGNode;
#[derive(Component)]
pub struct ConsoleMessagesNode;
#[derive(Component)]
pub struct ConsoleMessagesBGNode;
#[derive(Resource)]
pub struct HudCommunicationState {
    pub chat_messages_node: Entity,
    pub console_messages_node: Entity,
    pub console_messages_bg_node: Entity,
    pub communication_input_node: Entity,
    pub communication_input_focused: bool,
    pub is_displaying_console: bool,
    pub chat_messages_bg_node: Entity,
}
#[derive(Component)]
pub struct CommunicationInputNode;
#[derive(Component)]
pub struct ToggleConsoleButton;

pub(crate) fn toggle_console_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ToggleConsoleButton>)>,
    mut state: ResMut<HudCommunicationState>,
    mut style_query: Query<&mut Style>,
    keys: Res<Input<KeyCode>>,
    mut focus_event: EventWriter<FocusTextInput>,
    mut open_hud: EventWriter<OpenHud>,
    text_input: Res<TextInput>,
    binds: Res<KeyBinds>,
) {
    if keys.just_pressed(binds.bind(TOGGLE_CONSOLE_BIND)) && text_input.focused_input.is_none() {
        state.is_displaying_console = true;
        match style_query.get_mut(state.chat_messages_bg_node) {
            Ok(mut style) => {
                style.display = Display::None;
            }
            Err(_) => {
                warn!("Couldnt find visibility component of chat messages node.");
            }
        }
        match style_query.get_mut(state.console_messages_bg_node) {
            Ok(mut style) => {
                style.display = Display::Flex;
            }
            Err(_) => {
                warn!("Couldnt find visibility component of console messages node.");
            }
        }
        focus_event.send(FocusTextInput {
            entity: state.communication_input_node,
        });
        open_hud.send(OpenHud { open: true });
    }

    for interaction in interaction_query.iter() {
        match interaction {
            Interaction::Clicked => {
                state.is_displaying_console = !state.is_displaying_console;

                match style_query.get_mut(state.chat_messages_bg_node) {
                    Ok(mut style) => {
                        if state.is_displaying_console {
                            style.display = Display::None;
                        } else {
                            style.display = Display::Flex;
                        }
                    }
                    Err(_) => {
                        warn!("Couldnt find visibility component of chat messages node.");
                    }
                }
                match style_query.get_mut(state.console_messages_bg_node) {
                    Ok(mut style) => {
                        if state.is_displaying_console {
                            style.display = Display::Flex;
                        } else {
                            style.display = Display::None;
                        }
                    }
                    Err(_) => {
                        warn!("Couldnt find visibility component of console messages node.");
                    }
                }
            }
            _ => (),
        }
    }
}

pub(crate) fn build_communication_ui(
    hud_state: Res<HudState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let sourcecode_font = asset_server.load(SOURCECODE_REGULAR_FONT);

    let mut chat_messages_node = Entity::from_bits(0);
    let mut chat_messages_bg_node = Entity::from_bits(0);

    let mut console_messages_node = Entity::from_bits(0);
    let mut communication_input_node = Entity::from_bits(0);
    let mut console_messages_bg_node = Entity::from_bits(0);
    commands
        .entity(hud_state.left_edge_node)
        .with_children(|parent| {
            chat_messages_bg_node = parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(35.)),
                        flex_direction: FlexDirection::ColumnReverse,
                        overflow: Overflow::Hidden,

                        ..Default::default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 1.0, 0.05).into(),
                    ..Default::default()
                })
                .insert(ChatMessagesBGNode)
                .with_children(|parent| {
                    chat_messages_node = parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::ColumnReverse,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            ScrollingListInverted::default(),
                            AccessibilityNode(NodeBuilder::new(Role::List)),
                        ))
                        .insert(ChatMessagesNode)
                        .id();
                })
                .id();

            console_messages_bg_node = parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(35.)),
                        flex_direction: FlexDirection::ColumnReverse,
                        display: Display::None,
                        overflow: Overflow::Hidden,
                        ..Default::default()
                    },
                    background_color: Color::rgba(0.25, 0.25, 0.25, 0.4).into(),

                    ..Default::default()
                })
                .insert(ConsoleMessagesBGNode)
                .with_children(|parent| {
                    console_messages_node = parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::ColumnReverse,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            ScrollingListInverted::default(),
                            AccessibilityNode(NodeBuilder::new(Role::List)),
                        ))
                        .insert(ConsoleMessagesNode)
                        .id();
                })
                .id();
        });

    commands
        .entity(hud_state.bottom_edge_node)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(29.18), Val::Percent(100.)),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    let text = "...".to_string();
                    let mut builder = parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Percent(50.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                    communication_input_node = builder.id();
                    builder.insert((
                        TextInputNode {
                            placeholder_active: true,
                            character_filter_option: Some(CharacterFilter::Chat),
                            placeholder_text_option: Some(text.to_owned()),
                            bg_color: Color::rgba(0.26, 0.3, 0.49, 0.5),
                            bg_color_focused: Color::rgba(0.46, 0.5, 0.79, 0.5),
                            bg_color_hover: Color::rgba(0.26, 0.3, 0.79, 0.5),
                            ..Default::default()
                        },
                        Interaction::default(),
                        CommunicationInputNode,
                    ));

                    builder.with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            text,
                            TextStyle {
                                font: sourcecode_font.clone(),
                                font_size: COMMUNICATION_FONT_SIZE,
                                color: Color::WHITE.into(),
                            },
                        ));
                    });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(3.3), Val::Percent(25.)),

                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        background_color: Color::DARK_GRAY.into(),
                                        style: Style {
                                            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    ToggleConsoleButton,
                                    SFButton::default(),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "~",
                                        TextStyle {
                                            font: sourcecode_font.clone(),
                                            font_size: 16.0,
                                            color: Color::WHITE.into(),
                                        },
                                    ));
                                });
                        });
                });
        });
    commands.insert_resource(HudCommunicationState {
        chat_messages_node,
        communication_input_node,
        communication_input_focused: false,
        console_messages_node,
        is_displaying_console: false,
        console_messages_bg_node,
        chat_messages_bg_node,
    });
}

pub const CONSOLE_FONT_COLOR: Color = Color::WHITE;

pub(crate) fn console_welcome_message(
    mut events: EventWriter<DisplayConsoleMessage>,
    asset_server: Res<AssetServer>,
) {
    let cargo_toml_contents = fs::read_to_string("core/app/Cargo.toml").unwrap();
    let cargo = Manifest::from_slice(cargo_toml_contents.as_bytes()).unwrap();

    let mut bevy_version = "".to_string();

    match cargo.dependencies.get("bevy").unwrap() {
        cargo_toml::Dependency::Simple(v) => {
            bevy_version = v.clone();
        }
        cargo_toml::Dependency::Detailed(v) => {
            bevy_version = v.version.clone().unwrap();
        }
        _ => (),
    }
    let mut sf_version = "".to_string();

    match cargo.package.unwrap().version {
        cargo_toml::Inheritable::Set(v) => {
            sf_version = v;
        }
        _ => (),
    }

    let welcome_message = format!("Space Frontiers v{}\n", sf_version)
        + &format!("Bevy v{}\n", bevy_version)
        + "Write \"help\" for a list of available commands.";

    events.send(DisplayConsoleMessage {
        sections: vec![TextSection::new(
            welcome_message,
            TextStyle {
                font: asset_server.load(SOURCECODE_REGULAR_FONT),
                font_size: COMMUNICATION_FONT_SIZE,
                color: CONSOLE_FONT_COLOR.into(),
            },
        )],
    });
}

pub const MESSAGES_DEFAULT_MAX_WIDTH: f32 = 380.;
