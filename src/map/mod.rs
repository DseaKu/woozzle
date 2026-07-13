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
            .add_observer(systems::update_viewport_hexes)
            .init_resource::<resources::VisibleHexes>()
            .init_resource::<resources::TileData>();
    }
}
