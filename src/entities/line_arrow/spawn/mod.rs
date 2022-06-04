pub mod entity_bundle;

use bevy_core::Timer;
use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::Commands,
};

use crate::core::{
    entity::spawn::{DefaultSpawnEvent, SpawnEvent},
    physics::components::{WorldMode, WorldModes},
};

use super::components::{LineArrow, PointArrow};

pub struct LineArrowSummoner {
    pub duration: f32,
}

pub fn summon_line_arrow(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEvent<LineArrowSummoner>>,
) {
    for spawn_event in spawn_events.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert_bundle((
                spawn_event.spawn_data.entity_transform,
                LineArrow,
                PointArrow {
                    timer: Timer::from_seconds(spawn_event.summoner.duration, false),
                },
                WorldMode {
                    mode: WorldModes::Static,
                },
            ));
    }
}

pub fn default_line_arrow(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEvent<LineArrowSummoner>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_name == "lineArrow" {
            spawner.send(SpawnEvent {
                spawn_data: spawn_event.spawn_data.clone(),
                summoner: LineArrowSummoner { duration: 6000.0 },
            });
        }
    }
}