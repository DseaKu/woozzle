use bevy::prelude::*;

/// Fired after a tile was placed or removed.
#[derive(Event)]
pub struct TilesChanged;
