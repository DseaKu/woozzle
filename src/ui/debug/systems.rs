use super::components;
use super::events::*;
use super::resources::*;
use crate::input;
use bevy::prelude::*;

pub fn show_debug_ui(
    _trigger: On<ShowDebugUi>,
    mut commands: Commands,
    mut debug_ui_state: ResMut<DebugUiState>,
) {
    use components::{RootNodeBundle, SubNode};
    commands
        .spawn(RootNodeBundle::new())
        .with_children(|builder| {
            builder.spawn(SubNode::new("Mouse"));
            builder.spawn(SubNode::new_indented("World Pos: x=0, y=0"));
            builder.spawn(SubNode::new_indented("Hex Pos: None"));
        });
    debug_ui_state.is_enabled = true;
}

pub fn update_mouse_parameters(_trigger: On<ShowDebugUi>) {}
pub fn hide_debug_ui(
    _trigger: On<HideDebugUi>,
    mut commands: Commands,
    root_node_entity: Single<Entity, With<components::RootNodeLabel>>,
    mut debug_ui_state: ResMut<DebugUiState>,
) {
    commands.entity(*root_node_entity).despawn();
    debug_ui_state.is_enabled = false;
}

pub fn toggle_debug_ui(
    _trigger: On<input::ToggleDebugUiEvent>,
    mut commands: Commands,
    debug_ui_state: Res<DebugUiState>,
) {
    if !debug_ui_state.is_enabled {
        commands.trigger(ShowDebugUi);
    } else {
        commands.trigger(HideDebugUi);
    }
}
