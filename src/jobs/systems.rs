use bevy::prelude::*;
use super::components::*;

pub fn process_job(
    mut woozzle_action_queues: Query<(Entity, &mut ActionQueue)>,
    mut commands: Commands,
) {
    for (woozzle, action_queue) in &mut woozzle_action_queues {
        // Label jobless Woozzles, so that it can be filtered out easily afterwards
        if action_queue.0.is_empty() {
            commands.entity(woozzle).insert(JobLess);
            continue;
        }
    }
}
/*
=============================================================================
  Woozzle Task System - Marker Component Architecture Concept
=============================================================================

  This system is designed using the "Marker Component" pattern for fast ECS
  querying. Instead of iterating over all workers and checking if their queue
  is empty, we use a `JobLess` component as a flag.

  Workflow:
  ---------
  1. `get_a_job` System:
     - Queries for: `Query<(&mut ActionQueue, Entity), With<JobLess>>`
     - When a new job is found:
         a) Push the actions (GoTo, PickUp, etc.) into the ActionQueue.
         b) REMOVE the `JobLess` component: `commands.entity(worker).remove::<JobLess>();`

  2. `process_job` System (or individual action execution systems):
     - Executes the current action.
     - When an action is completed, it pops the next action from the ActionQueue.
     - If the ActionQueue becomes completely empty:
         a) The worker has finished all tasks.
         b) INSERT the `JobLess` component: `commands.entity(worker).insert(JobLess);`

  Performance Benefits:
  ---------------------
  By maintaining the `JobLess` marker, Bevy can instantly find idle workers
  in memory without scanning busy workers, which scales efficiently even with
  thousands of entities!
*/
