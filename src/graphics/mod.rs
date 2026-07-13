use bevy::prelude::*;

mod components;
pub mod resources;
mod systems;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::load_tileset_assets)
            .add_systems(Startup, systems::load_woozzle_assets)
            .add_systems(Update, systems::despawn_tiles)
            .add_systems(Update, systems::spawn_tiles)
            .add_observer(systems::remove_woozzle_sprite)
            .add_observer(systems::insert_woozzle_sprite)
            .init_resource::<resources::WoozzleAsset>()
            .init_resource::<resources::TilesetAsset>()
            .init_resource::<resources::TileSpriteEntities>();
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
