pub use super::hex::components::Hex;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use strum::EnumCount;
use strum_macros::EnumCount;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, EnumCount, Serialize, Deserialize)]
pub enum TerrainType {
    _Empty,
    Grass,
    _Water,
    _Dirt,
}
impl TerrainType {
    pub fn to_atlas_index(self) -> usize {
        use TerrainType::*;
        match self {
            _Empty => 0,
            Grass => 1,
            _Water => 2,
            _Dirt => 3,
        }
    }
    pub fn n_of_types() -> usize {
        TerrainType::COUNT
    }
}
