use bevy::prelude::{Commands, Entity};
use resources::content::SF_CONTENT_PREFIX;
use sfx::builder::{get_random_pitch_scale, Sfx};

pub struct Throw2SfxBundle;

pub const THROW2_PLAY_BACK_DURATION: f32 = 0.5 + 1.;

impl Throw2SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 15.,
                unit_size: 1.,
                stream_id: SF_CONTENT_PREFIX.to_string() + "throw2",

                play_back_duration: THROW2_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
