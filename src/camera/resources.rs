use bevy::prelude::*;

/// World-space rectangle currently visible through the camera.
#[derive(Resource, Default)]
pub struct PlayerView {
    pub min: Vec2,
    pub max: Vec2,
    pub center: Vec2,
}
