use bevy::prelude::{Commands, Entity};
use resources::content::SF_CONTENT_PREFIX;
use sfx::builder::{get_random_pitch_scale, Sfx};

pub struct Swing1SfxBundle;

pub const SWING1_PLAY_BACK_DURATION: f32 = 0.5 + 1.;

impl Swing1SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 12.,
                unit_size: 1.,
                stream_id: SF_CONTENT_PREFIX.to_string() + "swing1",

                play_back_duration: SWING1_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
