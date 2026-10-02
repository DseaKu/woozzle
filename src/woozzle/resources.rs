use crate::map;
use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Resource, Default)]
pub struct WoozzlesByHex {
    pub entities: HashMap<map::components::Hex, Vec<Entity>>,
}

/// Which job idle woozzles get assigned
#[derive(Resource, Default, Clone, Copy, PartialEq, Eq)]
pub enum JobMode {
    #[default]
    Wander,
    Patrol,
}
