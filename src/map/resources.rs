use super::components::Hex;
use bevy::{platform::collections::HashMap, prelude::*};

/// Lookup from hex to its tile entity.
#[derive(Resource, Default)]
pub struct TilesByHex {
    pub entities: HashMap<Hex, Entity>,
}
