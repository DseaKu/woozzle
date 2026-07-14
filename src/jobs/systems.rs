use super::components::*;
use crate::woozzle::{self, components::MoveSpeed};
use bevy::prelude::*;

pub fn process_job(
    mut woozzle_action_queues: Query<(Entity, &mut ActionQueue), Without<Busy>>,
    mut commands: Commands,
) {
    for (woozzle, mut action_queue) in &mut woozzle_action_queues {
        // Mark jobless Woozzles, so that it can be filtered out afterwards and assign a new job
        if action_queue.0.is_empty() {
            commands.entity(woozzle).insert(JobLess);
            continue;
        }

        commands.entity(woozzle).insert(Busy);

        let next_action = action_queue.0.pop_front().unwrap();

        match next_action {
            Action::GoToPoint(pos) => {
                commands.entity(woozzle).insert(GoToPoint(pos));
            }
            Action::Wait(time) => {
                commands.entity(woozzle).insert(Wait(time));
            }
        }
    }
}

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
