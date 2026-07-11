use bevy::prelude::*;

mod camera;
mod graphics;
mod input;
mod macros;
mod map;
mod ui;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                // Only for pixel art, otherwise texture will look blurred
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(ui::UiPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(map::MapPlugin)
        .add_plugins(graphics::GraphicsPlugin)
        .run();
}
