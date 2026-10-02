use crate::jobs::components::{Busy, GoToPoint, Wait};
use crate::woozzle::bundles::COLLISION_RADIUS;
use crate::woozzle::components::{GhostMode, MoveSpeed, StuckCounter, Woozzle};
use avian2d::prelude::*;
use bevy::prelude::*;

const GHOST_SPEED_FACTOR: f32 = 1.0;

pub fn run_wait(
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

pub fn tick_ghost_mode(
    mut query: Query<(Entity, &mut GhostMode)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut ghost) in &mut query {
        ghost.0 -= time.delta_secs();

        // End ghost mode
        if ghost.0 <= 0.0 {
            commands
                .entity(entity)
                .remove::<GhostMode>()
                .insert(Collider::circle(COLLISION_RADIUS));
        }
    }
}

type MoverQuery<'a> = (
    Entity,
    &'a GoToPoint,
    &'a mut Transform,
    &'a MoveSpeed,
    Option<&'a GhostMode>,
    &'a mut LinearVelocity,
    &'a CollidingEntities,
    &'a mut StuckCounter,
);
pub fn run_go_to_point(
    mut query: Query<MoverQuery, With<Woozzle>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (
        woozzle,
        go_to_point,
        mut transform,
        move_speed,
        ghost_mode,
        mut velocity,
        collision,
        mut stuck_counter,
    ) in &mut query
    {
        const MICRO_GHOST_THRESHOLD: u32 = 15;
        const STUCK_MAX: u32 = 200;
        const GHOST_DURATION: f32 = 10.0;
        const MICRO_GHOST_DURATION: f32 = 0.2;
        const ARRIVAL_TOLERANCE_MAX_FACTOR: f32 = 3.0;
        const STUCK_STEP: u32 = 3;

        let dst_pos = go_to_point.target;
        let cur_pos = transform.translation.truncate();
        let direction = dst_pos - cur_pos;
        let distance = direction.length();
        // Derive the speed from the base speed, so ghosting can't drift it
        let speed = match ghost_mode {
            Some(_) => move_speed.0 * GHOST_SPEED_FACTOR,
            None => move_speed.0,
        };
        let step_size = speed * time.delta_secs();

        // Increase arrival_tolerance by the stuck_counter
        let progress = (stuck_counter.0 as f32 / STUCK_MAX as f32).clamp(0.0, 1.0);
        let arrival_tolerance = go_to_point.arrival_tolerance.lerp(
            go_to_point.arrival_tolerance * ARRIVAL_TOLERANCE_MAX_FACTOR,
            progress,
        );

        // Arrival Tolerance: If woozzle arrived perfectly or it got close but got blocked
        if distance <= step_size || (distance < arrival_tolerance && !collision.is_empty()) {
            velocity.0 = Vec2::ZERO;

            if distance <= step_size {
                transform.translation.x = dst_pos.x;
                transform.translation.y = dst_pos.y;
            }

            commands.entity(woozzle).remove::<(GoToPoint, Busy)>();
            if go_to_point.reset_stuck_on_arrival {
                stuck_counter.0 = 0;
            }
            continue;
        }

        // Collision logic: Micro Ghost first, then Ghost Mode
        if !collision.is_empty() {
            let previous_count = stuck_counter.0;
            stuck_counter.0 += STUCK_STEP;

            // Try micro ghost step, when the counter crosses the threshold
            if previous_count < MICRO_GHOST_THRESHOLD && stuck_counter.0 >= MICRO_GHOST_THRESHOLD {
                commands
                    .entity(woozzle)
                    .remove::<Collider>()
                    .insert(GhostMode(MICRO_GHOST_DURATION));
            }

            // Turn into Ghost Mode if still stuck
            if stuck_counter.0 >= STUCK_MAX {
                commands
                    .entity(woozzle)
                    .remove::<Collider>()
                    .insert(GhostMode(GHOST_DURATION));
                stuck_counter.0 = 0;
            }
        } else {
            stuck_counter.0 = stuck_counter.0.saturating_sub(1);
        }

        let target_velocity = direction.normalize() * speed;

        // Smoothly steer towards the target
        velocity.0 = velocity.0.lerp(target_velocity, 5.0 * time.delta_secs());
    }
}
