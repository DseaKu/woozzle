use bevy::prelude::*;

mod debug_ui;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(debug_ui::DebugUiPlugin);
    }
}
