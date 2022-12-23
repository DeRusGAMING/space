use std::collections::BTreeMap;

use atmospherics::diffusion::{get_atmos_index, AtmosphericsResource};
use bevy::prelude::{Added, Entity, Query, ResMut, Transform};
use entity::{
    entity_data::{DefaultMapEntity, EntityData},
    examine::{Examinable, RichName},
};
use gridmap::grid::{EntityGridData, GridmapMain};
use map::{map::GREEN_MAP_TILE_ENTRANCE, map_input::MapData};
use math::grid::{world_to_cell_id, Vec2Int};
use text_api::core::{FURTHER_ITALIC_FONT, HEALTHY_COLOR};

use crate::spawn::{
    BRIDGE_AIRLOCK_ENTITY_NAME, GOVERNMENT_AIRLOCK_ENTITY_NAME, VACUUM_AIRLOCK_ENTITY_NAME,
};

use super::resources::Airlock;

/// On new air lock spawn.
#[cfg(feature = "server")]
pub(crate) fn airlock_added(
    mut airlocks: Query<(Entity, &EntityData, &Transform, &mut Examinable), Added<Airlock>>,
    mut atmospherics_resource: ResMut<AtmosphericsResource>,
) {
    for (
        _airlock_entity,
        entity_data_component,
        rigid_body_position_component,
        mut examinable_component,
    ) in airlocks.iter_mut()
    {
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

        if entity_data_component.entity_type == BRIDGE_AIRLOCK_ENTITY_NAME {
            examinable_component.name = RichName {
                name: "bridge airlock".to_string(),
                n: false,
                ..Default::default()
            };
            let mut examine_map = BTreeMap::new();
            examine_map.insert(
                0,
                "An air lock with bridge department colors. Access is only granted to high ranking staff."
                    .to_string(),
            );
            examine_map.insert(
                1,
                "[font=".to_string()
                    + FURTHER_ITALIC_FONT
                    + "][color="
                    + HEALTHY_COLOR
                    + "]It is fully operational.[/color][/font]",
            );
            examinable_component.assigned_texts = examine_map;
        } else if entity_data_component.entity_type == GOVERNMENT_AIRLOCK_ENTITY_NAME {
            examinable_component.name = RichName {
                name: "government airlock".to_string(),
                n: false,
                ..Default::default()
            };
            let mut examine_map = BTreeMap::new();
            examine_map.insert(
                0,
                "An air lock with government department colors. Access is only granted to a few elite crew members on-board."
                    .to_string(),
            );
            examine_map.insert(
                1,
                "[font=".to_string()
                    + FURTHER_ITALIC_FONT
                    + "][color="
                    + HEALTHY_COLOR
                    + "]It is fully operational.[/color][/font]",
            );
        } else if entity_data_component.entity_type == "securityAirlock" {
            examinable_component.name = RichName {
                name: "security airlock".to_string(),
                n: false,
                ..Default::default()
            };
            let mut examine_map = BTreeMap::new();
            examine_map.insert(
                0,
                "An air lock with security department markings. It will only grant access to those authorised to use it."
                    .to_string(),
            );
            examine_map.insert(
                1,
                "[font=".to_string()
                    + FURTHER_ITALIC_FONT
                    + "][color="
                    + HEALTHY_COLOR
                    + "]It is fully operational.[/color][/font]",
            );
        } else if entity_data_component.entity_type == VACUUM_AIRLOCK_ENTITY_NAME {
            examinable_component.name = RichName {
                name: "vacuum airlock".to_string(),
                n: false,
                ..Default::default()
            };
            let mut examine_map = BTreeMap::new();
            examine_map.insert(
                0,
                "An air lock with vacuum warning colors. Opening this door will expose you to space."
                    .to_string(),
            );
            examine_map.insert(
                1,
                "[font=".to_string()
                    + FURTHER_ITALIC_FONT
                    + "][color="
                    + HEALTHY_COLOR
                    + "]It is fully operational.[/color][/font]",
            );
        }
    }
}

/// When a default map air lock gets spawned.
#[cfg(feature = "server")]
pub(crate) fn airlock_default_map_added(
    airlock_windows: Query<(Entity, &Transform, &DefaultMapEntity, &EntityData), Added<Airlock>>,
    mut map_data: ResMut<MapData>,
    mut gridmap_main: ResMut<GridmapMain>,
) {
    for (airlock_entity, rigid_body_position_component, _, entity_data_component) in
        airlock_windows.iter()
    {
        let cell_id = world_to_cell_id(rigid_body_position_component.translation.into());
        let cell_id2 = Vec2Int {
            x: cell_id.x,
            y: cell_id.z,
        };
        map_data.data.insert(cell_id2, GREEN_MAP_TILE_ENTRANCE);

        gridmap_main.entity_data.insert(
            cell_id,
            EntityGridData {
                entity: airlock_entity,
                entity_type: entity_data_component.entity_type.to_string(),
            },
        );
    }
}