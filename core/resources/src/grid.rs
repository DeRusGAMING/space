use crate::math::Vec3Int;
use serde::{Deserialize, Serialize};

/// All six faces of the cell. Represents walls, ceilings and floors.
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Default, Clone, Debug)]
pub enum CellFace {
    #[default]
    FrontWall,
    RightWall,
    BackWall,
    LeftWall,
    Floor,
    Ceiling,
    Center,
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
pub struct TargetCell {
    pub id: Vec3Int,
    pub face: CellFace,
}
