use crate::map;
use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Resource, Default)]
pub struct Data {
    pub entities: HashMap<map::components::Hex, Vec<Entity>>,
}

#[derive(Resource, Default)]
pub struct Visible {
    pub entities: Vec<Entity>,
}

#[derive(Resource, Default)]
pub struct MajorJobFlag(pub bool);
