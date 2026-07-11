use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(systems::set_tile)
            .add_observer(systems::remove_tiles)
            .add_systems(Update, systems::update_visible_tiles)
            .init_resource::<resources::VisibleTiles>()
            .init_resource::<resources::MapData>();
    }
}
