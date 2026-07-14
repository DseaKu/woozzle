use bevy::prelude::*;

#[derive(Component, Default, PartialEq, Eq)]
pub enum WoozzleState {
    #[default]
    Idle,
    _Walking,
}

#[derive(Component)]
pub struct Woozzle;

#[derive(Component)]
pub struct NeedsTask;
