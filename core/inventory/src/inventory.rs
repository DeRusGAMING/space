use std::collections::HashMap;

use bevy::prelude::{Component, Entity, EventReader, SystemLabel, Query, warn, EventWriter, info};
use math::grid::Vec2Int;
use networking::{server::{ConnectedPlayer, OutgoingReliableServerMessage}, client::IncomingReliableServerMessage};
use serde::{Serialize, Deserialize};

use crate::{item::InventoryItem, net::InventoryServerMessage};

#[derive(PartialEq, Copy, Clone, Debug, Default)]

pub enum SlotType {
    #[default]
    Generic,
    Helmet,
    Jumpsuit,
    Holster,
}

/// An inventory slot, an inventory can contain many of these.
#[derive(Default)]
pub struct Slot {
    pub name: String,
    pub slot_type: SlotType,
    pub space: HashMap<Vec2Int, Entity>,
    pub items: Vec<SlotItem>,
    // Dividable by two.
    pub size: Vec2Int,
}
/// Event that adds an inventory item entity to an inventory slot.
pub struct AddItemToSlot {
    pub slot_id : u8,
    pub inventory_entity : Entity,
    pub item_entity : Entity,
}

pub struct SlotItem {
    pub entity : Entity,
    pub position : Vec2Int,
}

/// The inventory component.
#[derive(Component, Default)]

pub struct Inventory {
    pub slots: HashMap<u8, Slot>,
    pub active_item: Option<Entity>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]

pub enum SpawnItemLabel {
    SpawnHeldItem,
    AddingComponent,
}

/// Event that fires when an item was successfully added to an inventory slot.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemAddedToSlot {
    pub slot_id : u8,
    pub inventory_entity : Entity,
    pub item_entity : Entity,
    pub position : Vec2Int,
}

pub (crate) fn add_item_to_slot(mut added : EventWriter<ItemAddedToSlot>, mut events : EventReader<AddItemToSlot>, mut inventory_query : Query<&mut Inventory>, inventory_item_query : Query<&InventoryItem>,) {

    for event in events.iter() {

        match inventory_item_query.get(event.item_entity) {
            Ok(inventory_item_component) => {
                match inventory_query.get_mut(event.inventory_entity) {
                    Ok(mut inventory_component) => {

                        for (id,slot) in inventory_component.slots.iter_mut() {
                            if id == &event.slot_id {

                                let start_x = -(slot.size.x / 2);
                                let start_y = slot.size.y / 2;

                                let start_position = Vec2Int { x: start_x, y: start_y };
                                let mut slot_start_position = start_position.clone();
                                let mut free = false;
                                let test_cells_amount = inventory_item_component.slot_size.x*inventory_item_component.slot_size.y;

                                loop {
                                    
                                    for test_slot_i in 0..test_cells_amount {

                                        let y;
                                        if test_slot_i == 0 {
                                            y=0;
                                        } else {
                                            y=test_slot_i / inventory_item_component.slot_size.y;
                                        }
                                        let x;
                                        if test_slot_i == 0 {
                                            x=0;
                                        } else {
                                            x=test_slot_i- (y*inventory_item_component.slot_size.y);
                                        }
                                        
                                        match slot.space.get(&Vec2Int{ x: x+slot_start_position.x, y:y+slot_start_position.y }) {
                                            Some(_) => {break;},
                                            None => {},
                                        }


                                        if test_slot_i == test_cells_amount-1 {
                                            free=true;
                                        }


                                    }

                                    if free {
                                        break;
                                    }

                                    if slot_start_position.x > (slot.size.x/2) - inventory_item_component.slot_size.x {
                                        slot_start_position.x=start_position.x;
                                        slot_start_position.y-=1;
                                    } else {
                                        slot_start_position.x+=1;
                                    }

                                    if slot_start_position.y < -(slot.size.y/2) {
                                        break;
                                    }
                                    
                                }

                                if !free {
                                    warn!("No empty space left in inventory slot.");
                                    continue;
                                }

                                slot.items.push(SlotItem {
                                    entity: event.item_entity,
                                    position: slot_start_position,
                                });

                                for test_slot_i in 0..test_cells_amount {

                                    let y;
                                    if test_slot_i == 0 {
                                        y=0;
                                    } else {
                                        y=test_slot_i / inventory_item_component.slot_size.y;
                                    }
                                    let x;
                                    if test_slot_i == 0 {
                                        x=0;
                                    } else {
                                        x=test_slot_i- (y*inventory_item_component.slot_size.y);
                                    }

                                    slot.space.insert(Vec2Int{ x: x+slot_start_position.x, y:y+slot_start_position.y }, event.item_entity);

                                }

                                added.send(ItemAddedToSlot { slot_id: event.slot_id, inventory_entity: event.inventory_entity, item_entity: event.item_entity, position: slot_start_position});

                            }
                        }


                    },
                    Err(_) => {
                        warn!("Couldnt find inventory component for entity");
                    },
                }

            },
            Err(_) => {
                warn!("Couldnt find inventory item.");
            },
        }

    }

}

pub (crate) fn added_item_to_slot(mut events : EventReader<ItemAddedToSlot>, connected_players : Query<&ConnectedPlayer>, mut net : EventWriter<OutgoingReliableServerMessage<InventoryServerMessage>>) {

    for event in events.iter() {

        match connected_players.get(event.inventory_entity) {
            Ok(player) => {

                net.send(OutgoingReliableServerMessage { handle: player.handle, message: InventoryServerMessage::ItemAddedToSlot(event.clone()) });

            },
            Err(_) => {},
        }

    }

}

pub (crate) fn client_item_added_to_slot(
    mut net : EventReader<IncomingReliableServerMessage<InventoryServerMessage>>,
) {

    for message in net.iter() {
        match &message.message {
            InventoryServerMessage::ItemAddedToSlot(event) => {

                info!("Received item added to slot: {:?}", event.item_entity);

            },
        }
    }

}
