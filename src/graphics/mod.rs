use bevy::prelude::*;

pub mod components;
pub mod resources;
mod loader;
mod systems;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, loader::load_tileset_assets)
            .add_systems(Startup, loader::load_woozzle_assets)
            .add_systems(Update, systems::animate_sprites)
            .add_observer(systems::remove_tile_sprite)
            .add_observer(systems::insert_tile_sprite)
            .add_observer(systems::remove_woozzle_sprite)
            .add_observer(systems::insert_woozzle_sprite)
            .init_resource::<resources::WoozzleAsset>()
            .init_resource::<resources::TilesetAsset>();
    }
}

pub enum DrawOrder {
    Ground,
    OnGround,
}

impl DrawOrder {
    pub fn as_f32(&self) -> f32 {
        match self {
            DrawOrder::Ground => 0.0,
            DrawOrder::OnGround => 1.0,
        }
    }
}
