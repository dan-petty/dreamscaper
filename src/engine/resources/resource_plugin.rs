//! Environmental resources, diurnal cycle clock, and asset registrations.

use bevy::prelude::*;
use bevy::reflect::TypePath;
use serde::{Deserialize, Serialize};

/// Diurnal cycle clock managing day/night progression.
#[derive(Resource, Debug, Clone, Reflect)]
pub struct GameTimeOfDay {
    /// Elapsed in-game time in seconds within current cycle.
    pub current_time: f32,
    /// Full length of one complete 24-hour cycle in real-world seconds.
    pub day_length_seconds: f32,
    /// Normalized time of day (0.0 = dawn, 0.25 = noon, 0.5 = dusk, 0.75 = midnight).
    pub normalized_time: f32,
}

impl Default for GameTimeOfDay {
    fn default() -> Self {
        Self {
            current_time: 30.0, // Start around morning
            day_length_seconds: 120.0,
            normalized_time: 0.25,
        }
    }
}

impl GameTimeOfDay {
    pub fn is_daytime(&self) -> bool {
        self.normalized_time >= 0.2 && self.normalized_time <= 0.7
    }
}

/// Global environmental ambient settings.
#[derive(Resource, Debug, Clone)]
pub struct EnvironmentSettings {
    pub ambient_light: Vec3,
    pub base_moisture: f32,
    pub base_temperature: f32,
}

impl Default for EnvironmentSettings {
    fn default() -> Self {
        Self {
            ambient_light: Vec3::new(1.0, 1.0, 1.0),
            base_moisture: 0.5,
            base_temperature: 22.0,
        }
    }
}

/// Custom asset definition representing loaded map data.
#[derive(Asset, TypePath, Debug, Clone, Serialize, Deserialize)]
pub struct TileMapAsset {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub tile_size: [f32; 2],
    pub default_biome: String,
}

/// Resource plugin registering environment clocks and custom assets.
pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameTimeOfDay>()
            .init_resource::<EnvironmentSettings>()
            .init_asset::<TileMapAsset>()
            .add_systems(Update, advance_time_of_day);
    }
}

/// Advances the diurnal clock and adjusts ambient lighting.
fn advance_time_of_day(
    time: Res<Time>,
    mut clock: ResMut<GameTimeOfDay>,
    mut env: ResMut<EnvironmentSettings>,
) {
    clock.current_time = (clock.current_time + time.delta_seconds()) % clock.day_length_seconds;
    clock.normalized_time = clock.current_time / clock.day_length_seconds;

    // Modulate ambient lighting based on solar curve
    let sun_factor = (clock.normalized_time * std::f32::consts::TAU)
        .sin()
        .max(0.1);
    env.ambient_light = Vec3::splat(0.2 + (sun_factor * 0.8));
}
