use crate::{graphics, jobs::components::ActionQueue, woozzle::components::MoveSpeed};

const BASE_SPEED: f32 = 100.0;

use bevy::prelude::*;

#[derive(Bundle)]
pub struct Woozzle {
    label: super::components::Woozzle,
    transform: Transform,
    action_queue: ActionQueue,
    speed: MoveSpeed,
}

impl Woozzle {
    pub fn new(pos: Vec2) -> Self {
        Self {
            label: super::components::Woozzle,
            transform: Transform::from_xyz(pos.x, pos.y, graphics::DrawOrder::OnGround.as_f32()),
            action_queue: ActionQueue::default(),
            speed: MoveSpeed(BASE_SPEED),
        }
    }
}
