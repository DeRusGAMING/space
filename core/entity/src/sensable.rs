use bevy::ecs::entity::Entity;
use bevy::prelude::{Component, EventWriter, Res};
use networking::messages::NetUnloadEntity;
use server_instance::core::HandleToEntity;

use crate::entity_data::unload_entity;

/// The component for entities that can be sensed.
#[derive(Component, Default)]
#[cfg(feature = "server")]
pub struct Sensable {
    pub is_light: bool,
    pub is_audible: bool,
    pub sensed_by: Vec<Entity>,
    pub sensed_by_cached: Vec<Entity>,
    pub always_sensed: bool,
}

#[cfg(feature = "server")]
impl Sensable {
    pub fn despawn(
        &mut self,
        entity: Entity,
        mut net_unload_entity: &mut EventWriter<NetUnloadEntity>,
        handle_to_entity: &Res<HandleToEntity>,
    ) {
        // Shouldn't be called from the same stage visible_checker.system() runs in.

        for sensed_by_entity in self.sensed_by.iter() {
            match handle_to_entity.inv_map.get(&sensed_by_entity) {
                Some(handle) => {
                    unload_entity(*handle, entity, &mut net_unload_entity, true);
                }
                None => {}
            }
        }
        for sensed_by_entity in self.sensed_by_cached.iter() {
            match handle_to_entity.inv_map.get(&sensed_by_entity) {
                Some(handle) => {
                    unload_entity(*handle, entity, &mut net_unload_entity, true);
                }
                None => {}
            }
        }

        self.sensed_by = vec![];
        self.sensed_by_cached = vec![];
    }
}
