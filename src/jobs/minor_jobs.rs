use crate::jobs::components::{Busy, GoToPoint, Wait};
use crate::woozzle::bundles::COLLSION_RADIUS;
use crate::woozzle::components::{CollisionCounter, GhostMode, MoveSpeed, Woozzle};
use avian2d::prelude::*;
use bevy::prelude::*;

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
    mut query: Query<(Entity, &mut GhostMode)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut ghost) in &mut query {
        ghost.0 -= time.delta_secs();

        if ghost.0 <= 0.0 {
            commands
                .entity(entity)
                .remove::<GhostMode>()
                .insert(Collider::circle(COLLSION_RADIUS));
        }
    }
}

type MoverQuery<'a> = (
    Entity,
    &'a GoToPoint,
    &'a mut Transform,
    &'a MoveSpeed,
    &'a mut LinearVelocity,
    &'a CollidingEntities,
    &'a mut CollisionCounter,
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
        mut collision_counter,
    ) in &mut query
    {
        const ARRIVAL_TOLERANCE: f32 = 50.0;
        const COLLISION_MAX: u32 = 500;
        const GHOST_DURATION: f32 = 1.0;

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
            collision_counter.0 = 0;
            continue;
        }

        // Ghost Mode woozzle, if too many collisions
        if !collision.is_empty() {
            collision_counter.0 += 1;
            if collision_counter.0 >= COLLISION_MAX {
                commands
                    .entity(woozzle)
                    .remove::<Collider>()
                    .insert(GhostMode(GHOST_DURATION));
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
