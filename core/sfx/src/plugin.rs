use std::env;

use bevy::{
    prelude::{App, Plugin, SystemSet},
    time::FixedTimestep,
};
use entity::entity_data::INTERPOLATION_LABEL1;
use resources::labels::PostUpdateLabels;

use crate::{entity_update::SfxAutoDestroyTimers, timers::free_sfx};

use super::{
    entity_update::{repeating_sfx_update, sfx_update},
    timers::tick_timers_slowed,
};
use bevy::app::CoreStage::PostUpdate;

pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
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
    }
}
