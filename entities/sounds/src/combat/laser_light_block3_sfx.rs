use bevy::prelude::{Commands, Entity};
use resources::content::SF_CONTENT_PREFIX;
use sfx::builder::{get_random_pitch_scale, Sfx};

pub struct LaserLightBlock3Bundle;

pub const LASER_LIGHT_BLOCK3_PLAY_BACK_DURATION: f32 = 0.7 + 1.;

impl LaserLightBlock3Bundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 15.,
                unit_size: 1.,
                stream_id: SF_CONTENT_PREFIX.to_string() + "laser_light_block3",

                play_back_duration: LASER_LIGHT_BLOCK3_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
