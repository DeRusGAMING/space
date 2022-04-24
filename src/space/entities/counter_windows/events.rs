use bevy_ecs::entity::Entity;

use crate::space::core::{
    entity::components::EntityGroup, networking::resources::ReliableServerMessage,
};

pub struct CounterWindowSensorCollision {
    pub collider1_entity: Entity,
    pub collider2_entity: Entity,

    pub collider1_group: EntityGroup,
    pub collider2_group: EntityGroup,

    pub started: bool,
}

pub struct InputCounterWindowToggleOpen {
    pub handle_option: Option<u32>,

    pub opener: Entity,
    pub opened: u64,
}
pub struct CounterWindowLockOpen {
    pub handle_option: Option<u32>,

    pub locked: Entity,
    pub locker: Entity,
}

pub struct CounterWindowLockClosed {
    pub handle_option: Option<u32>,

    pub locked: Entity,
    pub locker: Entity,
}

pub struct CounterWindowUnlock {
    pub handle_option: Option<u32>,

    pub locked: Entity,
    pub locker: Entity,
}

pub struct NetCounterWindow {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

use bevy_app::EventWriter;
use bevy_ecs::system::Res;

use crate::space::core::tab_actions::resources::QueuedTabActions;

pub fn counter_windows_actions(
    queue: Res<QueuedTabActions>,

    mut counter_window_toggle_open_event: EventWriter<InputCounterWindowToggleOpen>,
    mut counter_window_lock_open_event: EventWriter<CounterWindowLockOpen>,
    mut counter_window_lock_closed_event: EventWriter<CounterWindowLockClosed>,
    mut counter_window_unlock_event: EventWriter<CounterWindowUnlock>,
) {
    for queued in queue.queue.iter() {
        if queued.tab_id == "actions::counter_windows/toggleopen" {
            if queued.target_entity_option.is_some() {
                counter_window_toggle_open_event.send(InputCounterWindowToggleOpen {
                    opener: queued.player_entity,
                    opened: queued.target_entity_option.unwrap(),
                    handle_option: queued.handle_option,
                });
            }
        } else if queued.tab_id == "actions::counter_windows/lockopen" {
            if queued.target_entity_option.is_some() {
                counter_window_lock_open_event.send(CounterWindowLockOpen {
                    locked: Entity::from_bits(queued.target_entity_option.unwrap()),
                    locker: queued.player_entity,
                    handle_option: queued.handle_option,
                });
            }
        } else if queued.tab_id == "actions::counter_windows/lockclosed" {
            if queued.target_entity_option.is_some() {
                counter_window_lock_closed_event.send(CounterWindowLockClosed {
                    locked: Entity::from_bits(queued.target_entity_option.unwrap()),
                    locker: queued.player_entity,
                    handle_option: queued.handle_option,
                });
            }
        } else if queued.tab_id == "actions::counter_windows/unlock" {
            if queued.target_entity_option.is_some() {
                counter_window_unlock_event.send(CounterWindowUnlock {
                    locked: Entity::from_bits(queued.target_entity_option.unwrap()),
                    locker: queued.player_entity,
                    handle_option: queued.handle_option,
                });
            }
        }
    }
}
