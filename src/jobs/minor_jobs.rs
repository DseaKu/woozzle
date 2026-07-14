use crate::jobs::components::{Busy, GoToPoint};
use crate::woozzle::components::MoveSpeed;
use bevy::prelude::*;

pub fn go_to_point(
    query: Query<(Entity, &GoToPoint, &mut Transform, &MoveSpeed)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (woozzle, go_to_point, mut transform, move_speed) in query {
        let dst_pos = go_to_point.0;
        let cur_pos = transform.translation.truncate();
        let direction = dst_pos - cur_pos;
        let distance = direction.length();
        let step_size = move_speed.0 * time.delta_secs();

        // Entity arrived at destination
        if distance <= step_size {
            // Last step, snap exactly to the destination
            // transform.translation.x = dst_pos.x;
            // transform.translation.y = dst_pos.y;
            commands.entity(woozzle).remove::<(GoToPoint, Busy)>();
            continue;
        }

        let velocity = direction.normalize() * step_size;
        transform.translation += velocity.extend(0.0);
    }
}
