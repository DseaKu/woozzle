use super::components;
use super::events;
use super::resources;
use crate::camera::resources::PlayerView;
use crate::input;
use crate::map::components::Hex;
use bevy::prelude::*;

pub fn show_debug_ui(
    _trigger: On<events::ShowDebugUi>,
    mut commands: Commands,
    mut debug_ui_state: ResMut<resources::DebugUiState>,
) {
    use components::*;
    commands
        .spawn(RootNodeBundle::new())
        .with_children(|builder| {
            builder.spawn(ContainerNode::new("Mouse Position:"));
            builder.spawn(ItemText::new(MouseWorldPosTextLabel));
            builder.spawn(ItemText::new(MouseHexPosTextLabel));

            builder.spawn(ContainerNode::new("Camera:"));
            builder.spawn(ItemText::new(CameraCenterLabel));
        });
    debug_ui_state.is_enabled = true;
}

pub fn update_camera_center_text(
    debug_ui_state: Res<resources::DebugUiState>,
    mut text: Single<&mut Text, With<components::CameraCenterLabel>>,
    player_view: Res<PlayerView>,
) {
    crate::guard_update!(debug_ui_state.is_changed() || player_view.is_changed());
    **text = format!(
        "Top Left= {}, Center: {}",
        player_view.top_left, player_view.center
    )
    .into();
}

pub fn update_mouse_world_pos_text(
    debug_ui_state: Res<resources::DebugUiState>,
    mut text: Single<&mut Text, With<components::MouseHexPosTextLabel>>,
    mouse_pos: Res<input::resources::MousePos>,
) {
    crate::guard_update!(debug_ui_state.is_enabled || mouse_pos.is_changed());
    let pos = Hex::from_world(mouse_pos.world);
    **text = format!("Hex q={}, r={}", pos.q, pos.r).into();
}

pub fn update_mouse_hex_pos_text(
    debug_ui_state: Res<resources::DebugUiState>,
    mut text: Single<&mut Text, With<components::MouseWorldPosTextLabel>>,
    mouse_pos: Res<input::resources::MousePos>,
) {
    crate::guard_update!(debug_ui_state.is_enabled || mouse_pos.is_changed());
    let pos = mouse_pos.world;
    **text = format!("World x={:.2}, y={:.2}", pos.x, pos.y).into();
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
    _trigger: On<input::events::ToggleDebugUiEvent>,
    mut commands: Commands,
    debug_ui_state: Res<resources::DebugUiState>,
) {
    if !debug_ui_state.is_enabled {
        commands.trigger(events::ShowDebugUi);
    } else {
        commands.trigger(events::HideDebugUi);
    }
}
