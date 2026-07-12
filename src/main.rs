use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod camera;
mod diagnostic;
mod graphics;
mod input;
mod macros;
mod map;
mod ui;
mod woozzle;

const IS_VSYNC_ENABLED: bool = true;
const IS_INSPECTOR_ENABLED: bool = false;

fn main() {
    let mut app = App::new();

    // Bevy Plugins
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            present_mode: if IS_VSYNC_ENABLED {
                bevy::window::PresentMode::AutoVsync
            } else {
                bevy::window::PresentMode::Immediate
            },
            ..default()
        }),
        ..default()
    };

    app.add_plugins(
        DefaultPlugins
            .set(window_plugin)
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
    // Own Plugins
    .add_plugins(ui::UiPlugin)
    .add_plugins(input::InputPlugin)
    .add_plugins(camera::CameraPlugin)
    .add_plugins(map::MapPlugin)
    .add_plugins(graphics::GraphicsPlugin)
    .add_plugins(woozzle::WoozzlePlugin)
    .add_plugins(diagnostic::DiagnosticPlugin);

    // Third-Party Plugins
    if IS_INSPECTOR_ENABLED {
        app.add_plugins(EguiPlugin::default())
            .add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}
