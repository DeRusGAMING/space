use bevy_ecs::{entity::Entity, event::EventWriter, system::ResMut};

use crate::core::{
    console_commands::events::NetConsoleCommands, networking::resources::ReliableServerMessage,
    pawn::resources::UsedNames,
};

pub fn player_selector_to_entities(
    command_executor_entity: Entity,
    command_executor_handle_option: Option<u32>,
    player_selector: &str,
    used_names: &mut ResMut<UsedNames>,
    net_console_commands: &mut EventWriter<NetConsoleCommands>,
) -> Vec<Entity> {
    if player_selector == "*" {
        return used_names.names.values().copied().collect();
    } else if player_selector == "@me" {
        return vec![command_executor_entity];
    }

    let matching_names: Vec<_> = used_names
        .names
        .iter()
        .filter(|(player_name, _)| {
            let player_selector = player_selector.to_lowercase();
            player_name.to_lowercase().contains(&player_selector)
        })
        .collect();

    let message = match &matching_names[..] {
        [(_, &entity)] => return vec![entity],
        [] => {
            format!("Couldn't find player \"{player_selector}\"")
        }
        [conflicts @ ..] => {
            let mut names = String::new();
            for (name, _entity) in conflicts.iter() {
                names.push_str(name);
                names.push('\n');
            }

            format!("Player selector \"{player_selector}\" is not specific enough.\n{names}")
        }
    };
    if let Some(handle) = command_executor_handle_option {
        net_console_commands.send(NetConsoleCommands {
            handle,
            message: ReliableServerMessage::ConsoleWriteLine(format!(
                "[color=#ff6600]{message}[/color]"
            )),
        });
    }
    vec![]
}
