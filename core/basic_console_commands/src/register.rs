use resources::is_server::is_server;
pub fn register_basic_console_commands_for_type<T: EntityType + Clone + Default + 'static>(
    app: &mut App,
) {
    if is_server() {
        app.add_event::<RconSpawnEntity<T>>()
            .add_system(rcon_entity_console_commands::<T>);
    }
}
use bevy::prelude::App;
use entity::entity_types::EntityType;

use crate::commands::{
    rcon_entity_console_commands, rcon_spawn_entity, RconSpawnEntity, RconSpawnHeldEntity,
};

pub fn register_basic_console_commands_for_inventory_item_type<
    T: EntityType + Clone + Default + 'static,
>(
    app: &mut App,
) {
    if is_server() {
        app.add_event::<RconSpawnEntity<T>>()
            .add_system(rcon_entity_console_commands::<T>)
            .add_system(rcon_spawn_entity::<T>)
            .add_event::<RconSpawnHeldEntity<T>>();
    }
}
