use super::components::{Hex, TerrainType};
use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Resource, Default)]
pub struct MapData {
    pub tiles: HashMap<Hex, TerrainType>,
}
impl MapData {
    pub fn set_tile(&mut self, hex: Hex, terrain_type: TerrainType) {
        self.tiles.insert(hex, terrain_type);
    }
}
