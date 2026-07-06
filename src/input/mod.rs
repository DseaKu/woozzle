use bevy::prelude::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, trigger_debug_ui_toggle);
    }
}

#[derive(Event)]
pub struct ToggleDebugUiEvent;

fn trigger_debug_ui_toggle(keyboard_input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        commands.trigger(ToggleDebugUiEvent);
    }
}
