use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct DebugUiState {
    pub is_enabled: bool,
}
#[derive(Resource, Default)]
pub struct MouseParameters {
    pub world_pos: Option<Vec2>,
    pub hex_pos: Option<Vec2>,
}
