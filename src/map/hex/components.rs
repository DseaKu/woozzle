use super::conversion;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
}
impl Hex {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }
    pub fn to_world(self) -> Vec2 {
        conversion::hex_to_world(self)
    }
    pub fn from_world(pixel: Vec2) -> Hex {
        conversion::world_to_hex(pixel)
    }
}
