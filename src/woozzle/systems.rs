use super::bundles;
use crate::input;
use bevy::prelude::*;

pub fn spawn_woozle(_trigger: On<input::events::SpawnWoozleEvent>, mut commands: Commands) {
    commands.spawn(bundles::Woozzle::new());
}
