use bevy_ecs::{entity::Entity, system::Commands};

use crate::core::sfx::components::{get_random_pitch_scale, Sfx};

pub struct Swing4SfxBundle;

pub const SWING4_PLAY_BACK_DURATION: f32 = 0.5 + 1.;

impl Swing4SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn_bundle((Sfx {
                unit_db: 12.,
                unit_size: 1.,
                stream_id: "/content/audio/combat/swing4.sample".to_string(),
                play_back_duration: SWING4_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
