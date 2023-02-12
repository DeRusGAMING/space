use bevy::{
    prelude::{App, IntoSystemDescriptor, Plugin, SystemSet},
    time::FixedTimestep,
};
use entity::entity_data::INTERPOLATION_LABEL1;
use networking::messaging::{register_reliable_message, MessageSender};
use player::plugin::ConfigurationLabel;
use resources::{
    is_server::is_server,
    labels::{ActionsLabels, BuildingLabels, PostUpdateLabels, StartupLabels},
};

use crate::{
    connections::configure,
    examine::{
        examine_grid, examine_map, examine_map_abilities, examine_map_health, finalize_examine_map,
        finalize_grid_examine_input, incoming_messages, set_action_header_name,
        GridmapExamineMessages, InputExamineMap,
    },
    floor::{add_floor_tile, AddTile},
    fov::ProjectileFOV,
    graphics::set_cell_graphics,
    grid::{Gridmap, RemoveCell},
    init::{load_ron_gridmap, startup_map_cell_properties, startup_misc_resources},
    net::{GridmapClientMessage, GridmapServerMessage},
    wall::add_wall_tile,
};
use bevy::app::CoreStage::{PostUpdate, PreUpdate};

use super::{
    fov::{senser_update_fov, DoryenMap},
    sensing_ability::gridmap_sensing_ability,
    updates::gridmap_updates_manager,
};

pub struct GridmapPlugin;

impl Plugin for GridmapPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(senser_update_fov)
                .add_event::<RemoveCell>()
                .add_system_set(
                    SystemSet::new()
                        .with_run_criteria(
                            FixedTimestep::step(1. / 4.).with_label(INTERPOLATION_LABEL1),
                        )
                        .with_system(gridmap_updates_manager),
                )
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
        } else {
            app.add_system(set_cell_graphics);
        }

        app.add_startup_system(startup_misc_resources.label(StartupLabels::MiscResources))
            .add_startup_system(
                startup_map_cell_properties
                    .label(StartupLabels::InitDefaultGridmapData)
                    .label(BuildingLabels::TriggerBuild)
                    .after(StartupLabels::MiscResources),
            )
            .add_startup_system(
                load_ron_gridmap
                    .label(StartupLabels::BuildGridmap)
                    .after(StartupLabels::InitDefaultGridmapData),
            )
            .init_resource::<Gridmap>()
            .init_resource::<DoryenMap>()
            .add_system(add_wall_tile)
            .add_system(add_floor_tile)
            .add_event::<AddTile>();

        register_reliable_message::<GridmapClientMessage>(app, MessageSender::Client);
        register_reliable_message::<GridmapServerMessage>(app, MessageSender::Server);
    }
}
