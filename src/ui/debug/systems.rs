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
        .spawn(DebugUiRootBundle::new())
        .with_children(|builder| {
            builder.spawn(SectionHeader::new("System:"));
            builder.spawn(SectionItem::new(FpsLabel));

            builder.spawn(SectionHeader::new("Mouse Position:"));
            builder.spawn(SectionItem::new(MouseWorldPosLabel));
            builder.spawn(SectionItem::new(MouseHexPosLabel));

            builder.spawn(SectionHeader::new("Camera:"));
            builder.spawn(SectionItem::new(CameraViewLabel));

            builder.spawn(SectionHeader::new("Entities:"));
            builder.spawn(SectionItem::new(TileCountLabel));
            builder.spawn(SectionItem::new(WoozzleCountLabel));
        });
    debug_ui_state.is_enabled = true;
}
// pub fn update_XXX_text(
//     debug_ui_state: Res<resources::DebugUiState>,
//     mut text: Single<&mut Text, With<components::XXXLabel>>,
// ) {
//     crate::return_unless!(debug_ui_state.is_enabled);
// **text = format!(
//     "Top Left= {}, Center: {}",
//     player_view.min, player_view.center
// )
// .into();
// }
pub fn update_woozzle_count_text(
    debug_ui_state: Res<resources::DebugUiState>,
    mut text: Single<&mut Text, With<components::WoozzleCountLabel>>,
    woozzles: Query<&ViewVisibility, With<woozzle::components::Woozzle>>,
) {
    crate::return_unless!(debug_ui_state.is_enabled);
    let total = woozzles.iter().len();
    let visible = woozzles.iter().filter(|v| v.get()).count();
    **text = format!("Woozzles: Total={}, Visible={}", total, visible).into();
}

pub fn update_tile_count_text(
    debug_ui_state: Res<resources::DebugUiState>,
    mut text: Single<&mut Text, With<components::TileCountLabel>>,
    tiles: Query<&ViewVisibility, With<map::components::TerrainType>>,
) {
    crate::return_unless!(debug_ui_state.is_enabled);
    let total = tiles.iter().len();
    let visible = tiles.iter().filter(|v| v.get()).count();
    **text = format!("Tiles: Total={}, Visible={}", total, visible).into();
}

pub fn update_fps_text(
    debug_ui_state: Res<resources::DebugUiState>,
    mut text: Single<&mut Text, With<components::FpsLabel>>,
    fps: Res<diagnostic::Fps>,
) {
    crate::return_unless!(debug_ui_state.is_enabled);
    **text = format!("Fps= {:.0}", fps.value).into();
}

pub fn update_camera_view_text(
    debug_ui_state: Res<resources::DebugUiState>,
    mut text: Single<&mut Text, With<components::CameraViewLabel>>,
    player_view: Res<camera::resources::PlayerView>,
) {
    crate::return_unless!(
        debug_ui_state.is_enabled && (debug_ui_state.is_changed() || player_view.is_changed())
    );
    **text = format!(
        "Min={:.0}, Max={:.0}, Center={:.0}",
        player_view.min, player_view.max, player_view.center
    )
    .into();
}

pub fn update_mouse_hex_pos_text(
    debug_ui_state: Res<resources::DebugUiState>,
    mut text: Single<&mut Text, With<components::MouseHexPosLabel>>,
    mouse_pos: Res<input::resources::MousePos>,
) {
    crate::return_unless!(
        debug_ui_state.is_enabled && (debug_ui_state.is_changed() || mouse_pos.is_changed())
    );
    let pos = map::components::Hex::from_world(mouse_pos.world);
    **text = format!("Hex q={}, r={}", pos.q, pos.r).into();
}

pub fn update_mouse_world_pos_text(
    debug_ui_state: Res<resources::DebugUiState>,
    mut text: Single<&mut Text, With<components::MouseWorldPosLabel>>,
    mouse_pos: Res<input::resources::MousePos>,
) {
    crate::return_unless!(
        debug_ui_state.is_enabled && (debug_ui_state.is_changed() || mouse_pos.is_changed())
    );
    let pos = mouse_pos.world;
    **text = format!("World x={:.2}, y={:.2}", pos.x, pos.y).into();
}

pub fn hide_debug_ui(
    _trigger: On<events::HideDebugUi>,
    mut commands: Commands,
    root_node_entity: Single<Entity, With<components::DebugUiRoot>>,
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
