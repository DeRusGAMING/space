use bevy::prelude::{Commands, Entity};
use resources::content::SF_CONTENT_PREFIX;
use sfx::builder::{get_random_pitch_scale, Sfx};

pub struct LaserLightHit4Bundle;

pub const LASER_LIGHT_HIT4_PLAY_BACK_DURATION: f32 = 2.1 + 1.;

impl LaserLightHit4Bundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 25.,
                unit_size: 1.,
                stream_id: SF_CONTENT_PREFIX.to_string() + "laser_light_hit4",

                play_back_duration: LASER_LIGHT_HIT4_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
