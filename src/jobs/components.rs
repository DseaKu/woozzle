use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component)]
pub struct GoToPoint(pub Vec2);

#[derive(Component)]
pub struct Wait(pub f32);

#[derive(Clone)]
pub enum Action {
    // GoToHex(Hex),
    GoToPoint(Vec2),
    Wait(f32),
}

#[derive(Component, Default)]
pub struct ActionQueue(pub VecDeque<Action>);

#[derive(Component)]
pub struct JobLess;

#[derive(Component)]
pub struct Busy;
