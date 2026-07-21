use super::systems;
use bevy::prelude::*;
use strum::EnumCount;
use strum_macros::EnumCount;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
pub enum TerrainType {
    _Empty,
    _Grass,
    Water,
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

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
}
impl Hex {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }
    pub fn to_world(self) -> Vec2 {
        systems::from_hex_to_world(self)
    }
    pub fn from_world(pixel: Vec2) -> Hex {
        systems::from_world_to_hex(pixel)
    }
}
