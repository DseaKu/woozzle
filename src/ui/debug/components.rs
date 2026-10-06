use bevy::prelude::*;

const LEFT_MARGIN: f32 = 20.0;
const TOP_MARGIN: f32 = 20.0;
const INDENTED_MARGIN: f32 = 30.0;

#[derive(Component)]
pub struct DebugUiRoot;

// Markers for the text lines the update_*_text systems write to

#[derive(Component)]
pub struct FpsLabel;

#[derive(Component)]
pub struct MouseWorldPosLabel;

#[derive(Component)]
pub struct MouseHexPosLabel;

#[derive(Component)]
pub struct CameraViewLabel;

#[derive(Component)]
pub struct TileCountLabel;

#[derive(Component)]
pub struct WoozzleCountLabel;

/// Top-left column holding all debug sections.
#[derive(Bundle)]
pub struct DebugUiRootBundle {
    node: Node,
    label: DebugUiRoot,
}

impl DebugUiRootBundle {
    pub fn new() -> Self {
        Self {
            node: Node {
                position_type: PositionType::Absolute,
                left: Val::Px(LEFT_MARGIN),
                top: Val::Px(TOP_MARGIN),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            label: DebugUiRoot,
        }
    }
}

#[derive(Bundle)]
pub struct SectionHeader {
    text: Text,
    node: Node,
}

impl SectionHeader {
    pub fn new(text: &str) -> Self {
        Self {
            text: Text::new(text),
            node: Node {
                flex_direction: FlexDirection::Column,
                ..default()
            },
        }
    }
}

/// Indented text line, tagged with marker `L` so it can be queried.
#[derive(Bundle)]
pub struct SectionItem<L: Component> {
    text: Text,
    node: Node,
    label: L,
}
impl<L: Component> SectionItem<L> {
    pub fn new(label: L) -> Self {
        Self {
            text: Text::default(),
            node: Node {
                flex_direction: FlexDirection::Column,
                margin: UiRect::left(Val::Px(INDENTED_MARGIN)),
                ..default()
            },
            label,
        }
    }
}
