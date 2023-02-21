use actions::{
    net::{ActionsClientMessage, ActionsServerMessage, TabPressed},
    networking::NetAction,
};
use bevy::{
    prelude::{
        info, warn, AssetServer, BuildChildren, Button, ButtonBundle, Changed, Children, Color,
        Commands, Component, DespawnRecursiveExt, EventReader, EventWriter, NodeBundle, Query, Res,
        TextBundle, With,
    },
    text::TextStyle,
    ui::{
        AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, Size, Style, Val,
    },
};
use entity::spawn::PawnEntityId;
use inventory::server::inventory::Inventory;
use networking::client::{IncomingReliableServerMessage, OutgoingReliableClientMessage};
use player::configuration::Boarded;

use crate::hud::{HudState, LeftContentHud};

use super::build::{InventoryHudState, OpenInventoryHud};

pub const INVENTORY_HUD_BG_COLOR: Color = Color::rgba(0.1, 0.1, 0.44, 0.9);
pub const ACTIONS_HUD_BG_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 1.);

pub(crate) fn slot_item_actions(
    mut net: EventReader<IncomingReliableServerMessage<ActionsServerMessage>>,
    inventory_state: Res<InventoryHudState>,
    hud_state: Res<HudState>,
    mut commands: Commands,
    children_query: Query<&Children>,
    asset_server: Res<AssetServer>,
) {
    if !inventory_state.open || !hud_state.expanded {
        return;
    }
    for message in net.iter() {
        match &message.message {
            ActionsServerMessage::TabData(data) => {
                info!("{:?}", data);

                match children_query.get(hud_state.left_content_node) {
                    Ok(c) => {
                        for child in c.iter() {
                            commands.entity(*child).despawn_recursive();
                        }
                    }
                    Err(_) => {}
                }

                let mut builder = commands.entity(hud_state.left_content_node);

                let arizone_font = asset_server.load("fonts/ArizoneUnicaseRegular.ttf");
                let empire_font = asset_server.load("fonts/AAbsoluteEmpire.ttf");

                if data.len() == 0 {
                    continue;
                }

                let item_name = data.get(0).unwrap().item_name.clone();

                builder.with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: INVENTORY_HUD_BG_COLOR.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.), Val::Percent(3.)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,

                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        item_name,
                                        TextStyle {
                                            font_size: 13.0,
                                            color: Color::WHITE,
                                            font: arizone_font.clone(),
                                        },
                                    ));
                                });
                            parent.spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.), Val::Percent(8.)),

                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                            let actions_bg = ACTIONS_HUD_BG_COLOR;
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(65.), Val::Percent(36.)),
                                        flex_direction: FlexDirection::Column,
                                        ..Default::default()
                                    },
                                    background_color: ACTIONS_HUD_BG_COLOR.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    let mut sorted_data = data.clone();

                                    sorted_data.sort_by_key(|d| d.tab_list_priority);
                                    sorted_data.reverse();

                                    for net_action in sorted_data.iter() {
                                        parent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    size: Size::new(
                                                        Val::Percent(100.),
                                                        Val::Percent(10.),
                                                    ),
                                                    ..Default::default()
                                                },

                                                ..Default::default()
                                            })
                                            .with_children(|parent| {
                                                parent
                                                    .spawn(ButtonBundle {
                                                        style: Style {
                                                            size: Size::new(
                                                                Val::Percent(100.),
                                                                Val::Percent(100.),
                                                            ),
                                                            justify_content: JustifyContent::Center,
                                                            align_items: AlignItems::Center,
                                                            ..Default::default()
                                                        },
                                                        background_color: actions_bg.into(),

                                                        ..Default::default()
                                                    })
                                                    .insert(SlotItemActionButton {
                                                        data: net_action.clone(),
                                                    })
                                                    .with_children(|parent| {
                                                        parent.spawn(TextBundle::from_section(
                                                            net_action.text.clone(),
                                                            TextStyle {
                                                                font_size: 13.0,
                                                                color: Color::WHITE,
                                                                font: empire_font.clone(),
                                                            },
                                                        ));
                                                    });
                                            });
                                    }
                                });
                        });
                });
            }
        }
    }
}

pub(crate) fn item_actions_button_events(
    mut interaction_query: Query<
        (&Interaction, &SlotItemActionButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut net: EventWriter<OutgoingReliableClientMessage<ActionsClientMessage>>,
    state: Res<Inventory>,
    pawn: Res<PawnEntityId>,
) {
    for (interaction, component, mut bg_color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                let mut midnight = ACTIONS_HUD_BG_COLOR;
                midnight.set_a(1.);
                *bg_color = midnight.into();
                net.send(OutgoingReliableClientMessage {
                    message: ActionsClientMessage::TabPressed(TabPressed {
                        id: component.data.id.clone(),
                        action_taker: pawn.option.expect("Pawn not yet initialized."),
                        target_cell_option: None,
                        target_entity_option: state.active_item,
                        action_taker_item: None,
                    }),
                });
            }
            Interaction::Hovered => {
                let gray = Color::GRAY;

                *bg_color = gray.into();
            }
            Interaction::None => {
                let mut gray = INVENTORY_HUD_BG_COLOR;
                gray.set_a(1.);
                *bg_color = gray.into();
            }
        }
    }
}
#[derive(Component)]
pub struct SlotItemActionButton {
    pub data: NetAction,
}
pub(crate) fn hide_actions(
    boarded: Res<Boarded>,
    mut events: EventReader<OpenInventoryHud>,
    query: Query<&Children, With<LeftContentHud>>,
    hud: Res<HudState>,
    mut commands: Commands,
) {
    for event in events.iter() {
        if !boarded.boarded {
            continue;
        }
        if !event.open {
            match query.get(hud.left_content_node) {
                Ok(children) => {
                    for child in children.iter() {
                        commands.entity(*child).despawn_recursive();
                    }
                }
                Err(_) => {
                    warn!("Could not find left content node");
                }
            }
        }
    }
}
