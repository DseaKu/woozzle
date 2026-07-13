use super::resources;
use crate::map;
use bevy::{input::mouse::MouseWheel, prelude::*};

const ZOOM: f32 = 10.0;
const CULLING_FACTOR: f32 = -0.80; // 10% of screen size
const CAMERA_SPEED: f32 = 200.0;

const ZOOM_SPEED: f32 = 0.05;
const ZOOM_STEP_SIZE: f32 = 0.0625;
const MIN_SCALE: f32 = ZOOM_STEP_SIZE * 2.0;
const MAX_SCALE: f32 = ZOOM_STEP_SIZE * 4.0;

pub fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera_transform: Single<&mut Transform, With<Camera2d>>,
) {
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    // Apply the movement if any keys were pressed
    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
        camera_transform.translation += direction * CAMERA_SPEED * time.delta_secs();
    }
}

pub fn zoom_camera(
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    camera_query: Single<&mut Projection, With<Camera2d>>,
) {
    let mut projection = camera_query.into_inner();
    let Projection::Orthographic(ortho) = projection.as_mut() else {
        return;
    };

    for event in mouse_wheel_events.read() {
        let zoom_delta = event.y * ZOOM_SPEED;
        ortho.scale -= zoom_delta;

        ortho.scale = (ortho.scale / ZOOM_STEP_SIZE).round() * ZOOM_STEP_SIZE;

        ortho.scale = ortho.scale.clamp(MIN_SCALE, MAX_SCALE);
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Msaa::Off,
        Projection::Orthographic(OrthographicProjection {
            scale: 1. / ZOOM,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

pub fn update_player_view(
    mut player_view: ResMut<resources::PlayerView>,
    camera_query: Single<(&Camera, &GlobalTransform, &Projection)>,
    mut commands: Commands,
) {
    let (camera, camera_transform, projection) = *camera_query;
    let new_center = camera_transform.translation().truncate();

    let Some(viewport_size) = camera.logical_viewport_size() else {
        return;
    };

    let current_scale = if let Projection::Orthographic(ortho) = projection {
        ortho.scale
    } else {
        return;
    };

    let half_width = (viewport_size.x / 2.0) * (1.0 + CULLING_FACTOR) * current_scale;
    let half_height = (viewport_size.y / 2.0) * (1.0 + CULLING_FACTOR) * current_scale;

    let new_top_left = new_center - Vec2::new(half_width, half_height);
    let new_bottom_right = new_center + Vec2::new(half_width, half_height);

    crate::guard_update!(player_view.center != new_center || player_view.top_left != new_top_left);

    *player_view = resources::PlayerView {
        top_left: new_top_left,
        bot_right: new_bottom_right,
        center: new_center,
    };

    commands.trigger(super::events::PlayerViewUpdated);
}

pub fn update_viewport_hexes(
    _trigger: On<super::events::PlayerViewUpdated>,
    mut visible_hexes: ResMut<super::resources::VisibleHexes>,
    player_view: Res<super::resources::PlayerView>,
    mut commands: Commands,
) {
    use map::components::Hex;

    // Check all 4 corners to find the true min/max of the slanted hex grid
    let top_left = player_view.top_left;
    let bot_right = player_view.bot_right;
    let top_right = Vec2::new(bot_right.x, top_left.y);
    let bot_left = Vec2::new(top_left.x, bot_right.y);

    let corners = [
        Hex::from_world(top_left),
        Hex::from_world(bot_right),
        Hex::from_world(top_right),
        Hex::from_world(bot_left),
    ];

    let mut min_q = i32::MAX;
    let mut max_q = i32::MIN;
    let mut min_r = i32::MAX;
    let mut max_r = i32::MIN;

    for hex in &corners {
        min_q = min_q.min(hex.q);
        max_q = max_q.max(hex.q);
        min_r = min_r.min(hex.r);
        max_r = max_r.max(hex.r);
    }

    visible_hexes.tiles.clear();
    for q in (min_q - 1)..=(max_q + 1) {
        for r in (min_r - 1)..=(max_r + 1) {
            visible_hexes.tiles.push(Hex::new(q, r));
        }
    }
    commands.trigger(super::events::VisibleHexesUpdated);
}
