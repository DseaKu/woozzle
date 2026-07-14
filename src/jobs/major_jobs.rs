use crate::jobs::components::{Action, ActionQueue};
use bevy::prelude::*;

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
