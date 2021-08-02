
use bevy::prelude::{Transform};

use crate::space_core::components::{cached_broadcast_transform::CachedBroadcastTransform, entity_data::{EntityData}, entity_updates::EntityUpdates, footsteps_walking::FootstepsWalking, repeating_sfx::RepeatingSfx, sensable::Sensable, static_transform::StaticTransform, update_transform::UpdateTransform};

pub struct FootstepsSprintingSfxBundle;

impl FootstepsSprintingSfxBundle {

    pub fn new(passed_transform : Transform) -> (
        StaticTransform,
        EntityData,
        Sensable,
        RepeatingSfx,
        EntityUpdates,
        FootstepsWalking,
        UpdateTransform,
        CachedBroadcastTransform
    ) {


        (StaticTransform {
            transform: passed_transform,
        },
        EntityData {
            entity_class : "RepeatingSFX".to_string(),
            ..Default::default()
        },
        Sensable {
            is_audible: true,
            ..Default::default()
        },
        RepeatingSfx {
            unit_db: 18.0,
            unit_size: 1.,
            stream_id: "concrete_sprinting_footsteps".to_string(),
            auto_destroy : true,
            repeat_time: 0.35,
            ..Default::default()
        },
        EntityUpdates::default(),
        FootstepsWalking,
        UpdateTransform,
        CachedBroadcastTransform::default(),
        )

    }

}
