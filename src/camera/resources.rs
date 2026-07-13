use crate::map;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerView {
    pub top_left: Vec2,
    pub bot_right: Vec2,
    pub center: Vec2,
}
#[derive(Resource, Default)]
pub struct VisibleHexes {
    pub tiles: Vec<map::components::Hex>,
}
