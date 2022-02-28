use std::{collections::HashMap, f32::consts::PI};


use bevy_internal::prelude::{Entity, EventReader, Query, Local};

use crate::space::core::pawn::{components::StandardCharacter, events::InputMouseDirectionUpdate};

#[derive(Default)]
pub struct TimeStampPerEntity {
    pub data: HashMap<Entity, u64>,
}

pub fn mouse_direction_update(
    mut update_events: EventReader<InputMouseDirectionUpdate>,
    mut standard_characters: Query<&mut StandardCharacter>,
    mut time_stamp_per_entity: Local<TimeStampPerEntity>,
) {
    for event in update_events.iter() {
        match time_stamp_per_entity.data.get(&event.entity) {
            Some(time_stamp) => {
                if time_stamp > &event.time_stamp {
                    continue;
                }
            }
            None => {}
        }

        time_stamp_per_entity
            .data
            .insert(event.entity, event.time_stamp);

        match standard_characters.get_mut(event.entity) {
            Ok(mut standard_character_component) => {
                if standard_character_component.combat_mode == false {
                    continue;
                }

                let direction = event.direction.clamp(-PI, PI);

                standard_character_component.facing_direction = direction;
            }
            Err(_rr) => {}
        }
    }
}
