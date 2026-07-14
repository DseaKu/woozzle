use crate::graphics;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct Woozzle {
    label: super::components::Woozzle,
    transform: Transform,
    state: super::components::WoozzleState,
    need_task: super::components::NeedsTask,
}

impl Woozzle {
    pub fn new(pos: Vec2) -> Self {
        Self {
            label: super::components::Woozzle,
            transform: Transform::from_xyz(pos.x, pos.y, graphics::DrawOrder::OnGround.as_f32()),
            state: super::components::WoozzleState::default(),
            need_task: super::components::NeedsTask,
        }
    }
}
