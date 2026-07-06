use bevy::prelude::*;

mod camera;
mod input;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ui::UiPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(camera::CameraPlugin)
        .run();
}
