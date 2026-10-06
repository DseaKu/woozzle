use bevy::prelude::*;

/// Fired after `WoozzlesByHex` was rebuilt or extended.
#[derive(Event)]
pub struct WoozzlesByHexUpdated;
