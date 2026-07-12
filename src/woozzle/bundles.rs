use crate::graphics;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct Woozzle {
    transform: Transform,
}

impl Woozzle {
    pub fn new(pos: Vec2) -> Self {
        Self {
            transform: Transform::from_xyz(pos.x, pos.y, graphics::DrawOrder::OnGround.as_f32()),
        }
    }
}
