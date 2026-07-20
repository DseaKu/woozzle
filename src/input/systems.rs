use super::events;
use super::resources;
use bevy::prelude::*;
use bevy::window;

pub fn send_change_major_job_event(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        commands.trigger(events::ChangeMajorJob);
    }
}

pub fn send_toggle_debug_ui_event(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        commands.trigger(events::ToggleDebugUi);
    }
}

pub fn update_mouse_world_pos(
    mut mouse_pos: ResMut<resources::MousePos>,
    window: Single<&Window, With<window::PrimaryWindow>>,
    camera_q: Single<(&Camera, &GlobalTransform)>,
) {
    if let Some(cursor_pos) = window.cursor_position() {
        let (camera, camera_transform) = *camera_q;
        if let Ok(new_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            crate::guard_update!(mouse_pos.world != new_world_pos);

            mouse_pos.world = new_world_pos;
        }
    }
}

pub fn send_set_tile_event(mut commands: Commands, mouse_input: Res<ButtonInput<MouseButton>>) {
    if mouse_input.just_pressed(MouseButton::Left) {
        commands.trigger(events::SetTile);
    }
}

pub fn send_remove_tile_event(mut commands: Commands, mouse_input: Res<ButtonInput<MouseButton>>) {
    if mouse_input.just_pressed(MouseButton::Right) {
        commands.trigger(events::RemoveTile);
    }
}

pub fn send_spawn_woozle_event(mut commands: Commands, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::KeyE) {
        commands.trigger(events::SpawnWoozle);
    }
}
