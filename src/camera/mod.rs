use bevy::prelude::*;

pub mod resources;
mod systems;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::PlayerView>()
            .add_systems(Update, systems::update_player_view)
            .add_systems(Update, systems::move_camera)
            .add_systems(Update, systems::zoom_camera)
            .add_systems(Startup, systems::spawn_camera);
    }
}
