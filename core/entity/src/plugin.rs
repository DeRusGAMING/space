use api::data::{PostUpdateLabels, StartupLabels, SummoningLabels};
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};
use bevy::time::FixedTimestep;
use console_commands::commands::ConsoleCommandsLabels;
use networking::messages::net_system;

use crate::entity_data::{RawSpawnEvent, INTERPOLATION_LABEL1};
use crate::init::{initialize_console_commands, startup_entities};
use crate::meta::EntityDataResource;
use crate::spawn::DefaultSpawnEvent;

use super::entity_data::{broadcast_position_updates, NetShowcase};
use bevy::app::CoreStage::PostUpdate;

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NetShowcase>()
            .init_resource::<EntityDataResource>()
            .add_event::<RawSpawnEvent>()
            .add_event::<DefaultSpawnEvent>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(
                        FixedTimestep::step(1. / 2.).with_label(INTERPOLATION_LABEL1),
                    )
                    .with_system(broadcast_position_updates),
            )
            .add_startup_system(
                startup_entities
                    .before(StartupLabels::BuildGridmap)
                    .label(StartupLabels::InitEntities),
            )
            .add_startup_system(
                initialize_console_commands
                    .before(ConsoleCommandsLabels::Finalize)
                    .label(SummoningLabels::TriggerSummon),
            )
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net)
                    .with_system(net_system::<NetShowcase>),
            );
    }
}
