use bevy::prelude::*;

pub mod components;
mod systems;

#[derive(Component)]
pub struct JobsPlugin;
impl Plugin for JobsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::process_job);
    }
}
