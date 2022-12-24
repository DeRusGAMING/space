use bevy::prelude::{App, IntoSystemDescriptor, Plugin, ResMut};
use combat::melee_queries::melee_attack_handler;
use combat::sfx::{attack_sfx, health_combat_hit_result_sfx};
use entity::entity_data::initialize_entity_data;
use entity::entity_types::init_entity_type;
use entity::meta::{EntityDataProperties, EntityDataResource};
use entity::spawn::build_base_entities;
use inventory::spawn_item::build_inventory_items;
use physics::spawn::build_rigid_bodies;
use resources::is_server::is_server;
use resources::labels::{ActionsLabels, BuildingLabels, CombatLabels, StartupLabels, UpdateLabels};

use crate::action::{
    build_actions, construct_action_prequisite_check, construction_tool_actions,
    construction_tool_is_holding_item_prequisite_check,
    construction_tool_search_distance_prequisite_check, deconstruct_action_prequisite_check,
    text_tree_input_selection,
};
use crate::construction_tool::{ConstructionTool, InputConstructionOptionsSelection};

use super::{
    construction_tool::{
        construction_tool, InputConstruct, InputConstructionOptions, InputDeconstruct,
    },
    spawn::{
        build_construction_tools, build_raw_construction_tools, default_build_construction_tools,
        ConstructionToolType,
    },
};

pub struct ConstructionToolAdminPlugin;

impl Plugin for ConstructionToolAdminPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_event::<InputConstruct>()
                .add_event::<InputDeconstruct>()
                .add_event::<InputConstructionOptions>()
                .add_event::<InputConstructionOptionsSelection>()
                .add_system(
                    construction_tool
                        .after(UpdateLabels::TextTreeInputSelection)
                        .before(UpdateLabels::DeconstructCell),
                )
                .add_system(
                    melee_attack_handler::<ConstructionTool>
                        .label(CombatLabels::WeaponHandler)
                        .after(CombatLabels::CacheAttack),
                )
                .add_system(
                    attack_sfx::<ConstructionTool>
                        .after(CombatLabels::WeaponHandler)
                        .after(CombatLabels::Query),
                )
                .add_system(
                    health_combat_hit_result_sfx::<ConstructionTool>
                        .after(CombatLabels::FinalizeApplyDamage),
                )
                .add_system(
                    construction_tool_is_holding_item_prequisite_check
                        .label(ActionsLabels::Approve)
                        .after(ActionsLabels::Build),
                )
                .add_system(
                    construction_tool_search_distance_prequisite_check
                        .label(ActionsLabels::Approve)
                        .after(ActionsLabels::Build),
                )
                .add_system(
                    deconstruct_action_prequisite_check
                        .label(ActionsLabels::Approve)
                        .after(ActionsLabels::Build),
                )
                .add_system(
                    construct_action_prequisite_check
                        .label(ActionsLabels::Approve)
                        .after(ActionsLabels::Build),
                )
                .add_system(
                    construction_tool_actions
                        .label(ActionsLabels::Action)
                        .after(ActionsLabels::Approve),
                )
                .add_system(
                    build_actions
                        .label(ActionsLabels::Build)
                        .after(ActionsLabels::Init),
                )
                .add_system(text_tree_input_selection.label(UpdateLabels::TextTreeInputSelection));
        }
        init_entity_type::<ConstructionToolType>(app);
        app.add_startup_system(content_initialization.before(StartupLabels::InitEntities))
            .add_system(
                build_construction_tools::<ConstructionToolType>
                    .after(BuildingLabels::TriggerBuild),
            )
            .add_system(
                (build_base_entities::<ConstructionToolType>).after(BuildingLabels::TriggerBuild),
            )
            .add_system(
                (build_rigid_bodies::<ConstructionToolType>).after(BuildingLabels::TriggerBuild),
            )
            .add_system(
                (build_inventory_items::<ConstructionToolType>).after(BuildingLabels::TriggerBuild),
            )
            .add_system((build_raw_construction_tools).after(BuildingLabels::TriggerBuild))
            .add_system(
                (default_build_construction_tools)
                    .label(BuildingLabels::DefaultBuild)
                    .after(BuildingLabels::NormalBuild),
            );
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: ConstructionToolType::default().identifier,
        id: entity_data.get_id_inc(),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
