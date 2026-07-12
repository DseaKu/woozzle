use bevy::prelude::*;

#[derive(Event)]
pub struct ToggleDebugUiEvent;

#[derive(Event)]
pub struct SetTileEvent;

#[derive(Event)]
pub struct RemoveTileEvent;

#[derive(Event)]
pub struct SpawnWoozleEvent;
