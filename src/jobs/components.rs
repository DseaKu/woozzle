use bevy::prelude::*;
use std::collections::VecDeque;

use crate::map::components::Hex;

pub enum Action {
    GoToHex(Hex),
    GoToPoint(Vec2),
    Wait(f32),
}

#[derive(Component, Default)]
pub struct ActionQueue(pub VecDeque<Action>);

#[derive(Component)]
pub struct CurrentAction(pub Action);

#[derive(Component)]
pub struct JobLess;
