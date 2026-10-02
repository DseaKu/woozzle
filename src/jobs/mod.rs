use bevy::prelude::*;

mod actions;
pub mod components;
pub mod planners;
mod systems;

#[derive(Component)]
pub struct JobsPlugin;
impl Plugin for JobsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, systems::start_next_action)
            .add_systems(FixedUpdate, actions::run_wait)
            .add_systems(FixedUpdate, actions::tick_ghost_mode)
            .add_systems(FixedUpdate, actions::run_go_to_point);
    }
}
