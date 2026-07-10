use bevy::{platform::collections::HashMap, prelude::*};

use crate::map::components::{Hex, TerrainType};

#[derive(Resource, Default)]
pub struct TilesetAsset {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Default)]
pub struct SpawnedTileTexutures {
    pub tiles: HashMap<Hex, TerrainType>,
}
