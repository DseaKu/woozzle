use bevy::prelude::*;

pub mod components;
mod loader;
pub mod resources;
mod systems;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, loader::load_tileset_atlas)
            .add_systems(Startup, loader::load_woozzle_atlas)
            .add_systems(Update, systems::animate_sprites)
            .add_observer(systems::insert_tile_sprite)
            .add_observer(systems::insert_woozzle_sprite)
            .init_resource::<resources::WoozzleAtlas>()
            .init_resource::<resources::TilesetAtlas>();
    }
}

pub enum DrawOrder {
    Ground,
    OnGround,
}

impl DrawOrder {
    pub fn z(&self) -> f32 {
        match self {
            DrawOrder::Ground => 0.0,
            DrawOrder::OnGround => 1.0,
        }
    }
}
