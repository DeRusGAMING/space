use crate::{
    actions::build_actions,
    examine::{
        examine_entity, finalize_entity_examine_input, finalize_examine_entity,
        finalize_examine_map, ExamineEntityMessages, GridmapExamineMessages, NetConnExamine,
        NetExamine,
    },
};
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin};
use bevy::{
    app::CoreStage::{PostUpdate, PreUpdate},
    prelude::SystemSet,
};
use networking::messages::net_system;
use server::labels::{ActionsLabels, PostUpdateLabels, PreUpdateLabels};

pub struct ExaminablePlugin;
impl Plugin for ExaminablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            build_actions
                .label(ActionsLabels::Build)
                .after(ActionsLabels::Init),
        )
        .add_system_to_stage(
            PostUpdate,
            finalize_examine_map.before(PostUpdateLabels::EntityUpdate),
        )
        .add_event::<NetExamine>()
        .add_system_set_to_stage(
            PostUpdate,
            SystemSet::new()
                .after(PostUpdateLabels::VisibleChecker)
                .label(PostUpdateLabels::Net)
                .with_system(net_system::<NetExamine>)
                .with_system(net_system::<NetConnExamine>),
        )
        .add_event::<NetConnExamine>()
        .add_system_to_stage(
            PostUpdate,
            finalize_examine_entity.before(PostUpdateLabels::EntityUpdate),
        )
        .add_system(examine_entity.after(ActionsLabels::Action))
        .init_resource::<ExamineEntityMessages>()
        .init_resource::<GridmapExamineMessages>()
        .add_system_to_stage(
            PreUpdate,
            finalize_entity_examine_input.after(PreUpdateLabels::ProcessInput),
        );
    }
}
