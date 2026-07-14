use crate::{camera, diagnostic, input, map, woozzle};
use bevy::prelude::*;

use super::{components, events, resources};

pub fn show_debug_ui(
    _trigger: On<events::ShowDebugUi>,
    mut commands: Commands,
    mut debug_ui_state: ResMut<resources::DebugUiState>,
) {
    use components::*;
    commands
        .spawn(RootNodeBundle::new())
        .with_children(|builder| {
            builder.spawn(ContainerNode::new("System:"));
            builder.spawn(ItemText::new(FpsLabel));

            builder.spawn(ContainerNode::new("Mouse Position:"));
            builder.spawn(ItemText::new(MouseWorldPosTextLabel));
            builder.spawn(ItemText::new(MouseHexPosTextLabel));

            builder.spawn(ContainerNode::new("Camera:"));
            builder.spawn(ItemText::new(CameraCenterLabel));

            builder.spawn(ContainerNode::new("Entites:"));
            builder.spawn(ItemText::new(TileEntityLabel));
            builder.spawn(ItemText::new(WoozzleEntityLabel));
        });
    debug_ui_state.is_enabled = true;
}
// pub fn update_XXX_text(
//     debug_ui_state: Res<resources::DebugUiState>,
//     mut text: Single<&mut Text, With<components::XXXLabel>>,
// ) {
//     crate::guard_update!(debug_ui_state.is_enabled);
// **text = format!(
//     "Top Left= {}, Center: {}",
//     player_view.top_left, player_view.center
// )
// .into();
// }
pub fn update_woozzle_entity_text(
    debug_ui_state: Res<resources::DebugUiState>,
    mut text: Single<&mut Text, With<components::WoozzleEntityLabel>>,
    woozzle_data: Res<woozzle::resources::Data>,
    visible_woozzles: Res<woozzle::resources::Visible>,
) {
    crate::guard_update!(debug_ui_state.is_enabled);
    **text = format!(
        "Woozles={}, Culled={}",
        woozzle_data.entities.len(),
        visible_woozzles.entities.len()
    )
    .into();
}

pub fn update_tile_entity_text(
    debug_ui_state: Res<resources::DebugUiState>,
    mut text: Single<&mut Text, With<components::TileEntityLabel>>,
    tile_data: Res<map::resources::Data>,
    visible_tiles: Res<map::resources::Visible>,
) {
    crate::guard_update!(debug_ui_state.is_enabled);
    **text = format!(
        "Tiles={}, Culled={}",
        tile_data.entities.len(),
        visible_tiles.entities.len()
    )
    .into();
}

pub fn update_fps_text(
    debug_ui_state: Res<resources::DebugUiState>,
    mut text: Single<&mut Text, With<components::FpsLabel>>,
    fps: Res<diagnostic::Fps>,
) {
    crate::guard_update!(debug_ui_state.is_enabled);
    **text = format!("Fps= {:.0}", fps.value).into();
}

pub fn update_camera_center_text(
    debug_ui_state: Res<resources::DebugUiState>,
    mut text: Single<&mut Text, With<components::CameraCenterLabel>>,
    player_view: Res<camera::resources::PlayerView>,
) {
    crate::guard_update!(debug_ui_state.is_changed() || player_view.is_changed());
    **text = format!(
        "Top Left={:.0}, Center={:.0}",
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
    let pos = map::components::Hex::from_world(mouse_pos.world);
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
    _trigger: On<input::events::ToggleDebugUi>,
    mut commands: Commands,
    debug_ui_state: Res<resources::DebugUiState>,
) {
    if !debug_ui_state.is_enabled {
        commands.trigger(events::ShowDebugUi);
    } else {
        commands.trigger(events::HideDebugUi);
    }
}
