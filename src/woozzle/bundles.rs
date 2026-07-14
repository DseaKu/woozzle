use crate::{graphics, jobs::components::ActionQueue, woozzle::components::MoveSpeed};

const BASE_SPEED: f32 = 100.0;
const COLLSION_RADIUS: f32 = 2.5;

use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct Woozzle {
    label: super::components::Woozzle,
    transform: Transform,
    action_queue: ActionQueue,
    speed: MoveSpeed,
    rigid_body: RigidBody,
    collider: Collider,
    locked_axes: LockedAxes,
    sleeping_disabled: SleepingDisabled,
}

impl Woozzle {
    pub fn new(pos: Vec2) -> Self {
        Self {
            label: super::components::Woozzle,
            transform: Transform::from_xyz(pos.x, pos.y, graphics::DrawOrder::OnGround.as_f32()),
            action_queue: ActionQueue::default(),
            speed: MoveSpeed(BASE_SPEED),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::circle(COLLSION_RADIUS),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            sleeping_disabled: SleepingDisabled,
        }
    }
}
