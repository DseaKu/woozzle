use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

pub struct DebugUiPlugin;
impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(systems::toggle_debug_ui)
            .add_observer(systems::show_debug_ui)
            .add_observer(systems::hide_debug_ui)
            .add_observer(systems::update_mouse_parameters)
            .init_resource::<resources::DebugUiState>();
    }
}
