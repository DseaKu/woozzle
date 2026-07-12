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
    pub fn remove_tile(&mut self, hex: Hex) {
        self.tiles.remove(&hex);
    }
}

/// A mathematical list of Hex coordinates that are currently within the camera's visible viewport.
/// This resource only tracks abstract map logic and does not care about Bevy Entities or Sprites.
#[derive(Resource, Default)]
pub struct ViewportHexes {
    pub tiles: Vec<Hex>,
}
