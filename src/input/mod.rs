use bevy::prelude::*;

pub mod events;
pub mod resources;
mod systems;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::MousePos>()
            .add_systems(Update, systems::trigger_toggle_debug_ui)
            .add_systems(Update, systems::trigger_place_tile)
            .add_systems(Update, systems::trigger_remove_tile)
            .add_systems(Update, systems::trigger_spawn_woozzle)
            .add_systems(Update, systems::trigger_toggle_job_mode)
            .add_systems(Update, systems::update_mouse_world_pos);
    }
}
