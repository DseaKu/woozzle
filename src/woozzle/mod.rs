use crate::camera;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

pub mod bundles;
pub mod components;
pub mod events;
pub mod resources;
mod systems;

use std::time::Duration;

const FACE_DIR_INTERVAL: f32 = 5.0;
const UPDATE_HEX_INTERVAL: f32 = 1.0;

pub struct WoozzlePlugin;
impl Plugin for WoozzlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::Data>()
            .add_systems(Update, systems::get_a_job)
            .add_systems(Update, systems::update_sprite_facing)
            .add_systems(
                Update,
                systems::mark_all_face_dir_dirty
                    .run_if(on_timer(Duration::from_secs_f32(FACE_DIR_INTERVAL))),
            )
            .add_systems(
                Update,
                systems::update_woozzle_hex_data
                    .run_if(on_timer(Duration::from_secs_f32(UPDATE_HEX_INTERVAL))),
            )
            .init_resource::<resources::Visible>()
            .init_resource::<resources::MajorJobFlag>()
            .add_observer(systems::change_major_job)
            .add_observer(systems::update_visible_woozzles::<events::DataUpdated>)
            .add_observer(systems::update_visible_woozzles::<camera::events::VisibleHexesUpdated>)
            .add_observer(systems::set_woozle);
    }
}
