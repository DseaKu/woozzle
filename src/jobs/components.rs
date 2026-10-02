use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component)]
pub struct GoToPoint {
    pub target: Vec2,
    pub arrival_tolerance: f32,
    pub reset_stuck_on_arrival: bool,
}

#[derive(Component)]
pub struct Wait(pub f32);

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

#[derive(Component, Default)]
pub struct ActionQueue(pub VecDeque<Action>);

#[derive(Component)]
pub struct Idle;

#[derive(Component)]
pub struct Busy;
