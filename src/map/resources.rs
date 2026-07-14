use super::components::Hex;
use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Resource, Default)]
pub struct Data {
    pub entities: HashMap<Hex, Entity>,
}

#[derive(Resource, Default)]
pub struct Visible {
    pub entities: Vec<Entity>,
}
