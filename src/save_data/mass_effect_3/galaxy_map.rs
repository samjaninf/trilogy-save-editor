use serde::Serialize;

use crate::save_data::common::Vector2d;

#[derive(Serialize, SaveData, Clone)]
pub struct GalaxyMap {
    planets: Vec<Planet>,
    systems: Vec<System>,
}

#[derive(Serialize, SaveData, Default, Clone)]
pub struct Planet {
    id: i32,
    visited: bool,
    probes: Vec<Vector2d>,
    show_as_scanned: bool,
}

#[derive(Serialize, SaveData, Default, Clone)]
pub struct System {
    id: i32,
    reaper_alert_level: f32,
    reaper_detected: bool,
}
