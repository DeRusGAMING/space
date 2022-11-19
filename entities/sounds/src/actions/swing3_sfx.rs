use bevy::prelude::{Commands, Entity};
use sfx::builder::{get_random_pitch_scale, Sfx};

#[cfg(feature = "server")]
pub struct Swing3SfxBundle;

#[cfg(feature = "server")]
pub const SWING3_PLAY_BACK_DURATION: f32 = 0.5 + 1.;

#[cfg(feature = "server")]
impl Swing3SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 12.,
                unit_size: 1.,
                stream_id: "/content/audio/combat/swing3.sample".to_string(),
                play_back_duration: SWING3_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
