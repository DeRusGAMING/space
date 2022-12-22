use std::collections::HashMap;

use bevy::prelude::Resource;

pub const SF_CONTENT_PREFIX: &str = "sf::";

/// Resource that contains entity meta data.
#[derive(Default, Resource)]
#[cfg(feature = "server")]
pub struct EntityDataResource {
    pub data: Vec<EntityDataProperties>,
    pub incremented_id: usize,
    pub id_to_name: HashMap<usize, String>,
    pub name_to_id: HashMap<String, usize>,
}

#[cfg(feature = "server")]
impl EntityDataResource {
    pub fn get_id_inc(&mut self) -> usize {
        let return_val = self.incremented_id.clone();
        self.incremented_id += 1;
        return_val
    }
}

#[cfg(feature = "server")]
impl Default for EntityDataProperties {
    fn default() -> Self {
        Self {
            name: Default::default(),
            id: Default::default(),
            grid_item: None,
        }
    }
}

/// Meta data for an entity.
#[cfg(feature = "server")]
pub struct EntityDataProperties {
    pub name: String,
    pub id: usize,
    pub grid_item: Option<GridItemData>,
}
use bevy::prelude::Transform;

/// For entities that are also registered with the gridmap.
#[cfg(feature = "server")]
pub struct GridItemData {
    pub transform_offset: Transform,
    /// So this entity can be built on a cell when another item is already present on that cell.
    pub can_be_built_with_grid_item: Vec<String>,
}
