use crate::woozzle::components::{CollisionCounter, MoveSpeed};
use crate::{graphics, jobs::components::ActionQueue};

const BASE_SPEED: f32 = 100.0;
pub const COLLSION_RADIUS: f32 = 7.0;

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
    locked_axes: LockedAxes, // Disable spinning when colliding
    colliding_entities: CollidingEntities,
    collsion_counter: CollisionCounter,
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
            colliding_entities: CollidingEntities::default(),
            collsion_counter: CollisionCounter::default(),
        }
    }
}
