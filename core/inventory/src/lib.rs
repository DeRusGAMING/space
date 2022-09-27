//! Inventory management of entities.
//! A storage component for inventory items.
//! Not all inventory holding entities are humanoids or pawns.
//! Currently used by Humanoids.
//! Also manages actions related to the inventory system.

/// Manage actions related to inventory.
mod actions;
/// Manage inventory entity updates, such as attaching items to other items.
mod entity_update;
/// Manage inventory item events such as dropping or throwing them.
mod item_events;
/// Manage netcode.
mod net;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Switch actively selected hand for inventory holder.
mod switch_hands;
