use super::events;
use super::resources;
use bevy::prelude::*;
use bevy::window;

pub fn trigger_debug_ui_toggle(keyboard_input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        commands.trigger(events::ToggleDebugUiEvent);
    }
}

pub fn update_mouse_world_pos(
    mut mouse_pos: ResMut<resources::MousePos>,
    window: Single<&Window, With<window::PrimaryWindow>>,
    camera_q: Single<(&Camera, &GlobalTransform)>,
) {
    if let Some(cursor_pos) = window.cursor_position() {
        let (camera, camera_transform) = *camera_q;
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            mouse_pos.world = world_pos;
        }
    }
}

pub fn poll_mouse_left(mut commands: Commands, mouse_input: Res<ButtonInput<MouseButton>>) {
    if mouse_input.just_pressed(MouseButton::Left) {
        commands.trigger(events::SetTileEvent);
    }
}
