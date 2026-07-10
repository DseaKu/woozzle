use super::resources;
use bevy::prelude::*;
const PADDING: f32 = 100.0;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn update_player_view(
    mut player_view: ResMut<resources::PlayerView>,
    camera: Single<&Camera>,
    camera_transform: Single<&GlobalTransform>,
) {
    let Some(viewport_size) = camera.logical_viewport_size() else {
        return;
    };
    let center = camera_transform.translation().truncate();

    let half_width = (viewport_size.x / 2.0) + PADDING;
    let half_height = (viewport_size.y / 2.0) + PADDING;

    *player_view = resources::PlayerView {
        top_left: center - Vec2::new(half_width, half_height),
        bottom_right: center + Vec2::new(half_width, half_height),
        center,
    };
}

// let mut visible_tiles = Vec::new();
// for q in (top_left_hex.q - 1)..=(bottom_right_hex.q + 1) {
//     for r in (top_left_hex.r - 1)..=(bottom_right_hex.r + 1) {
//         visible_tiles.push(Hex::new(q, r));
//     }
// }
