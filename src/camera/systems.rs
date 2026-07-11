use super::resources;
use bevy::prelude::*;

const ZOOM: f32 = 10.0;
const INVERSE_ZOOM: f32 = 1.0 / ZOOM;
const CULLING_BUFFER: f32 = 100.0;

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

    let half_width = (viewport_size.x / 2.0) * INVERSE_ZOOM + CULLING_BUFFER;
    let half_height = (viewport_size.y / 2.0) * INVERSE_ZOOM + CULLING_BUFFER;

    let new_top_left = new_center - Vec2::new(half_width, half_height);
    let new_bottom_right = new_center + Vec2::new(half_width, half_height);

    crate::guard_update!(player_view.center != new_center || player_view.top_left != new_top_left);

    *player_view = resources::PlayerView {
        top_left: new_top_left,
        bot_right: new_bottom_right,
        center: new_center,
    };
}
