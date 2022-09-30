use bevy::prelude::{Component, Entity};
use doryen_fov::FovRecursiveShadowCasting;
use math::grid::Vec2Int;

/// Used to check if entities are authorized to examine and obtain certain additional data.
#[derive(PartialEq)]
pub enum SensingAbility {
    AtmosphericsSensor,
    ShipEngineerKnowledge,
}

/// The component of entities that can sense other entities.
#[derive(Component)]
pub struct Senser {
    pub cell_id: Vec2Int,
    pub fov: FovRecursiveShadowCasting,
    pub sensing: Vec<Entity>,
    pub sfx: Vec<Entity>,
    pub sensing_abilities: Vec<SensingAbility>,
}

impl Default for Senser {
    fn default() -> Self {
        Self {
            cell_id: Vec2Int { x: 0, y: 0 },
            fov: FovRecursiveShadowCasting::new(FOV_MAP_WIDTH, FOV_MAP_WIDTH),
            sensing: vec![],
            sfx: vec![],
            sensing_abilities: vec![],
        }
    }
}

/// Turning up these values drastically increases fov calculation time.
/// The largest maps we can support with f32 accuracy is a 2000x2000 tiled map.
/// FOV calculation time will take 10x-15x slower, up to 2-3ms for just a single player calculation.
/// For bigger maps than 500x500 gridmaps we need a new and better FOV algorithm.
/// Dividible by 2.
pub const FOV_MAP_WIDTH: usize = 500;

/// Use this to use the Doryen FOV algorithm.
pub fn to_doryen_coordinates(x: i16, y: i16) -> (usize, usize) {
    let mut n_x = x + FOV_MAP_WIDTH as i16 / 2;
    let mut n_y = y + FOV_MAP_WIDTH as i16 / 2;

    if doryen_coordinates_out_of_range(n_x as usize, n_y as usize) {
        n_x = 0;
        n_y = 0;
    }

    (n_x as usize, n_y as usize)
}
/// Check if supplied doryen coordinates are out of range as a function.
pub fn doryen_coordinates_out_of_range(x: usize, y: usize) -> bool {
    x > FOV_MAP_WIDTH || y > FOV_MAP_WIDTH
}
