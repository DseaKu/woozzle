use bevy::prelude::*;

#[derive(Event)]
pub struct ToggleDebugUi;

#[derive(Event)]
pub struct SetTile;

#[derive(Event)]
pub struct RemoveTile;

#[derive(Event)]
pub struct SpawnWoozle;

#[derive(Event)]
pub struct ChangeMajorJob;
