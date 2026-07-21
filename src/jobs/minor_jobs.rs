use std::f32::consts::TAU;

use crate::jobs::components::{Action, ActionQueue, Busy, GoToPoint, Wait};
use crate::woozzle::bundles::COLLSION_RADIUS;
use crate::woozzle::components::{CollisionCounter, GhostMode, MoveSpeed, Woozzle};
use avian2d::prelude::*;
use bevy::prelude::*;

const GHOST_SPEED_FACTOR: f32 = 3.0;

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

pub fn ghost_system(
    mut query: Query<(Entity, &mut GhostMode, &mut MoveSpeed)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut ghost, mut move_speed) in &mut query {
        ghost.0 -= time.delta_secs();

        // End ghost mode
        if ghost.0 <= 0.0 {
            commands
                .entity(entity)
                .remove::<GhostMode>()
                .insert(Collider::circle(COLLSION_RADIUS));
            move_speed.0 /= GHOST_SPEED_FACTOR;
        }
    }
}

type MoverQuery<'a> = (
    Entity,
    &'a GoToPoint,
    &'a mut Transform,
    &'a mut MoveSpeed,
    &'a mut LinearVelocity,
    &'a CollidingEntities,
    &'a mut CollisionCounter,
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
        mut move_speed,
        mut velocity,
        collision,
        mut collision_counter,
        mut action_queue,
    ) in &mut query
    {
        const COLLISION_DETOUR: u32 = 50;
        const COLLISION_MAX: u32 = 500;
        const GHOST_DURATION: f32 = 1.0;

        let dst_pos = go_to_point.0;
        let arrival_tolerance = go_to_point.1;
        let cur_pos = transform.translation.truncate();
        let direction = dst_pos - cur_pos;
        let distance = direction.length();
        let step_size = move_speed.0 * time.delta_secs();

        // Arrival Tolerance: If woozzle arrived perfectly or it got close but got blocked
        if distance <= step_size || (distance < arrival_tolerance && !collision.is_empty()) {
            velocity.0 = Vec2::ZERO;

            if distance <= step_size {
                transform.translation.x = dst_pos.x;
                transform.translation.y = dst_pos.y;
            }

            commands.entity(woozzle).remove::<(GoToPoint, Busy)>();
            collision_counter.0 = 0;
            continue;
        }

        // Collision logic: Detour step first, then Ghost Mode
        if !collision.is_empty() {
            collision_counter.0 += 1;

            // Try a quick random detour step
            if collision_counter.0 == COLLISION_DETOUR {
                let angle = rand::random_range(0.0..TAU);
                let distance = rand::random_range(20.0..50.0);
                let random_offset = Vec2::new(angle.cos(), angle.sin() * distance);

                action_queue
                    .0
                    .push_front(Action::GoToPoint(dst_pos, arrival_tolerance));
                action_queue
                    .0
                    .push_front(Action::GoToPoint(cur_pos + random_offset, 10.0));

                commands.entity(woozzle).remove::<(GoToPoint, Busy)>();
                continue;
            }

            // Turn into Ghost Mode if still stuck
            if collision_counter.0 >= COLLISION_MAX {
                commands
                    .entity(woozzle)
                    .remove::<Collider>()
                    .insert(GhostMode(GHOST_DURATION));
                move_speed.0 *= GHOST_SPEED_FACTOR;
                collision_counter.0 = 0;
            }
        } else {
            collision_counter.0 = collision_counter.0.saturating_sub(1);
        }

        let target_velocity = direction.normalize() * move_speed.0;

        // Smoothly steer towards the target
        velocity.0 = velocity.0.lerp(target_velocity, 5.0 * time.delta_secs());
    }
}
