use bevy::prelude::{Changed, Query};
use entity::entity_data::{get_entity_update_difference, EntityUpdates};
use networking::server::EntityUpdateData;

use crate::item::InventoryItem;

/// Inventory item update for 3d Godot attachments.

pub(crate) fn inventory_item_update(
    mut updated_entities: Query<(&InventoryItem, &mut EntityUpdates), Changed<InventoryItem>>,
) {
    for (inventory_item_component, mut entity_updates_component) in updated_entities.iter_mut() {
        let old_entity_updates = entity_updates_component.updates.clone();

        let insert_map = entity_updates_component
            .updates
            .get_mut(&".".to_string())
            .unwrap();

        insert_map.insert(
            "worn_is_attached".to_string(),
            EntityUpdateData::Bool(inventory_item_component.is_attached_when_worn),
        );

        let difference_updates =
            get_entity_update_difference(old_entity_updates, &entity_updates_component.updates);

        entity_updates_component
            .updates_difference
            .push(difference_updates);
    }
}
