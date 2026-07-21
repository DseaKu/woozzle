use crate::woozzle::components::DirtyFaceDir;

use super::components::*;
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

        // Pop next action
        let next_action = action_queue.0.pop_front().unwrap();
        commands.entity(woozzle).insert(Busy);
        match next_action {
            Action::GoToPoint {
                target,
                arrival_tolerance,
            } => {
                commands
                    .entity(woozzle)
                    .insert(GoToPoint {
                        target,
                        arrival_tolerance,
                    });
            }
            Action::Wait(time) => {
                commands.entity(woozzle).insert(Wait(time));
            }
        }

        // Mark for updating face dir
        commands.entity(woozzle).insert(DirtyFaceDir);
    }
}
