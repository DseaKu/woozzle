use super::components::Hex;
use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Resource, Default)]
pub struct TilesByHex {
    pub entities: HashMap<Hex, Entity>,
}
