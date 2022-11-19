use bevy::prelude::{info, ResMut, Resource};
use bevy::prelude::{Entity, SystemLabel};
use networking::server::PendingNetworkMessage;
use networking::server::{GodotVariant, ReliableServerMessage};
use networking::server::{GodotVariantValues, PendingMessage};
use networking_macros::NetMessage;

#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetConsoleCommands {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
/// Resource containing all registered custom console commands.
#[derive(Default, Resource)]
#[cfg(feature = "server")]
pub struct AllConsoleCommands {
    pub list: Vec<(String, String, Vec<(String, GodotVariant)>)>,
}
/// Initialize console commands.
#[cfg(feature = "server")]
pub fn initialize_console_commands(mut commands: ResMut<AllConsoleCommands>) {
    commands.list.push((
        "rcon".to_string(),
        "For server administrators only. Obtaining rcon status allows for usage of rcon_* commands"
            .to_string(),
        vec![("password".to_string(), GodotVariant::String)],
    ));

    commands.list.push((
        "rconStatus".to_string(),
        "For server administrators only. Check if the server has granted you the RCON status."
            .to_string(),
        vec![],
    ));

    info!("Loaded {} different console commands.", commands.list.len());
}

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
#[cfg(feature = "server")]
pub enum ConsoleCommandsLabels {
    Finalize,
}

/// Client input console command message event.
#[cfg(feature = "server")]
pub struct InputConsoleCommand {
    /// The connection handle tied to the entity performing the command.
    pub handle_option: Option<u64>,
    /// The entity performing the command.
    pub entity: Entity,
    /// The command name.
    pub command_name: String,
    /// The passed arguments to the command as variants.
    pub command_arguments: Vec<GodotVariantValues>,
}
