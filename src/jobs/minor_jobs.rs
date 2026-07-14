use crate::jobs::components::{Busy, GoToPoint};
use crate::woozzle::components::MoveSpeed;
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn go_to_point(
    mut query: Query<(
        Entity,
        &GoToPoint,
        &mut Transform,
        &MoveSpeed,
        &mut LinearVelocity,
    )>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (woozzle, go_to_point, mut transform, move_speed, mut velocity) in &mut query {
        let dst_pos = go_to_point.0;
        let cur_pos = transform.translation.truncate();
        let direction = dst_pos - cur_pos;
        let distance = direction.length();
        let step_size = move_speed.0 * time.delta_secs();

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
