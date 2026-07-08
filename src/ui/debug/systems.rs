use super::components;
use super::events;
use super::resources;
use crate::input;
use bevy::prelude::*;

pub fn show_debug_ui(
    _trigger: On<events::ShowDebugUi>,
    mut commands: Commands,
    mut debug_ui_state: ResMut<resources::DebugUiState>,
    mouse_parameters: Res<resources::MouseParameters>,
) {
    use components::{RootNodeBundle, SubNode};
    let mouse_world_pos = mouse_parameters.world_pos.unwrap_or_default();
    commands
        .spawn(RootNodeBundle::new())
        .with_children(|builder| {
            builder.spawn(SubNode::new("Mouse"));
            builder.spawn(SubNode::new_indented(&format!(
                "World Pos: {}",
                mouse_world_pos.x
            )));
            builder.spawn(SubNode::new_indented("Hex Pos: None"));
        });
    debug_ui_state.is_enabled = true;
}

pub fn update_mouse_parameters(_trigger: On<events::ShowDebugUi>) {}

pub fn hide_debug_ui(
    _trigger: On<events::HideDebugUi>,
    mut commands: Commands,
    root_node_entity: Single<Entity, With<components::RootNodeLabel>>,
    mut debug_ui_state: ResMut<resources::DebugUiState>,
) {
    commands.entity(*root_node_entity).despawn();
    debug_ui_state.is_enabled = false;
}

pub fn toggle_debug_ui(
    _trigger: On<input::ToggleDebugUiEvent>,
    mut commands: Commands,
    debug_ui_state: Res<resources::DebugUiState>,
) {
    if !debug_ui_state.is_enabled {
        commands.trigger(events::ShowDebugUi);
    } else {
        commands.trigger(events::HideDebugUi);
    }
}
