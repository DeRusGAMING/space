use std::env;

use bevy::{
    prelude::{App, IntoSystemDescriptor, Plugin, SystemSet},
    time::FixedTimestep,
};
use entity::{entity_data::INTERPOLATION_LABEL1, examine::RichName};
use player::{plugin::ConfigurationLabel, spawn_points::SpawnPoints};
use resources::labels::{
    ActionsLabels, PostUpdateLabels, StartupLabels, SummoningLabels, UpdateLabels,
};

use crate::{
    connections::configure,
    examine::{
        examine_grid, examine_map, examine_map_abilities, examine_map_health, finalize_examine_map,
        finalize_grid_examine_input, set_action_header_name, GridmapExamineMessages,
        InputExamineMap,
    },
    fov::ProjectileFOV,
    grid::{GridmapData, GridmapDetails1, GridmapMain, RemoveCell},
    init::{startup_build_map, startup_map_cells, startup_misc_resources},
    networking::incoming_messages,
};
use bevy::app::CoreStage::{PostUpdate, PreUpdate};

use super::{
    events::{gridmap_updates_manager, remove_cell},
    fov::{projectile_fov, senser_update_fov, DoryenMap},
    sensing_ability::gridmap_sensing_ability,
};

#[allow(dead_code)]
pub struct Details1CellProperties {
    pub id: i64,
    pub name: RichName,
    pub description: String,
}

impl Default for Details1CellProperties {
    fn default() -> Self {
        Self {
            id: 0,
            name: Default::default(),
            description: "".to_string(),
        }
    }
}

pub struct GridmapPlugin;

impl Plugin for GridmapPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.init_resource::<GridmapDetails1>()
                .init_resource::<GridmapData>()
                .init_resource::<DoryenMap>()
                .init_resource::<SpawnPoints>()
                .add_system(senser_update_fov)
                .add_system(projectile_fov)
                .add_system(remove_cell.label(UpdateLabels::DeconstructCell))
                .add_event::<RemoveCell>()
                .add_startup_system(startup_misc_resources.label(StartupLabels::MiscResources))
                .add_startup_system(
                    startup_map_cells
                        .label(StartupLabels::InitDefaultGridmapData)
                        .label(SummoningLabels::TriggerSummon)
                        .after(StartupLabels::MiscResources),
                )
                .init_resource::<GridmapDetails1>()
                .add_startup_system(
                    startup_build_map
                        .label(StartupLabels::BuildGridmap)
                        .after(StartupLabels::InitDefaultGridmapData),
                )
                .add_system_set(
                    SystemSet::new()
                        .with_run_criteria(
                            FixedTimestep::step(1. / 4.).with_label(INTERPOLATION_LABEL1),
                        )
                        .with_system(gridmap_updates_manager),
                )
                .init_resource::<GridmapMain>()
                .add_system(gridmap_sensing_ability)
                .add_system(examine_map.after(ActionsLabels::Action))
                .add_system(
                    set_action_header_name
                        .after(ActionsLabels::Build)
                        .before(ActionsLabels::Approve),
                )
                .add_system(examine_map.after(ActionsLabels::Action))
                .add_system(examine_map_health.after(ActionsLabels::Action))
                .add_system(examine_map_abilities.after(ActionsLabels::Action))
                .add_event::<ProjectileFOV>()
                .add_system_to_stage(PreUpdate, finalize_grid_examine_input)
                .add_system_to_stage(PreUpdate, incoming_messages)
                .add_event::<InputExamineMap>()
                .init_resource::<GridmapExamineMessages>()
                .add_system_to_stage(
                    PostUpdate,
                    finalize_examine_map.before(PostUpdateLabels::EntityUpdate),
                )
                .add_system(examine_grid.after(ActionsLabels::Action))
                .add_system(
                    configure
                        .label(ConfigurationLabel::Main)
                        .after(ConfigurationLabel::SpawnEntity),
                );
        }
    }
}
