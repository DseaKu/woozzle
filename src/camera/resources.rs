use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerView {
    pub min: Vec2,
    pub max: Vec2,
    pub center: Vec2,
}
