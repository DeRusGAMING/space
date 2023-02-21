use actions::core::TargetCell;
use bevy::prelude::{Component, Entity};

/// The component.
#[derive(Component, Default)]

pub struct ConstructionTool {
    /// Currently selected construction cell option.
    pub construction_option: Option<String>,
}

/// Player requested input event.

pub struct InputConstruct {
    /// Connection handle that fired this input.
    pub handle_option: Option<u64>,
    /// Build on gridmap cell:
    pub target_cell: TargetCell,
    /// Entity that requested to construct.
    pub belonging_entity: Entity,
}
/// Player requested input event.

pub struct InputConstructionOptions {
    /// Connection handle that fired this input.
    pub handle_option: Option<u64>,
    /// Entity that requested to select construction option.
    pub entity: Entity,
}
/// Player requested input event.

pub struct InputDeconstruct {
    /// Connection handle that fired this input.
    pub handle_option: Option<u64>,
    pub target_cell_option: Option<TargetCell>,
    pub target_entity_option: Option<Entity>,
    /// Entity that requested to deconstruct.
    pub belonging_entity: Entity,
}
/// Client input construction options selection event.

pub struct InputConstructionOptionsSelection {
    pub handle_option: Option<u64>,
    pub menu_selection: String,
    // Entity has been validated.
    pub entity: Entity,
}