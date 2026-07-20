use crate::jobs::components::{Action, ActionQueue, Busy, GoToPoint, Wait};
use crate::woozzle::components::{MoveSpeed, Woozzle};
use avian2d::prelude::*;
use bevy::prelude::*;

const ARRIVAL_TOLERANCE: f32 = 5.0;

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
    for (woozzle, go_to_point, mut transform, move_speed, mut velocity, collision, action_queue) in
        &mut query
    {
        let dst_pos = go_to_point.0;
        let cur_pos = transform.translation.truncate();
        let direction = dst_pos - cur_pos;
        let distance = direction.length();
        let step_size = move_speed.0 * time.delta_secs();

        // Arrival Tolerance: If woozzle arrived perfectly or it got close but got blocked
        if distance <= step_size || (distance < ARRIVAL_TOLERANCE && !collision.is_empty()) {
            velocity.0 = Vec2::ZERO;

            if distance <= step_size {
                transform.translation.x = dst_pos.x;
                transform.translation.y = dst_pos.y;
            }

            commands.entity(woozzle).remove::<(GoToPoint, Busy)>();
            continue;
        }

        let target_velocity = direction.normalize() * move_speed.0;

        // Smoothly steer towards the target
        velocity.0 = velocity.0.lerp(target_velocity, 5.0 * time.delta_secs());
    }
}
