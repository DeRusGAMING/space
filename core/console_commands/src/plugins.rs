use std::env;

use bevy::prelude::{App, IntoSystemDescriptor, Plugin, SystemSet};
use networking::server::net_system;
use player::plugin::ConfigurationLabel;
use resources::labels::{PostUpdateLabels, StartupLabels};

use crate::{
    commands::{AllConsoleCommands, ConsoleCommandsLabels, InputConsoleCommand},
    connections::{configure, NetConfigure},
    init::initialize_console_commands,
    networking::incoming_messages,
};
use bevy::app::CoreStage::PostUpdate;

use super::commands::NetConsoleCommands;
use bevy::app::CoreStage::PreUpdate;
#[derive(Default)]
pub struct ConsoleCommandsPlugin;

impl Plugin for ConsoleCommandsPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.init_resource::<AllConsoleCommands>()
                .add_event::<NetConsoleCommands>()
                .add_startup_system(
                    initialize_console_commands
                        .label(ConsoleCommandsLabels::Finalize)
                        .label(StartupLabels::ConsoleCommands),
                )
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .after(PostUpdateLabels::VisibleChecker)
                        .label(PostUpdateLabels::Net)
                        .with_system(net_system::<NetConsoleCommands>)
                        .with_system(net_system::<NetConfigure>),
                )
                .add_system_to_stage(PreUpdate, incoming_messages)
                .add_event::<InputConsoleCommand>()
                .add_event::<NetConfigure>()
                .add_system(
                    configure
                        .label(ConfigurationLabel::Main)
                        .after(ConfigurationLabel::SpawnEntity),
                );
        }
    }
}
