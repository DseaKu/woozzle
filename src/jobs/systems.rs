use crate::woozzle::components::NeedsFacingUpdate;

use super::components::*;
use bevy::prelude::*;

pub fn start_next_action(
    mut woozzle_action_queues: Query<(Entity, &mut ActionQueue), Without<Busy>>,
    mut commands: Commands,
) {
    for (woozzle, mut action_queue) in &mut woozzle_action_queues {
        // Mark jobless Woozzles, so that it can be filtered out afterwards and assign a new job
        if action_queue.0.is_empty() {
            commands.entity(woozzle).insert(Idle);
            continue;
        }

        // Pop next action
        let next_action = action_queue.0.pop_front().unwrap();
        commands.entity(woozzle).insert(Busy);
        match next_action {
            Action::GoToPoint {
                target,
                arrival_tolerance,
                reset_stuck_on_arrival,
            } => {
                commands.entity(woozzle).insert(GoToPoint {
                    target,
                    arrival_tolerance,
                    reset_stuck_on_arrival,
                });
            }
            Action::Wait(time) => {
                commands.entity(woozzle).insert(Wait(time));
            }
        }

        // Mark for updating face dir
        commands.entity(woozzle).insert(NeedsFacingUpdate);
    }
}
