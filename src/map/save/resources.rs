use crate::map::components::TerrainType;
use crate::map::hex::components::Hex;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Resource, Serialize, Deserialize)]
pub struct MapSaveData {
    pub tiles: Vec<(Hex, TerrainType)>,
}
