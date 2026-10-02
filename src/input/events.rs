use bevy::prelude::*;

#[derive(Event)]
pub struct ToggleDebugUi;

#[derive(Event)]
pub struct PlaceTile;

#[derive(Event)]
pub struct RemoveTile;

#[derive(Event)]
pub struct SpawnWoozzle;

#[derive(Event)]
pub struct ToggleJobMode;

#[derive(Event)]
pub struct OpenRadialMenu;
