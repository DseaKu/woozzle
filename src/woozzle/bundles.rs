use crate::woozzle::components::{MoveSpeed, StuckCounter};
use crate::{graphics, jobs::components::ActionQueue};

const BASE_SPEED: f32 = 150.0;
pub const COLLISION_RADIUS: f32 = 5.5;

use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct WoozzleBundle {
    label: super::components::Woozzle,
    transform: Transform,
    action_queue: ActionQueue,
    speed: MoveSpeed,
    rigid_body: RigidBody,
    collider: Collider,
    locked_axes: LockedAxes, // Disable spinning when colliding
    colliding_entities: CollidingEntities,
    stuck_counter: StuckCounter,
}

impl WoozzleBundle {
    pub fn new(pos: Vec2) -> Self {
        Self {
            label: super::components::Woozzle,
            transform: Transform::from_xyz(pos.x, pos.y, graphics::DrawOrder::OnGround.z()),
            action_queue: ActionQueue::default(),
            speed: MoveSpeed(BASE_SPEED),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::circle(COLLISION_RADIUS),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            colliding_entities: CollidingEntities::default(),
            stuck_counter: StuckCounter::default(),
        }
    }
}
