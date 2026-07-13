use super::components::{Hex, TerrainType};
use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Resource, Default)]
pub struct TileData {
    pub tiles: HashMap<Hex, TerrainType>,
}

#[derive(Resource, Default)]
pub struct VisibleHexes {
    pub tiles: Vec<Hex>,
}
