use crate::map;
use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Resource, Default)]
pub struct WoozlesData {
    pub entities: HashMap<map::components::Hex, Vec<Entity>>,
}

#[derive(Resource, Default)]
pub struct VisibleWoozzle {
    pub entities: Vec<Entity>,
}
