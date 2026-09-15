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

use crate::engine::ecs::components::TileType;

/// Visual and ecological theme dictating landscape look and feel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TerrainTheme {
    #[default]
    Verdant,
    Celestial,
    Obsidian,
    Autumnal,
}

/// Comprehensive configurable parameters directing procedural scene generation.
#[derive(Resource, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TerrainConfig {
    pub width: u32,
    pub height: u32,
    pub wave_expansion_speed: f32,
    pub elevation_scale: f32,
    pub frequency: f32,
    pub roughness: f32,
    pub water_threshold: f32,
    pub sand_threshold: f32,
    pub meadow_threshold: f32,
    pub forest_threshold: f32,
    pub mountain_threshold: f32,
    pub feature_density: f32,
    pub theme: TerrainTheme,
}

impl Default for TerrainConfig {
    fn default() -> Self {
        Self {
            width: 32,
            height: 32,
            wave_expansion_speed: 3.0,
            elevation_scale: 3.0,
            frequency: 0.12,
            roughness: 0.5,
            water_threshold: 0.20,
            sand_threshold: 0.28,
            meadow_threshold: 0.65,
            forest_threshold: 0.85,
            mountain_threshold: 0.93,
            feature_density: 0.40,
            theme: TerrainTheme::Verdant,
        }
    }
}

/// Deterministic 2D procedural landscape evaluation based on configurable parameters.
pub fn evaluate_terrain_sample(x: f32, y: f32, config: &TerrainConfig) -> (i32, TileType, f32) {
    let freq = config.frequency;
    // Multi-octave harmonic combination
    let oct1 = ((x * freq).sin() * (y * freq).cos() + 1.0) * 0.5;
    let oct2 = (((x * 2.3 * freq) + 1.3).cos() * ((y * 2.1 * freq) + 2.7).sin() + 1.0) * 0.5;
    let raw_val = oct1 * (1.0 - config.roughness) + oct2 * config.roughness;
    let clamped_val = raw_val.clamp(0.0, 1.0);

    let (elevation, tile_type) = if clamped_val < config.water_threshold {
        (0, TileType::Water)
    } else if clamped_val < config.sand_threshold {
        (0, TileType::Sand)
    } else if clamped_val < config.meadow_threshold {
        (1, TileType::Meadow)
    } else if clamped_val < config.forest_threshold {
        (1, TileType::Forest)
    } else {
        let max_elev = config.elevation_scale.round() as i32;
        let elev = 1
            + (((clamped_val - config.forest_threshold)
                / (1.0 - config.forest_threshold).max(0.01))
                * (max_elev.max(1) as f32))
                .round() as i32;
        (elev, TileType::Stone)
    };

    (elevation, tile_type, clamped_val)
}

/// Resource plugin registering environment clocks, terrain configs, and custom assets.
pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameTimeOfDay>()
            .init_resource::<EnvironmentSettings>()
            .init_resource::<TerrainConfig>()
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
    clock.current_time = (clock.current_time + time.delta_secs()) % clock.day_length_seconds;
    clock.normalized_time = clock.current_time / clock.day_length_seconds;

    // Modulate ambient lighting based on solar curve
    let sun_factor = (clock.normalized_time * std::f32::consts::TAU)
        .sin()
        .max(0.1);
    env.ambient_light = Vec3::splat(0.2 + (sun_factor * 0.8));
}
