use super::events;
use super::resources;
use bevy::prelude::*;
use bevy::window;

#[allow(dead_code)]
pub fn trigger_open_radial_menu(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
) {
    if mouse_input.pressed(MouseButton::Right) {
        commands.trigger(events::OpenRadialMenu);
    }
}

pub fn trigger_toggle_job_mode(keyboard_input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        commands.trigger(events::ToggleJobMode);
    }
}

pub fn trigger_toggle_debug_ui(keyboard_input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
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
            crate::return_unless!(mouse_pos.world != new_world_pos);

            mouse_pos.world = new_world_pos;
        }
    }
}

pub fn trigger_place_tile(mut commands: Commands, mouse_input: Res<ButtonInput<MouseButton>>) {
    if mouse_input.just_pressed(MouseButton::Left) {
        commands.trigger(events::PlaceTile);
    }
}

pub fn trigger_remove_tile(mut commands: Commands, mouse_input: Res<ButtonInput<MouseButton>>) {
    if mouse_input.just_pressed(MouseButton::Middle) {
        commands.trigger(events::RemoveTile);
    }
}

pub fn trigger_spawn_woozzle(mut commands: Commands, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::KeyE) {
        commands.trigger(events::SpawnWoozzle);
    }
}
