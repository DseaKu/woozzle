use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

pub struct SavePlugin;
impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::MapSaveData>()
            .add_systems(Startup, systems::load_map_save);
    }
}
