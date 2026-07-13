use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

const FPS_REFRESH_INTERVAL: u64 = 1;

pub struct DebugUiPlugin;
impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(systems::toggle_debug_ui)
            .add_observer(systems::show_debug_ui)
            .add_observer(systems::hide_debug_ui)
            // Attach the run condition here
            .add_systems(
                Update,
                systems::update_fps_text
                    .run_if(on_timer(Duration::from_secs(FPS_REFRESH_INTERVAL))),
            )
            .add_systems(Update, systems::update_mouse_hex_pos_text)
            .add_systems(Update, systems::update_mouse_world_pos_text)
            .add_systems(Update, systems::update_camera_center_text)
            .add_systems(Update, systems::update_woozzle_entity_text)
            .add_systems(Update, systems::update_tile_entity_text)
            .init_resource::<resources::DebugUiState>();
    }
}
