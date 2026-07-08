use super::*;
use crate::input;

#[derive(Resource, Default)]
struct DebugUiState {
    is_enabled: bool,
}

#[derive(Resource, Default)]
struct MouseParameters {
    world_pos: Option<Vec2>,
    hex_pos: Option<Vec2>,
}

pub struct SystemPlugin;
impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(toggle_debug_ui)
            .add_observer(show_debug_ui)
            .add_observer(hide_debug_ui)
            .add_observer(fetch_mouse_parameters)
            .init_resource::<DebugUiState>()
            .init_resource::<MouseParameters>();
    }
}

#[derive(Event)]
struct ShowDebugUi;
fn show_debug_ui(
    _trigger: On<ShowDebugUi>,
    mut commands: Commands,
    mut debug_ui_state: ResMut<DebugUiState>,
    mouse_parameters: Res<MouseParameters>,
) {
    let mouse_world_pos = mouse_parameters.world_pos.unwrap_or_default();
    commands
        .spawn(RootNodeBundle::new())
        .with_children(|builder| {
            builder.spawn(SubNode::new("Mouse"));
            builder.spawn(SubNode::new_indented(&format!(
                "World Pos: x={}, y={}",
                mouse_world_pos.x, mouse_world_pos.y
            )));
            builder.spawn(SubNode::new_indented("Hex Pos:"));
        });
    debug_ui_state.is_enabled = true;
}

fn fetch_mouse_parameters(
    _trigger: On<ShowDebugUi>,
    mut mouse_parameters: ResMut<MouseParameters>,
) {
}

fn toggle_debug_ui(
    _trigger: On<input::ToggleDebugUiEvent>,
    mut commands: Commands,
    debug_ui_state: Res<DebugUiState>,
) {
    if !debug_ui_state.is_enabled {
        commands.trigger(ShowDebugUi);
    } else {
        commands.trigger(HideDebugUi);
    }
}

#[derive(Event)]
struct HideDebugUi;
fn hide_debug_ui(
    _trigger: On<HideDebugUi>,
    mut commands: Commands,
    root_node_entity: Single<Entity, With<RootNodeLabel>>,
    mut debug_ui_state: ResMut<DebugUiState>,
) {
    commands.entity(*root_node_entity).despawn();
    debug_ui_state.is_enabled = false;
}
