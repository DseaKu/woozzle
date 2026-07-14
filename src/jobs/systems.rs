use super::components::*;
use bevy::prelude::*;

pub fn process_job(
    mut woozzle_action_queues: Query<(Entity, &mut ActionQueue), Without<Busy>>,
    mut commands: Commands,
) {
    for (woozzle, mut action_queue) in &mut woozzle_action_queues {
        // Mark jobless Woozzles, so that it can be filtered out easily afterwards and assign a new
        // job
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
            Action::_Wait(time) => {
                commands.entity(woozzle).insert(_Wait(time));
            }
        }
    }
}

fn go_to_point(query: Query<(Entity, &GoToPoint, &mut Transform)>) {
    for (woozzle, go_to_point, mut transform) in query {
        let dst_pos = go_to_point.0;
    }
}
