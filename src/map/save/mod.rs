use super::components::{Hex, TerrainType};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod components;
mod systems;

#[derive(Default, Serialize, Deserialize)]
pub struct SavePlugin;
impl Plugin for SavePlugin {
    fn build(&self, _app: &mut App) {}
}

pub struct MapSaveData {
    pub tiles: Vec<(Hex, TerrainType)>,
}
