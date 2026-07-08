use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct DebugUiState {
    pub is_enabled: bool,
}
#[derive(Resource, Default)]
pub struct MouseText {
    pub world_pos: String,
    pub hex_pos: Option<Vec2>,
}
