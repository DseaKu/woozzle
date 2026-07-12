use crate::map;
use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Resource, Default)]
pub struct VisibleWoozles {
    entities: HashMap<map::components::Hex, Entity>,
}
