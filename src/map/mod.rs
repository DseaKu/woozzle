use bevy::prelude::*;

pub mod bundles;
pub mod components;
pub mod resources;
mod systems;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(systems::set_tile)
            .add_observer(systems::remove_tiles)
            .add_systems(Update, systems::update_viewport_hexes)
            .init_resource::<resources::ViewportHexes>()
            .init_resource::<resources::MapData>();
    }
}
