use bevy::{
    prelude::{
        AssetServer, BuildChildren, ButtonBundle, Color, Commands, Component, Entity, NodeBundle,
        Res, Resource, TextBundle,
    },
    text::TextStyle,
    ui::{AlignItems, Display, FlexDirection, FlexWrap, JustifyContent, Size, Style, Val},
};
use ui::{button::ButtonVisuals, fonts::ARIZONE_FONT};

#[derive(Component)]
pub struct EscapeMenuRoot;

#[derive(Resource)]
pub struct EscapeMenuState {
    pub root: Entity,
    pub visible: bool,
}
#[derive(Component)]
pub struct EscapeMenuControlsHeaderButton;
#[derive(Component)]
pub struct EscapeMenuGeneralHeaderButton;
#[derive(Component)]
pub struct EscapeMenuGraphicsHeaderButton;

pub(crate) fn build_escape_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sourcecode_font = asset_server.load(ARIZONE_FONT);
    let escape_root = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                display: Display::None,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EscapeMenuRoot)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(50.0), Val::Percent(60.0)),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    background_color: Color::rgba(0., 0.27, 0.6, 1.).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(7.)),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_wrap: FlexWrap::Wrap,
                                ..Default::default()
                            },
                            background_color: Color::DARK_GRAY.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(33.), Val::Percent(100.)),
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
                                                flex_direction: FlexDirection::Row,
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                flex_wrap: FlexWrap::Wrap,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(ButtonVisuals::default())
                                        .insert(EscapeMenuControlsHeaderButton)
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "Controls".to_string(),
                                                TextStyle {
                                                    font: sourcecode_font.clone(),
                                                    font_size: 12.0,
                                                    color: Color::WHITE.into(),
                                                },
                                            ));
                                        });
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(33.), Val::Percent(100.)),
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
                                                flex_direction: FlexDirection::Row,
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                flex_wrap: FlexWrap::Wrap,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(ButtonVisuals::default())
                                        .insert(EscapeMenuGeneralHeaderButton)
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "General".to_string(),
                                                TextStyle {
                                                    font: sourcecode_font.clone(),
                                                    font_size: 12.0,
                                                    color: Color::WHITE.into(),
                                                },
                                            ));
                                        });
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(33.), Val::Percent(100.)),
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
                                                flex_direction: FlexDirection::Row,
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                flex_wrap: FlexWrap::Wrap,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(ButtonVisuals::default())
                                        .insert(EscapeMenuGraphicsHeaderButton)
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "Graphics".to_string(),
                                                TextStyle {
                                                    font: sourcecode_font.clone(),
                                                    font_size: 12.0,
                                                    color: Color::WHITE.into(),
                                                },
                                            ));
                                        });
                                });
                        });
                });
        })
        .id();
    commands.insert_resource(EscapeMenuState {
        root: escape_root,
        visible: false,
    })
}
