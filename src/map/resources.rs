use super::components::{Hex, TerrainType};
use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Resource, Default)]
pub struct MapData {
    pub tiles: HashMap<Hex, TerrainType>,
}
