use bevy::prelude::*;

pub mod events;
pub mod resources;
mod systems;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::MousePos>()
            .add_systems(Update, systems::send_toggle_debug_ui_event)
            .add_systems(Update, systems::send_set_tile_event)
            .add_systems(Update, systems::send_remove_tile_event)
            .add_systems(Update, systems::send_spawn_woozle_event)
            .add_systems(Update, systems::send_change_major_job_event)
            .add_systems(Update, systems::update_mouse_world_pos);
    }
}
