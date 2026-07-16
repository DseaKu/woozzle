use crate::jobs::components::{Action, ActionQueue, Busy, GoToPoint, Wait};
use crate::woozzle::components::{MoveSpeed, Woozzle};
use avian2d::prelude::*;
use bevy::prelude::*;
use rand::RngExt;

const COLLIDING_BREAK_TIME: f64 = 0.3;

pub fn wait(
    query: Query<(Entity, &mut LinearVelocity, &mut Wait), With<Woozzle>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (woozzle, mut velocity, mut wait_time) in query {
        wait_time.0 -= time.delta_secs();

        velocity.0 = Vec2::ZERO;
        if wait_time.0 >= 0.0 {
            continue;
        }

        commands.entity(woozzle).remove::<(Wait, Busy)>();
    }
}

type MoverQuery<'a> = (
    Entity,
    &'a GoToPoint,
    &'a mut Transform,
    &'a MoveSpeed,
    &'a mut LinearVelocity,
    &'a CollidingEntities,
    &'a mut ActionQueue,
);
pub fn go_to_point(
    mut query: Query<MoverQuery, With<Woozzle>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (
        woozzle,
        go_to_point,
        mut transform,
        move_speed,
        mut velocity,
        collision,
        mut action_queue,
    ) in &mut query
    {
        let dst_pos = go_to_point.0;
        let cur_pos = transform.translation.truncate();
        let direction = dst_pos - cur_pos;
        let distance = direction.length();
        let step_size = move_speed.0 * time.delta_secs();

        // Entity collides
        if !collision.is_empty() {
            let random_time = rand::rng().random_range(0.1..=COLLIDING_BREAK_TIME) as f32;

            action_queue.0.push_front(Action::GoToPoint(go_to_point.0)); // Store current destination
            action_queue.0.push_front(Action::Wait(random_time)); // ...And take a break
            continue;
        }

        // Entity arrived at destination
        if distance <= step_size {
            // Stop moving
            velocity.0 = Vec2::ZERO;
            // Last step, snap exactly to the destination
            transform.translation.x = dst_pos.x;
            transform.translation.y = dst_pos.y;
            commands.entity(woozzle).remove::<(GoToPoint, Busy)>();
            continue;
        }

        // Apply physics velocity instead of moving the transform directly
        velocity.0 = direction.normalize() * move_speed.0;
    }
}
