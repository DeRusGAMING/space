use bevy::prelude::{Commands, Entity};
use sfx::builder::{get_random_pitch_scale, Sfx};

#[cfg(feature = "server")]
pub struct UIInteraction1SfxBundle;

#[cfg(feature = "server")]
pub const UI_INTERACTION1_PLAY_BACK_DURATION: f32 = 1.3 + 1.;

#[cfg(feature = "server")]
impl UIInteraction1SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn_bundle((Sfx {
                unit_db: 15.,
                unit_size: 1.,
                stream_id: "/content/audio/ui_interactions/ui_interaction1.sample".to_string(),
                play_back_duration: UI_INTERACTION1_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
