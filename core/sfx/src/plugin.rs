use bevy::{
    prelude::{App, Plugin, SystemSet},
    time::FixedTimestep,
};
use entity::entity_data::INTERPOLATION_LABEL1;
use networking::messaging::{init_reliable_message, MessageSender};
use resources::{is_server::is_server, labels::PostUpdateLabels};

use crate::{entity_update::SfxAutoDestroyTimers, networking::SfxServerMessage, timers::free_sfx};

use super::{
    entity_update::{repeating_sfx_update, sfx_update},
    timers::tick_timers_slowed,
};
use bevy::app::CoreStage::PostUpdate;

pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system_set(
                SystemSet::new()
                    .with_run_criteria(
                        FixedTimestep::step(1. / 2.).with_label(INTERPOLATION_LABEL1),
                    )
                    .with_system(tick_timers_slowed),
            )
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .label(PostUpdateLabels::EntityUpdate)
                    .with_system(sfx_update)
                    .with_system(repeating_sfx_update),
            )
            .add_system(free_sfx)
            .init_resource::<SfxAutoDestroyTimers>();
        }
        init_reliable_message::<SfxServerMessage>(app, MessageSender::Server);
    }
}
