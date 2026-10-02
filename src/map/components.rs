pub use super::hex::components::Hex;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use strum::EnumCount;
use strum_macros::EnumCount;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, EnumCount, Serialize, Deserialize)]
pub enum TerrainType {
    Empty,
    Grass,
    Water,
    Dirt,
}
impl TerrainType {
    pub fn to_atlas_index(self) -> usize {
        use TerrainType::*;
        match self {
            Empty => 0,
            Grass => 1,
            Water => 2,
            Dirt => 3,
        }
    }
    pub fn count() -> usize {
        TerrainType::COUNT
    }
}
