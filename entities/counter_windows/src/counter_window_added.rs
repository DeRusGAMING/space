use std::collections::BTreeMap;

use atmospherics::diffusion::{get_atmos_index, AtmosphericsResource};
use bevy::prelude::{Added, Entity, Query, ResMut, Transform};
use entity::{
    entity_data::{DefaultMapEntity, EntityData},
    examine::{Examinable, RichName},
};
use gridmap::grid::{EntityGridData, GridmapMain};
use map::{map::GREEN_MAP_TILE_COUNTER, map_input::MapData};
use math::grid::{world_to_cell_id, Vec2Int};
use text_api::core::{FURTHER_ITALIC_FONT, HEALTHY_COLOR};

use super::{
    counter_window_events::CounterWindow,
    spawn::{BRIDGE_COUNTER_WINDOW_ENTITY_NAME, SECURITY_COUNTER_WINDOW_ENTITY_NAME},
};

/// On counter window spawn.
#[cfg(feature = "server")]
pub(crate) fn counter_window_added(
    counter_windows: Query<(Entity, &Transform), Added<CounterWindow>>,
    mut atmospherics_resource: ResMut<AtmosphericsResource>,
) {
    for (_airlock_entity, rigid_body_position_component) in counter_windows.iter() {
        let cell_id = world_to_cell_id(rigid_body_position_component.translation.into());
        let cell_id2 = Vec2Int {
            x: cell_id.x,
            y: cell_id.z,
        };
        if AtmosphericsResource::is_id_out_of_range(cell_id2) {
            continue;
        }
        let atmos_id = get_atmos_index(cell_id2);
        let atmospherics = atmospherics_resource
            .atmospherics
            .get_mut(atmos_id)
            .unwrap();

        atmospherics.blocked = true;
    }
}

/// On default map counter window spawn.
#[cfg(feature = "server")]
pub(crate) fn counter_window_default_map_added(
    mut default_counter_windows: Query<
        (
            Entity,
            &Transform,
            &DefaultMapEntity,
            &EntityData,
            &mut Examinable,
        ),
        Added<CounterWindow>,
    >,
    mut map_data: ResMut<MapData>,
    mut gridmap_main: ResMut<GridmapMain>,
) {
    for (
        counter_window_entity,
        rigid_body_position_component,
        _,
        entity_data_component,
        mut examinable_component,
    ) in default_counter_windows.iter_mut()
    {
        let cell_id = world_to_cell_id(rigid_body_position_component.translation.into());
        let cell_id2 = Vec2Int {
            x: cell_id.x,
            y: cell_id.z,
        };
        map_data.data.insert(cell_id2, GREEN_MAP_TILE_COUNTER);

        gridmap_main.entity_data.insert(
            cell_id,
            EntityGridData {
                entity: counter_window_entity,
                entity_type: entity_data_component.entity_type.to_string(),
            },
        );

        if entity_data_component.entity_type.to_string() == SECURITY_COUNTER_WINDOW_ENTITY_NAME {
            examinable_component.name = RichName {
                name: "security counter window".to_string(),
                n: false,
                ..Default::default()
            };
            let mut examine_map = BTreeMap::new();
            examine_map.insert(0, "An airtight security window. It will only grant access to those authorised to use it.".to_string());
            examine_map.insert(
                1,
                "[font=".to_string()
                    + FURTHER_ITALIC_FONT
                    + "][color="
                    + HEALTHY_COLOR
                    + "]It is fully operational.[/color][/font]",
            );
            examinable_component.assigned_texts = examine_map;
        } else if entity_data_component.entity_type.to_string() == BRIDGE_COUNTER_WINDOW_ENTITY_NAME
        {
            examinable_component.name = RichName {
                name: "bridge counter window".to_string(),
                n: false,
                ..Default::default()
            };
            let mut examine_map = BTreeMap::new();
            examine_map.insert(0, "An airtight bridge window. It will only grant access to those authorised to use it.".to_string());
            examine_map.insert(
                1,
                "[font=".to_string()
                    + FURTHER_ITALIC_FONT
                    + "][color="
                    + HEALTHY_COLOR
                    + "]It is fully operational.[/color][/font]",
            );
            examinable_component.assigned_texts = examine_map;
        }
    }
}
