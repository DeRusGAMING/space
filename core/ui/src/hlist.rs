use bevy::{
    prelude::{
        warn, Added, AssetServer, BuildChildren, ButtonBundle, Changed, Color, Commands, Component,
        Entity, EventReader, EventWriter, Parent, Query, Res, TextBundle, With,
    },
    text::{TextSection, TextStyle},
    ui::{BackgroundColor, Interaction, Style},
};

use crate::{button::SFButton, fonts::SOURCECODE_REGULAR_FONT};

#[derive(Component, Default)]
pub struct HList {
    pub selected: Option<u8>,
    pub selections: Vec<String>,
    // Internal usage.
    pub selections_entities: Vec<Entity>,
}

pub(crate) fn hlist_created(
    mut query: Query<(Entity, &mut HList), Added<HList>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventWriter<FreezeButton>,
) {
    for (entity, mut hlist) in query.iter_mut() {
        let source_code = asset_server.load(SOURCECODE_REGULAR_FONT);

        commands.entity(entity).with_children(|parent| {
            let mut entities = vec![];
            let mut i = 0;
            for selection in &mut hlist.selections {
                let id = parent
                    .spawn(ButtonBundle {
                        style: Style {
                            ..Default::default()
                        },

                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..Default::default()
                    })
                    .insert(SFButton::default())
                    .insert(HListSub { selection: i })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_sections(vec![TextSection::new(
                            " ".to_string() + selection + " ",
                            TextStyle {
                                font: source_code.clone(),
                                font_size: 12.,
                                color: Color::WHITE,
                            },
                        )]));
                    })
                    .id();
                entities.push(id);
                i += 1;
            }
            hlist.selections_entities = entities;
        });
        match hlist.selected {
            Some(s) => {
                events.send(FreezeButton {
                    entity: hlist.selections_entities[s as usize],
                    id: s,
                    frozen: true,
                    first_time_freeze: true,
                });
            }
            None => {}
        }
    }
}

#[derive(Component)]
pub struct HListSub {
    pub selection: u8,
}

pub(crate) fn hlist_input(
    interaction_query: Query<
        (Entity, &Interaction, &Parent, &HListSub),
        (Changed<Interaction>, With<SFButton>),
    >,
    mut freeze: EventWriter<FreezeButton>,
    hlist_query: Query<&HList>,
) {
    for (entity, interaction, parent, hlist_sub) in interaction_query.iter() {
        match interaction {
            Interaction::Clicked => match hlist_query.get(**parent) {
                Ok(hlist) => {
                    freeze.send(FreezeButton {
                        entity,
                        frozen: true,
                        id: hlist_sub.selection,
                        first_time_freeze: false,
                    });
                    match hlist.selected {
                        Some(e) => {
                            let ent = hlist.selections_entities[e as usize];
                            if entity == ent {
                                continue;
                            }
                            freeze.send(FreezeButton {
                                entity: ent,
                                frozen: false,
                                id: hlist_sub.selection,
                                first_time_freeze: false,
                            });
                        }
                        None => {}
                    }
                }
                Err(_) => {}
            },
            _ => (),
        }
    }
}

pub struct FreezeButton {
    pub entity: Entity,
    pub id: u8,
    pub frozen: bool,
    pub first_time_freeze: bool,
}

pub(crate) fn freeze_button(
    mut events: EventReader<FreezeButton>,
    mut query: Query<(&mut SFButton, &Parent), With<HListSub>>,
    mut bg_query: Query<&mut BackgroundColor>,
    mut hlist_query: Query<&mut HList>,
) {
    for event in events.iter() {
        match query.get_mut(event.entity) {
            Ok((mut b, parent)) => {
                b.frozen = event.frozen;

                if !event.frozen {
                    continue;
                }
                match bg_query.get_mut(event.entity) {
                    Ok(mut bg) => {
                        *bg = b.pressed_color.into();
                    }
                    Err(_) => {
                        warn!("couldnt find bg color (1).");
                    }
                }
                let mut old_selection = None;

                match hlist_query.get_mut(**parent) {
                    Ok(mut hlist) => match hlist.selected {
                        Some(si) => {
                            old_selection = Some(hlist.selections_entities[si as usize]);
                            hlist.selected = Some(event.id);
                        }
                        None => {}
                    },
                    Err(_) => {
                        warn!("couldnt find bg color.");
                    }
                }

                match old_selection {
                    Some(old_ent) => match bg_query.get_mut(old_ent) {
                        Ok(mut bg) => {
                            if event.first_time_freeze {
                                continue;
                            }
                            *bg = b.default_parent_color.into();
                        }
                        Err(_) => {
                            warn!("couldnt find bg color.");
                        }
                    },
                    None => {}
                }
            }
            Err(_) => {
                warn!("Couldnt find SFButton to freeze.");
            }
        }
    }
}
