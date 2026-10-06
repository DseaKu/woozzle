use bevy::prelude::*;

#[derive(Component)]
pub struct Woozzle;

/// Base speed in world units per second.
#[derive(Component)]
pub struct MoveSpeed(pub f32);

/// Grows while blocked by collisions, see `jobs::actions::run_go_to_point`.
#[derive(Component, Default)]
pub struct StuckCounter(pub u32);

/// Remaining seconds without a collider, to escape being stuck.
#[derive(Component)]
pub struct GhostMode(pub f32);

/// Marker: re-evaluate the sprite flip from the current velocity.
#[derive(Component)]
pub struct NeedsFacingUpdate;
