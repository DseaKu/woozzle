use crate::input;
use bevy::prelude::*;

mod system;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_ui_toggle);
    }
}

fn handle_ui_toggle(_trigger: On<input::ToggleUiEvent>) {}
