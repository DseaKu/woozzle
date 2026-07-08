use super::*;
use crate::input;

#[derive(Resource, Default)]
struct DebugUiState {
    is_enabled: bool,
}
pub struct EventPlugin;
impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(toggle_debug_ui)
            .add_observer(show_debug_ui)
            .add_observer(hide_debug_ui)
            .init_resource::<DebugUiState>();
    }
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

#[derive(Event)]
struct ShowDebugUi;
fn show_debug_ui(
    _trigger: On<ShowDebugUi>,
    mut commands: Commands,
    mut debug_ui_state: ResMut<DebugUiState>,
) {
    commands
        .spawn(RootNodeBundle::new())
        .with_children(|builder| {
            builder.spawn(SubNode::new("Mouse"));
            builder.spawn(SubNode::new_indented("World Pos:"));
            builder.spawn(SubNode::new_indented("Hex Pos:"));
        });
    debug_ui_state.is_enabled = true;
}
