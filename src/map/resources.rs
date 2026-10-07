use super::components::Hex;
use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};

/// Lookup from hex to its tile entity.
#[derive(Resource, Default, Serialize, Deserialize)]
pub struct TilesByHex {
    pub entities: HashMap<Hex, Entity>,
}
