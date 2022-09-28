use bevy::{
    math::Quat,
    prelude::{Component, Transform},
};
use bevy_rapier3d::na::Quaternion;
use serde::Deserialize;

use crate::converters::string_transform_to_transform;

pub struct PawnYAxisRotations;

impl PawnYAxisRotations {
    pub fn new() -> Vec<Quaternion<f32>> {
        vec![
            //0deg
            Quaternion::new(1., 0., 0., 0.),
            //45deg
            Quaternion::new(0.9238795, 0., 0.3826834, 0.),
            //90deg
            Quaternion::new(
                std::f32::consts::FRAC_1_SQRT_2,
                0.,
                std::f32::consts::FRAC_1_SQRT_2,
                0.,
            ),
            //135deg
            Quaternion::new(0.3826834, 0., 0.9238795, 0.),
            //180deg
            Quaternion::new(0., 0., 1., 0.),
            //225deg
            Quaternion::new(-0.3826834, 0., 0.9238795, 0.),
            //270deg
            Quaternion::new(
                -std::f32::consts::FRAC_1_SQRT_2,
                0.,
                std::f32::consts::FRAC_1_SQRT_2,
                0.,
            ),
            //315deg
            Quaternion::new(-0.9238795, 0., 0.3826834, 0.),
        ]
    }
}

#[derive(Clone)]
pub enum PawnDesignation {
    Showcase,
    Player,
    Dummy,
    Ai,
}
/// Component that contains the spawn data of a to-be-spawned entity.
#[derive(Component)]
pub struct Spawning {
    pub transform: Transform,
}

/// A spawn point in which players will spawn.
pub struct SpawnPoint {
    pub point_type: String,
    pub transform: Transform,
}

impl SpawnPoint {
    pub fn new(raw: &SpawnPointRaw) -> SpawnPoint {
        let mut this_transform = string_transform_to_transform(&raw.transform);

        this_transform.translation.y = 0.05;

        this_transform.rotation = Quat::IDENTITY;

        SpawnPoint {
            point_type: raw.point_type.clone(),
            transform: this_transform,
        }
    }
}

/// Raw json.
#[derive(Deserialize)]
pub struct SpawnPointRaw {
    pub point_type: String,
    pub transform: String,
}
/// Resource containing all available spawn points for players.
#[derive(Default)]
pub struct SpawnPoints {
    pub list: Vec<SpawnPoint>,
    pub i: usize,
}
/// How far an entity can reach ie with picking up items.
pub const REACH_DISTANCE: f32 = 3.;
