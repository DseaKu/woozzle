use crate::ui::debug;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct DiagnosticPlugin;
impl Plugin for DiagnosticPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_fps).init_resource::<Fps>();
    }
}

#[derive(Resource, Default)]
pub struct Fps {
    pub value: f64,
}

fn update_fps(
    mut fps: ResMut<Fps>,
    diagnostics: Res<DiagnosticsStore>,
    debug_ui_state: Res<debug::resources::DebugUiState>,
) {
    crate::guard_update!(debug_ui_state.is_enabled);
    if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
        && let Some(fps_value) = fps_diagnostic.smoothed()
    {
        fps.value = fps_value;
    }
}
