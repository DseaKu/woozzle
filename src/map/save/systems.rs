use super::resources::MapSaveData;
use bevy::prelude::*;
use std::fs;

const SAVE_PATH: &str = "saves/map.json";

enum LoadOutcome {
    Loaded(MapSaveData), // File exists and parsed
    NoSaveFile,          // First run, no save yet
    Corrupt,             // Read or parse failed
}

pub fn load_map_save(mut save_data: ResMut<MapSaveData>) {
    use LoadOutcome::*;

    let outcome = load_from_disk();

    match outcome {
        Loaded(data) => *save_data = data,
        NoSaveFile => {
            info!("No save file, starting a new map")
        }
        // TODO: Allow the user to decide whether to back up the corrupt data, overwrite it, or cancel the process.
        Corrupt => {
            warn!("Save file corrupt, starting a new map")
        }
    };
}

/// Attempts to read and parse the save file.
/// Distinguishes between missing file (first run) and read/parse errors.
fn load_from_disk() -> LoadOutcome {
    let json = match fs::read_to_string(SAVE_PATH) {
        Ok(json) => json,

        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return LoadOutcome::NoSaveFile;
        }

        Err(e) => {
            error!("Failed to read from disk: {e}");
            return LoadOutcome::Corrupt;
        }
    };

    match parse_save_data(&json) {
        Some(data) => LoadOutcome::Loaded(data),
        None => LoadOutcome::Corrupt,
    }
}

/// Deserializes JSON string into MapSaveData.
fn parse_save_data(json: &str) -> Option<MapSaveData> {
    match serde_json::from_str(json) {
        Ok(data) => Some(data),

        Err(e) => {
            error!("Failed to parse save file: {e}");
            None
        }
    }
}
