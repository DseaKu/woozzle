use super::components::Hex;
use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Resource, Default)]
pub struct TileData {
    pub entities: HashMap<Hex, Entity>,
}

#[derive(Resource, Default)]
pub struct VisibleTiles {
    pub entities: Vec<Entity>,
}
