use api::{console_commands::ConsoleCommandVariant, data::EntityDataResource};
use bevy::prelude::{info, Res, ResMut};
use console_commands::commands::AllConsoleCommands;

/// Print startup entity data to console.
pub(crate) fn startup_entities(entity_data: Res<EntityDataResource>) {
    info!("Loaded {} different entity types.", entity_data.data.len());
}

/// Initialize console commands.
pub(crate) fn initialize_console_commands(mut commands: ResMut<AllConsoleCommands>) {
    commands.list.push((
        "spawn".to_string(),
        "For server administrators only. Spawn in entities in proximity.".to_string(),
        vec![
            ("entity_name".to_string(), ConsoleCommandVariant::String),
            ("amount".to_string(), ConsoleCommandVariant::Int),
            ("player_selector".to_string(), ConsoleCommandVariant::String),
        ],
    ));
}
