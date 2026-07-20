use bevy::prelude::*;

#[derive(Component)]
pub struct Woozzle;

#[derive(Component)]
pub struct MoveSpeed(pub f32);

#[derive(Component, Default)]
pub struct CollisionCounter(pub u32);

#[derive(Component)]
pub struct GhostMode(pub f32);

#[derive(Component)]
pub struct DirtyFaceDir;
