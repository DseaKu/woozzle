use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerView {
    pub top_left: Vec2,
    pub bottom_right: Vec2,
    pub center: Vec2,
}
