use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MousePos {
    pub world: Vec2,
}
