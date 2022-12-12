use bevy::math::{Mat4, Quat, Vec3};
use bevy::prelude::{Commands, EventReader, EventWriter, Transform};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use combat::attack::DEFAULT_INVENTORY_ITEM_DAMAGE;
use entity::entity_data::RawSpawnEvent;
use entity::examine::{Examinable, RichName};
use entity::health::DamageFlag;
use entity::rigid_body::STANDARD_BODY_FRICTION;
use entity::spawn::{
    BaseEntityBundle, BaseEntitySummonable, DefaultSpawnEvent, NoData, SpawnData, SpawnEvent,
};
use entity::spawn_rigidbody::{RigidBodyBundle, RigidBodySummonable};
use inventory_api::core::SlotType;
use inventory_item::combat::{DamageModel, MeleeCombat};
use inventory_item::item::InventoryItem;
use inventory_item::spawn::{InventoryItemBundle, InventoryItemSummonable};
use std::collections::BTreeMap;

use crate::construction_tool::CONSTRUCTION_TOOL_ENTITY_NAME;

use super::construction_tool::ConstructionTool;

#[cfg(feature = "server")]
pub fn get_default_transform() -> Transform {
    Transform::IDENTITY
}

#[cfg(feature = "server")]
impl BaseEntitySummonable<NoData> for ConstructionToolSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoData) -> BaseEntityBundle {
        let mut examine_map = BTreeMap::new();
        examine_map.insert(
            0,
            "A construction tool. Use this to construct or deconstruct ship hull cells."
                .to_string(),
        );
        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "admin construction tool".to_string(),
                    n: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            entity_name: CONSTRUCTION_TOOL_ENTITY_NAME.to_string(),
            ..Default::default()
        }
    }
}
use std::collections::HashMap;

#[cfg(feature = "server")]
impl InventoryItemSummonable for ConstructionToolSummoner {
    fn get_bundle(&self, spawn_data: &SpawnData) -> InventoryItemBundle {
        let mut attachment_transforms = HashMap::new();
        attachment_transforms.insert(
            "left_hand".to_string(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::from_axis_angle(Vec3::new(0.0697873, -0.966557, -0.246774), 1.8711933),
                Vec3::new(-0.047, 0.024, -0.035),
            )),
        );
        attachment_transforms.insert(
            "right_hand".to_string(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::from_axis_angle(Vec3::new(-0.1942536, 0.9779768, 0.076334), 2.1748603),
                Vec3::new(0.042, -0., -0.021),
            )),
        );
        attachment_transforms.insert(
            "holster".to_string(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::from_axis_angle(Vec3::new(-0.6264298, -0.1219246, 0.7698832), 2.4247889),
                Vec3::new(0., -0.093, 0.036),
            )),
        );

        let mut melee_damage_flags = HashMap::new();
        melee_damage_flags.insert(0, DamageFlag::SoftDamage);

        InventoryItemBundle {
            inventory_item: InventoryItem {
                in_inventory_of_entity: spawn_data.holder_entity_option,
                drop_transform: get_default_transform(),
                attachment_transforms: attachment_transforms.clone(),
                slot_type: SlotType::Holster,
                ..Default::default()
            },
            melee_combat: MeleeCombat {
                combat_melee_damage_model: DamageModel {
                    brute: DEFAULT_INVENTORY_ITEM_DAMAGE,
                    damage_flags: melee_damage_flags.clone(),
                    ..Default::default()
                },
                ..Default::default()
            },
            projectile_combat_option: None,
        }
    }
}

#[cfg(feature = "server")]
impl RigidBodySummonable<NoData> for ConstructionToolSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(STANDARD_BODY_FRICTION);
        friction.combine_rule = CoefficientCombineRule::Multiply;

        RigidBodyBundle {
            collider: Collider::cuboid(0.11 * 1.5, 0.1 * 1.5, 0.13 * 1.5),
            collider_transform: Transform::from_translation(Vec3::new(0., 0.087, 0.)),
            collider_friction: friction,
            ..Default::default()
        }
    }
}

#[cfg(feature = "server")]
pub struct ConstructionToolSummoner;

#[cfg(feature = "server")]
pub fn summon_construction_tool<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEvent<T>>,
) {
    for spawn_event in spawn_events.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(ConstructionTool::default());
    }
}

#[cfg(feature = "server")]
pub fn summon_raw_construction_tool(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_computer: EventWriter<SpawnEvent<ConstructionToolSummoner>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != CONSTRUCTION_TOOL_ENTITY_NAME {
            continue;
        }

        let mut entity_transform = Transform::from_translation(spawn_event.raw_entity.translation);
        entity_transform.rotation = spawn_event.raw_entity.rotation;
        entity_transform.scale = spawn_event.raw_entity.scale;
        summon_computer.send(SpawnEvent {
            spawn_data: SpawnData {
                entity_transform: entity_transform,
                default_map_spawn: true,
                entity_name: spawn_event.raw_entity.entity_type.clone(),
                entity: commands.spawn(()).id(),
                raw_entity_option: Some(spawn_event.raw_entity.clone()),
                ..Default::default()
            },
            summoner: ConstructionToolSummoner,
        });
    }
}

#[cfg(feature = "server")]
pub fn default_summon_construction_tool(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEvent<ConstructionToolSummoner>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_name != CONSTRUCTION_TOOL_ENTITY_NAME {
            continue;
        }
        spawner.send(SpawnEvent {
            spawn_data: spawn_event.spawn_data.clone(),
            summoner: ConstructionToolSummoner,
        });
    }
}
