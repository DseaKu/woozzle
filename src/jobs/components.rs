use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component)]
pub struct GoToPoint {
    pub target: Vec2,
    pub arrival_tolerance: f32,
    pub reset_counter_on_arrival: bool,
}

#[derive(Component)]
pub struct Wait(pub f32);

#[derive(Clone)]
pub enum Action {
    // GoToHex(Hex),
    GoToPoint {
        target: Vec2,
        arrival_tolerance: f32,
        reset_counter_on_arrival: bool,
    },
    Wait(f32),
}

#[derive(Component, Default)]
pub struct ActionQueue(pub VecDeque<Action>);

#[derive(Component)]
pub struct JobLess;

#[derive(Component)]
pub struct Busy;
