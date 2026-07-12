use super::resources;
use bevy::{input::mouse::MouseWheel, prelude::*};

const ZOOM: f32 = 10.0;
const CULLING_BUFFER: f32 = 200.0;
const CAMERA_SPEED: f32 = 200.0;

const ZOOM_SPEED: f32 = 0.05;
const MIN_SCALE: f32 = 0.05;
const MAX_SCALE: f32 = 0.25;

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

    // In your zoom_camera system
    for event in mouse_wheel_events.read() {
        let zoom_delta = event.y * ZOOM_SPEED;
        ortho.scale -= zoom_delta;

        // Snap to the nearest clean multiple (e.g., increments of 0.125 or 0.25)
        let step = 0.125;
        ortho.scale = (ortho.scale / step).round() * step;

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
) {
    let (camera, camera_transform, projection) = *camera_query;
    let new_center = camera_transform.translation().truncate();

    let Some(viewport_size) = camera.logical_viewport_size() else {
        return;
    };

    // Extract the dynamic scale from the projection
    let current_scale = if let Projection::Orthographic(ortho) = projection {
        ortho.scale
    } else {
        return;
    };

    let half_width = (viewport_size.x / 2.0) * current_scale + CULLING_BUFFER;
    let half_height = (viewport_size.y / 2.0) * current_scale + CULLING_BUFFER;

    let new_top_left = new_center - Vec2::new(half_width, half_height);
    let new_bottom_right = new_center + Vec2::new(half_width, half_height);

    crate::guard_update!(player_view.center != new_center || player_view.top_left != new_top_left);

    *player_view = resources::PlayerView {
        top_left: new_top_left,
        bot_right: new_bottom_right,
        center: new_center,
    };
}
