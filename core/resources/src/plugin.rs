use std::time::Duration;

use bevy::{
    app::ScheduleRunnerSettings,
    prelude::{App, Plugin},
};

use crate::{binds::KeyBinds, core::TickRate, is_server::is_server};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.init_resource::<KeyBinds>();
        }
        app.init_resource::<TickRate>();
        let rate = TickRate::default();
        app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f32(
            1. / rate.bevy_rate as f32,
        )));
    }
}
