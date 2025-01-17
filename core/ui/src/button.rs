use bevy::prelude::{Color, Component};
use bevy::window::{PrimaryWindow, Window};

use crate::text_input::INPUT_TEXT_BG;

pub const HOVERED_BUTTON: Color = INPUT_TEXT_BG;
pub const PRESSED_BUTTON: Color = Color::rgb(0.49, 0.73, 0.91);

/// Component for button visuals.

#[derive(Component)]
pub struct SFButton {
    pub hovered_color: Color,
    pub pressed_color: Color,
    pub default_parent_color: Color,
    pub default_color_option: Option<Color>,
    pub color_parent: bool,
    pub frozen: bool,
}

impl Default for SFButton {
    fn default() -> Self {
        Self {
            hovered_color: HOVERED_BUTTON,
            pressed_color: PRESSED_BUTTON,
            default_parent_color: Color::rgb(0.15, 0.15, 0.15),
            default_color_option: None,
            color_parent: true,
            frozen: false,
        }
    }
}
use bevy::prelude::warn;
use bevy::prelude::{Button, Parent, Query, With};
use bevy::ui::BackgroundColor;
use bevy::{
    prelude::{Changed, Entity},
    ui::Interaction,
};

pub(crate) fn button_hover_visuals(
    mut interaction_query: Query<
        (Entity, &Interaction, &Parent, &SFButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut color_query: Query<&mut BackgroundColor>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (entity, interaction, parent, button_visuals) in &mut interaction_query {
        let primary = primary_query.get_single().unwrap();

        match *interaction {
            Interaction::Clicked => {
                if button_visuals.color_parent {
                    if !primary.cursor.visible {
                        continue;
                    }
                    match color_query.get_mut(parent.get()) {
                        Ok(mut c) => {
                            *c = button_visuals.pressed_color.into();
                        }
                        Err(_rr) => {
                            warn!("Couldnt find button parent.");
                            continue;
                        }
                    }
                }
                match color_query.get_mut(entity) {
                    Ok(mut c) => {
                        *c = button_visuals.pressed_color.into();
                    }
                    Err(_rr) => {
                        warn!("Couldnt find button.");
                        continue;
                    }
                }
            }
            Interaction::Hovered => {
                if button_visuals.frozen {
                    continue;
                }
                if !primary.cursor.visible {
                    continue;
                }
                if button_visuals.color_parent {
                    match color_query.get_mut(parent.get()) {
                        Ok(mut c) => {
                            *c = button_visuals.hovered_color.into();
                        }
                        Err(_rr) => {
                            warn!("Couldnt find button parent.");
                            continue;
                        }
                    }
                }
                match color_query.get_mut(entity) {
                    Ok(mut c) => {
                        *c = button_visuals.hovered_color.into();
                    }
                    Err(_rr) => {
                        warn!("Couldnt find button.");
                        continue;
                    }
                }
            }
            Interaction::None => {
                if button_visuals.frozen {
                    continue;
                }
                if button_visuals.color_parent {
                    match color_query.get_mut(parent.get()) {
                        Ok(mut c) => {
                            *c = button_visuals.default_parent_color.into();
                        }
                        Err(_rr) => {
                            warn!("Couldnt find button parent.");
                            continue;
                        }
                    }
                }
                match color_query.get_mut(entity) {
                    Ok(mut c) => match button_visuals.default_color_option {
                        Some(col) => {
                            *c = col.into();
                        }
                        None => {
                            *c = button_visuals.default_parent_color.into();
                        }
                    },
                    Err(_rr) => {
                        warn!("Couldnt find button.");
                        continue;
                    }
                }
            }
        }
    }
}
