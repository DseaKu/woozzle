use super::components::{Hex, TerrainType};
use bevy::prelude::*;

use crate::graphics;

#[derive(Bundle)]
pub struct TileBundle {
    tile_type: TerrainType,
    transform: Transform,
}
impl TileBundle {
    pub fn new(hex: Hex, tile_type: TerrainType) -> Self {
        Self {
            transform: Transform::from_xyz(
                hex.to_world().x,
                hex.to_world().y,
                graphics::DrawOrder::Ground.z(),
            ),

            tile_type,
        }
    }
}
