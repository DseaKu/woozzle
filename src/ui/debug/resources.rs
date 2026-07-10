use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct DebugUiState {
    pub is_enabled: bool,
}
