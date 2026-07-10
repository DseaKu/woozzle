use bevy::prelude::*;

pub mod events;
pub mod resources;
mod systems;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::MousePos>()
            .add_systems(Update, systems::trigger_debug_ui_toggle)
            .add_systems(Update, systems::poll_mouse_left)
            .add_systems(Update, systems::update_mouse_world_pos);
    }
}
