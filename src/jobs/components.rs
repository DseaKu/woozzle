use bevy::prelude::*;
use std::collections::VecDeque;

/// Running action: walk to `target`.
#[derive(Component)]
pub struct GoToPoint {
    pub target: Vec2,
    /// Distance at which a blocked woozzle counts as arrived
    pub arrival_tolerance: f32,
    /// Clear `StuckCounter` on arrival
    pub reset_stuck_on_arrival: bool,
}

/// Running action: stand still for the remaining seconds.
#[derive(Component)]
pub struct Wait(pub f32);

/// Queued step; becomes the matching action component once started.
#[derive(Clone)]
pub enum Action {
    // GoToHex(Hex),
    GoToPoint {
        target: Vec2,
        arrival_tolerance: f32,
        reset_stuck_on_arrival: bool,
    },
    Wait(f32),
}

/// Pending actions, executed front to back.
#[derive(Component, Default)]
pub struct ActionQueue(pub VecDeque<Action>);

/// Queue ran empty; waiting for a new job.
#[derive(Component)]
pub struct Idle;

/// An action is running; the next one won't start until it is removed.
#[derive(Component)]
pub struct Busy;
