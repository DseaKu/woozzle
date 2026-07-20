use crate::jobs::components::{Action, ActionQueue};
use bevy::prelude::*;
use rand;

pub fn assign_rectangle_patrol(action_queue: &mut ActionQueue, start_pos: Vec2, size: f32) {
    action_queue.0.push_back(Action::GoToPoint(Vec2::new(
        start_pos.x + size,
        start_pos.y,
    )));
    action_queue.0.push_back(Action::GoToPoint(Vec2::new(
        start_pos.x + size,
        start_pos.y - size,
    )));
    action_queue.0.push_back(Action::GoToPoint(Vec2::new(
        start_pos.x,
        start_pos.y - size,
    )));
    action_queue.0.push_back(Action::GoToPoint(start_pos));
}

pub fn wandering(action_queue: &mut ActionQueue, start_pos: Vec2, range: f32) {
    const MAX_ACTIONS: u32 = 1;
    const MAX_WAIT_DURATION: f32 = 5.0;

    let num_actions = rand::random_range(1..=MAX_ACTIONS);

    for _ in 0..num_actions {
        match rand::random_range(0..10) {
            // Big step
            0..8 => {
                let random_point = Vec2::new(
                    rand::random_range(-range..range),
                    rand::random_range(-range..range),
                );
                action_queue
                    .0
                    .push_back(Action::GoToPoint(start_pos + random_point));
            }
            // Wait
            8..9 => {
                let duration = rand::random_range(0.0..MAX_WAIT_DURATION);
                action_queue.0.push_back(Action::Wait(duration));
            }
            _ => {} // 0: Do nothing (33% chance to pause planning)
        }
    }
}
