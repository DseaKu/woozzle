use super::resources;
use bevy::prelude::*;

const ZOOM: f32 = 3.0;
const INVERSE_ZOOM: f32 = 1.0 / ZOOM;
const CULLING_BUFFER: f32 = 100.0;
const CAMERA_SCALE: f32 = INVERSE_ZOOM + CULLING_BUFFER;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 1. / ZOOM,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

pub fn update_player_view(
    mut player_view: ResMut<resources::PlayerView>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = *camera_query;
    let new_center = camera_transform.translation().truncate();

    let Some(viewport_size) = camera.logical_viewport_size() else {
        return;
    };

    let half_width = (viewport_size.x / 2.0) * CAMERA_SCALE;
    let half_height = (viewport_size.y / 2.0) * CAMERA_SCALE;

    let new_top_left = new_center - Vec2::new(half_width, half_height);
    let new_bottom_right = new_center + Vec2::new(half_width, half_height);

    crate::guard_update!(player_view.center != new_center || player_view.top_left != new_top_left);

    *player_view = resources::PlayerView {
        top_left: new_top_left,
        bot_right: new_bottom_right,
        center: new_center,
    };
}

// let mut visible_tiles = Vec::new();
// for q in (top_left_hex.q - 1)..=(bottom_right_hex.q + 1) {
//     for r in (top_left_hex.r - 1)..=(bottom_right_hex.r + 1) {
//         visible_tiles.push(Hex::new(q, r));
//     }
// }
