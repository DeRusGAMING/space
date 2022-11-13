use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin};
use server_instance::labels::ActionsLabels;

use crate::actions::{examine, examine_prerequisite_check};

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(feature = "server") {
            app.add_system(
                examine_prerequisite_check
                    .label(ActionsLabels::Approve)
                    .after(ActionsLabels::Init),
            )
            .add_system(
                examine
                    .label(ActionsLabels::Action)
                    .after(ActionsLabels::Approve),
            );
        }
    }
}
