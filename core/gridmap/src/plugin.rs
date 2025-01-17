use std::time::Duration;

use bevy::{
    prelude::{App, CoreSet, IntoSystemConfig, Plugin},
    time::common_conditions::on_fixed_timer,
};
use networking::messaging::{register_reliable_message, MessageSender};
use player::plugin::ConfigurationLabel;
use resources::{
    is_server::is_server,
    labels::{ActionsLabels, BuildingLabels, PostUpdateLabels, StartupLabels},
};

use crate::{
    connections::configure,
    construction::{
        change_ghost_tile_request, client_mouse_click_input, create_select_cell_cam_state,
        input_ghost_rotation, input_yplane_position, move_ylevel_plane, register_input,
        select_cell_in_front_camera, set_yplane_position, show_ylevel_plane, update_ghost_cell,
        ConstructionCellSelectionChanged, GhostTileLabel, SetYPlanePosition,
    },
    examine::{
        examine_grid, examine_map, examine_map_abilities, examine_map_health, finalize_examine_map,
        finalize_grid_examine_input, incoming_messages, set_action_header_name,
        GridmapExamineMessages, InputExamineMap,
    },
    fov::ProjectileFOV,
    graphics::set_cell_graphics,
    grid::{
        add_cell_client, add_tile, add_tile_collision, add_tile_net, remove_cell_client,
        remove_tile, remove_tile_net, AddGroup, AddTile, Gridmap, RemoveTile,
    },
    init::{load_ron_gridmap, startup_map_tile_properties, startup_misc_resources},
    net::{GridmapClientMessage, GridmapServerMessage},
    wall::add_wall_group,
};

use super::{
    fov::{senser_update_fov, DoryenMap},
    sensing_ability::gridmap_sensing_ability,
};

pub struct GridmapPlugin;

impl Plugin for GridmapPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(senser_update_fov)
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
                .add_system(finalize_grid_examine_input.in_base_set(CoreSet::PreUpdate))
                .add_system(incoming_messages.in_base_set(CoreSet::PreUpdate))
                .add_event::<InputExamineMap>()
                .init_resource::<GridmapExamineMessages>()
                .add_system(
                    finalize_examine_map
                        .in_base_set(CoreSet::PostUpdate)
                        .before(PostUpdateLabels::EntityUpdate),
                )
                .add_system(examine_grid.after(ActionsLabels::Action))
                .add_system(
                    configure
                        .in_set(ConfigurationLabel::Main)
                        .after(ConfigurationLabel::SpawnEntity),
                )
                .add_system(add_tile_net)
                .add_system(remove_tile_net);
        } else {
            app.add_system(set_cell_graphics)
                .add_startup_system(create_select_cell_cam_state)
                .add_event::<SetYPlanePosition>()
                .add_system(show_ylevel_plane)
                .add_system(set_yplane_position)
                .add_system(input_yplane_position)
                .add_system(move_ylevel_plane)
                .add_system(
                    select_cell_in_front_camera
                        .run_if(on_fixed_timer(Duration::from_secs_f32(1. / 8.))),
                )
                .add_system(update_ghost_cell.in_set(GhostTileLabel::Update))
                .add_event::<ConstructionCellSelectionChanged>()
                .add_system(change_ghost_tile_request)
                .add_system(input_ghost_rotation.after(GhostTileLabel::Update))
                .add_system(client_mouse_click_input)
                .add_system(add_cell_client)
                .add_system(remove_cell_client)
                .add_startup_system(register_input);
        }

        app.add_startup_system(startup_misc_resources.in_set(StartupLabels::MiscResources))
            .add_startup_system(
                startup_map_tile_properties
                    .in_set(StartupLabels::InitDefaultGridmapData)
                    .in_set(BuildingLabels::TriggerBuild)
                    .after(StartupLabels::MiscResources),
            )
            .add_startup_system(
                load_ron_gridmap
                    .in_set(StartupLabels::BuildGridmap)
                    .after(StartupLabels::InitDefaultGridmapData),
            )
            .init_resource::<Gridmap>()
            .init_resource::<DoryenMap>()
            .add_system(add_tile)
            .add_event::<AddTile>()
            .add_event::<AddGroup>()
            .add_system(add_wall_group)
            .add_system(add_tile_collision)
            .add_system(remove_tile)
            .add_event::<RemoveTile>();

        register_reliable_message::<GridmapClientMessage>(app, MessageSender::Client);
        register_reliable_message::<GridmapServerMessage>(app, MessageSender::Server);
    }
}
