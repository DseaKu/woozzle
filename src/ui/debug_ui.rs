use bevy::prelude::*;

use crate::input;
const LEFT_MARGIN: f32 = 20.0;
const TOP_MARGIN: f32 = 20.0;

pub struct DebugUiPlugin;
impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_ui_toggle)
            .add_observer(spawn_debug_ui)
            .init_resource::<DebugUiState>()
            .add_observer(despawn_debug_ui);
    }
}

#[derive(Component)]
struct DebugUiLabel;

#[derive(Bundle)]
struct DebugUiBundle {
    node: Node,
    text: Text,
    label: DebugUiLabel,
}
impl DebugUiBundle {
    fn new() -> Self {
        Self {
            node: Node {
                position_type: PositionType::Absolute,
                left: Val::Px(LEFT_MARGIN),
                top: Val::Px(TOP_MARGIN),
                ..default()
            },
            text: Text::new("test"),
            label: DebugUiLabel,
        }
    }
}

#[derive(Resource, Default)]
pub struct DebugUiState {
    pub is_enabled: bool,
}

pub fn handle_ui_toggle(
    _trigger: On<input::ToggleDebugUiEvent>,
    mut commands: Commands,
    debug_ui_state: Res<DebugUiState>,
) {
    if !debug_ui_state.is_enabled {
        println!("spawn");
        commands.trigger(PendingSpawnDebugUiEvent);
    } else {
        println!("despawn");
        commands.trigger(PendingDespawnDebugUiEvent);
    }
}

#[derive(Event)]
struct PendingSpawnDebugUiEvent;
fn spawn_debug_ui(
    _trigger: On<PendingSpawnDebugUiEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut debug_ui_state: ResMut<DebugUiState>,
) {
    let _font = FontSource::from(asset_server.load("fonts/Hack-Regular.ttf"));
    commands.spawn(DebugUiBundle::new());
    debug_ui_state.is_enabled = true;
}

#[derive(Event)]
struct PendingDespawnDebugUiEvent;
fn despawn_debug_ui(
    _trigger: On<PendingDespawnDebugUiEvent>,
    mut commands: Commands,
    debug_ui_entity: Single<Entity, With<DebugUiLabel>>,
    mut debug_ui_state: ResMut<DebugUiState>,
) {
    commands.entity(*debug_ui_entity).despawn();
    debug_ui_state.is_enabled = false;
}
