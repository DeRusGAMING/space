use basic_console_commands::register::register_basic_console_commands_for_type;
use bevy::prelude::{App, CoreSet, IntoSystemConfig, Plugin};
use construction_tool::spawn::ConstructionToolType;
use entity::{base_mesh::link_base_mesh, entity_types::register_entity_type, loading::load_entity};

use inventory::server::inventory::SpawnItemLabel;
use physics::spawn::build_rigid_bodies;
use resources::{
    is_server::is_server,
    labels::{BuildingLabels, CombatLabels},
};

use crate::{
    boarding::spawn_boarding_player,
    hands_attack_handler::hands_attack_handler,
    setup_ui_showcase::human_male_setup_ui,
    spawn::{build_base_human_males, build_human_males, spawn_held_item, HumanMaleType},
};
pub struct HumanMalePlugin;

impl Plugin for HumanMalePlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(
                hands_attack_handler
                    .in_set(CombatLabels::WeaponHandler)
                    .after(CombatLabels::CacheAttack),
            )
            .add_system(human_male_setup_ui.in_set(BuildingLabels::TriggerBuild))
            .add_system(spawn_boarding_player.in_base_set(CoreSet::PostUpdate));
        } else {
            app.add_system(link_base_mesh::<HumanMaleType>)
                .add_system(load_entity::<HumanMaleType>);
        }
        register_entity_type::<HumanMaleType>(app);
        register_basic_console_commands_for_type::<HumanMaleType>(app);
        app.add_system(
            build_human_males
                .before(BuildingLabels::TriggerBuild)
                .in_set(BuildingLabels::NormalBuild),
        )
        .add_system((build_base_human_males::<HumanMaleType>).after(BuildingLabels::TriggerBuild))
        .add_system((build_rigid_bodies::<HumanMaleType>).after(BuildingLabels::TriggerBuild))
        .add_system(spawn_held_item::<ConstructionToolType>.in_set(SpawnItemLabel::SpawnHeldItem));
    }
}
