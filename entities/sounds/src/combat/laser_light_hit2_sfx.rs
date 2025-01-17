use bevy::prelude::{Commands, Entity};
use resources::content::SF_CONTENT_PREFIX;
use sfx::builder::{get_random_pitch_scale, Sfx};

pub struct LaserLightHit2Bundle;

pub const LASER_LIGHT_HIT2_PLAY_BACK_DURATION: f32 = 3. + 1.;

impl LaserLightHit2Bundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 25.,
                unit_size: 1.,
                stream_id: SF_CONTENT_PREFIX.to_string() + "laser_light_hit2",

                play_back_duration: LASER_LIGHT_HIT2_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
