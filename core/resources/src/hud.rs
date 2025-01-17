use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub struct HudState {
    pub expanded: bool,
    pub root_entity: Entity,
    pub left_content_node: Entity,
    pub right_content_node: Entity,
    pub center_content_node: Entity,
    pub left_edge_node: Entity,
    pub right_edge_node: Entity,
    pub top_edge_node: Entity,
    pub bottom_edge_node: Entity,
}
