use bevy::prelude::*;

pub mod events;
pub mod resources;
mod system;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::MousePos>()
            .add_systems(Update, system::trigger_debug_ui_toggle)
            .add_systems(Update, system::poll_mouse_left)
            .add_systems(Update, system::update_mouse_world_pos);
    }
}
