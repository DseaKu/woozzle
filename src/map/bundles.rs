use super::components::{Hex, TerrainType};
use bevy::prelude::*;

use crate::graphics;

#[derive(Bundle)]
pub struct HexTile {
    tile_type: TerrainType,
    transform: Transform,
}
impl HexTile {
    pub fn new(hex: Hex, tile_type: TerrainType) -> Self {
        Self {
            transform: Transform::from_xyz(
                hex.to_world().x,
                hex.to_world().y,
                graphics::DrawOrder::Ground.as_f32(),
            ),

            tile_type,
        }
    }
}
