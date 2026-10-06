use bevy::prelude::*;

/// Cursor position in world space (keeps the last value when the cursor leaves the window).
#[derive(Resource, Default)]
pub struct MousePos {
    pub world: Vec2,
}
