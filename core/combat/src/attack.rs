use std::collections::HashMap;

use bevy::{math::Vec3, prelude::Entity};
use entity::health::{DamageFlag, HealthFlag};
use resources::math::Vec3Int;

/// The attack event.
#[derive(Clone)]

pub struct Attack {
    /// Attacker entity id.
    pub attacker: Entity,
    /// Used weapon option.
    pub weapon_option: Option<Entity>,
    /// Targetted entity.
    pub targetted_entity: Option<Entity>,
    /// Targetted cell.
    pub targetted_cell: Option<Vec3Int>,
    /// Attack id.
    pub incremented_id: u64,
    /// Attack angle.
    pub angle: f32,
    /// Targetted limb.
    pub targetted_limb: String,
    /// Whether alt attack mode is enabled, so that you can melee with a projectile weapon in a different attack mode.
    pub alt_attack_mode: bool,
}

/// Hit results generated by physics queries as an event.
#[derive(Clone)]

pub struct QueryCombatHitResult {
    /// Attack id.
    pub incremented_id: u64,
    pub entities_hits: Vec<EntityHitSimple>,
    pub cell_hits: Vec<CellHitSimple>,
}

/// Entity hit for combat and related physics queries.
#[derive(Clone)]

pub struct EntityHitSimple {
    pub entity: Entity,
    pub hit_point: Vec3,
}
/// Cell hit for  combat and related physics queries.
#[derive(Clone)]

pub struct CellHitSimple {
    pub cell: Vec3Int,
    pub hit_point: Vec3,
}

/// Type of damage.

pub enum DamageType {
    Melee,
    Projectile,
}

/// Represents the hit result of a combat physics query.
#[allow(dead_code)]

pub enum HitResult {
    HitSoft,
    Blocked,
    Missed,
}

/// Combat type.
#[derive(Clone, Debug)]

pub enum CombatType {
    MeleeDirect,
    Projectile,
}

/// Contains (visual graphics) data of laser projectiles.
#[derive(Clone, Debug)]

pub enum ProjectileType {
    Laser((f32, f32, f32, f32), f32, f32, f32),
}

pub const DEFAULT_INVENTORY_ITEM_DAMAGE: f32 = 9.;

/// General function for returning the results of damage application.

pub fn calculate_damage(
    health_flags: &HashMap<u32, HealthFlag>,
    damage_flags: &HashMap<u32, DamageFlag>,

    brute: &f32,
    burn: &f32,
    toxin: &f32,
) -> (f32, f32, f32, HitResult) {
    let mut output_brute = brute.clone();
    let mut output_burn = burn.clone();
    let output_toxin = toxin.clone();

    let mut hit_result = HitResult::HitSoft;

    let mut damager_flags = vec![];

    for damage_flag in damage_flags.values() {
        damager_flags.push(damage_flag);
    }

    let mut structure_health_flags = vec![];

    for stucture_health_flag in health_flags.values() {
        structure_health_flags.push(stucture_health_flag);
    }

    let is_armour_plated = structure_health_flags.contains(&&HealthFlag::ArmourPlated);

    if damager_flags.contains(&&DamageFlag::SoftDamage) && is_armour_plated {
        output_brute = 0.;
        hit_result = HitResult::Blocked;
    } else if damager_flags.contains(&&DamageFlag::WeakLethalLaser) && is_armour_plated {
        output_burn *= 0.05;
        hit_result = HitResult::Blocked;
    }

    (output_brute, output_burn, output_toxin, hit_result)
}
