use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod camera;
mod diagnostic;
mod graphics;
mod input;
mod macros;
mod map;
mod ui;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            // Only for pixel art, otherwise texture will look blurred
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
    // Own Plugins
    .add_plugins(ui::UiPlugin)
    .add_plugins(input::InputPlugin)
    .add_plugins(camera::CameraPlugin)
    .add_plugins(map::MapPlugin)
    .add_plugins(graphics::GraphicsPlugin)
    .add_plugins(diagnostic::DiagnosticPlugin);

    // Third-Party Plugins
    if true {
        app.add_plugins(EguiPlugin::default())
            .add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}
