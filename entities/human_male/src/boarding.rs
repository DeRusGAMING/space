use crate::spawn::HumanMaleSummoner;
use bevy::prelude::{Added, Commands, Entity, EventWriter, Query, ResMut};
use construction_tool_admin::construction_tool::CONSTRUCTION_TOOL_ENTITY_NAME;
use entity::spawn::{SpawnData, SpawnEvent};
use helmet_security::helmet::HELMET_SECURITY_ENTITY_NAME;
use humanoid::humanoid::HUMAN_MALE_ENTITY_NAME;
use jumpsuit_security::jumpsuit::JUMPSUIT_SECURITY_ENTITY_NAME;
use networking::messages::PendingMessage;
use networking::messages::PendingNetworkMessage;
use networking::messages::{ReliableServerMessage, ServerConfigMessage};
use networking_macros::NetMessage;
use pawn::pawn::{PawnDesignation, PersistentPlayerData, Spawning, UsedNames};
use pistol_l1::pistol_l1::PISTOL_L1_ENTITY_NAME;
use player_controller::connection::SpawnPawnData;
use server::core::{ConnectedPlayer, HandleToEntity};

/// Spawn player as human male with preset inventory.
pub(crate) fn on_spawning(
    mut net_on_new_player_connection: EventWriter<NetOnSpawning>,
    query: Query<(Entity, &Spawning, &ConnectedPlayer, &PersistentPlayerData), Added<Spawning>>,
    mut commands: Commands,
    mut handle_to_entity: ResMut<HandleToEntity>,
    mut used_names: ResMut<UsedNames>,
    mut summon_human_male: EventWriter<SpawnEvent<HumanMaleSummoner>>,
) {
    for (
        entity_id,
        spawning_component,
        connected_player_component,
        persistent_player_data_component,
    ) in query.iter()
    {
        let passed_inventory_setup = vec![
            (
                "jumpsuit".to_string(),
                JUMPSUIT_SECURITY_ENTITY_NAME.to_string(),
            ),
            (
                "helmet".to_string(),
                HELMET_SECURITY_ENTITY_NAME.to_string(),
            ),
            ("holster".to_string(), PISTOL_L1_ENTITY_NAME.to_string()),
            (
                "left_hand".to_string(),
                CONSTRUCTION_TOOL_ENTITY_NAME.to_string(),
            ),
        ];

        let new_entity = commands.spawn().id();

        summon_human_male.send(SpawnEvent {
            spawn_data: SpawnData {
                entity: new_entity,
                entity_transform: spawning_component.transform,
                entity_name: HUMAN_MALE_ENTITY_NAME.to_string(),
                ..Default::default()
            },
            summoner: HumanMaleSummoner {
                character_name: persistent_player_data_component.character_name.clone(),
                user_name: persistent_player_data_component.user_name.clone(),
                spawn_pawn_data: SpawnPawnData {
                    persistent_player_data: persistent_player_data_component.clone(),
                    connected_player_option: Some(connected_player_component.clone()),
                    inventory_setup: passed_inventory_setup,
                    designation: PawnDesignation::Player,
                },
            },
        });

        let handle = *handle_to_entity.inv_map.get(&entity_id).unwrap();

        handle_to_entity.inv_map.remove(&entity_id);
        handle_to_entity.inv_map.insert(new_entity, handle);

        handle_to_entity.map.remove(&handle);
        handle_to_entity.map.insert(handle, new_entity);

        used_names.names.insert(
            persistent_player_data_component.character_name.clone(),
            new_entity,
        );

        commands.entity(entity_id).despawn();

        net_on_new_player_connection.send(NetOnSpawning {
            handle: handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::EntityId(
                new_entity.to_bits(),
            )),
        });
    }
}

#[derive(NetMessage)]
pub(crate) struct NetOnSpawning {
    pub handle: u64,
    pub message: ReliableServerMessage,
}