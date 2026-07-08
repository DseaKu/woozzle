use super::components;
use super::events;
use super::resources;
use crate::input;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn show_debug_ui(
    _trigger: On<events::ShowDebugUi>,
    mut commands: Commands,
    mut debug_ui_state: ResMut<resources::DebugUiState>,
    mouse_text: Res<resources::MouseText>,
) {
    use components::{RootNodeBundle, SubNode};

    commands
        .spawn(RootNodeBundle::new())
        .with_children(|builder| {
            builder.spawn(SubNode::new("Mouse"));
            builder.spawn(SubNode::new_indented(&mouse_text.world_pos));
        });
    debug_ui_state.is_enabled = true;
}

pub fn update_mouse_txt(
    mut mouse_text: ResMut<resources::MouseText>,
    debug_ui_state: Res<resources::DebugUiState>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    if !debug_ui_state.is_enabled {
        return;
    }
    mouse_text.world_pos = if let Some(coords) = window.cursor_position() {
        format!("x={}", coords.x)
    } else {
        "x=NaN".to_string()
    };
}

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
