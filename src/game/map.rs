//! Map generation, progressive terrain genesis, and environmental reactivity.

use crate::engine::ecs::components::{
    DynamicTile, Explorer, FeatureType, GridPosition, IsometricCoordinates, LandscapeFeature,
    TerrainDevelopmentState, TileType,
};
use crate::engine::resources::{
    evaluate_terrain_sample, GameTimeOfDay, TerrainConfig, TerrainTheme,
};
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerrainExpansionWave>()
            .add_systems(Startup, generate_map)
            .add_systems(
                Update,
                (progressive_terrain_genesis_system, update_tile_conditions),
            );
    }
}

/// Resource tracking the radial crystallization wave emitted from the explorer.
#[derive(Resource, Debug, Clone)]
pub struct TerrainExpansionWave {
    pub elapsed_time: f32,
    pub active: bool,
}

impl Default for TerrainExpansionWave {
    fn default() -> Self {
        Self {
            elapsed_time: 0.0,
            active: true,
        }
    }
}

/// Deterministic pseudo-random hash in [0.0, 1.0] for reproducible tile variance.
#[inline]
pub fn tile_hash(x: i32, y: i32) -> f32 {
    let seed = ((x.wrapping_mul(73856093)) ^ (y.wrapping_mul(19349663))) & 0xFFFF;
    (seed as f32) / 65535.0
}

/// Pure helper mapping tile type and theme to a natural landscape feature.
pub fn select_landscape_feature(
    theme: TerrainTheme,
    tile_type: TileType,
    hash: f32,
) -> Option<FeatureType> {
    match (theme, tile_type) {
        (TerrainTheme::Celestial, TileType::Meadow | TileType::Stone) => {
            Some(FeatureType::CrystalCluster)
        }
        (TerrainTheme::Obsidian, TileType::Stone) => Some(FeatureType::AncientStone),
        (TerrainTheme::Obsidian, TileType::Water) => Some(FeatureType::SpringWater),
        (_, TileType::Forest) => Some(FeatureType::Tree),
        (_, TileType::Meadow) if hash < 0.5 => Some(FeatureType::WildFlora),
        (_, TileType::Meadow) => Some(FeatureType::Tree),
        (_, TileType::Stone) => Some(FeatureType::AncientStone),
        (_, TileType::Water) => Some(FeatureType::SpringWater),
        _ => None,
    }
}

/// Initializes the map grid with the centered origin detailed and outer cells latent.
fn generate_map(mut commands: Commands, config: Res<TerrainConfig>) {
    let width = config.width as i32;
    let height = config.height as i32;
    let center_x = width / 2;
    let center_y = height / 2;

    for x in 0..width {
        for y in 0..height {
            let is_center = x == center_x && y == center_y;
            let (elevation, tile_type, raw_val) =
                evaluate_terrain_sample(x as f32, y as f32, &config);

            let (initial_elevation, initial_state) = if is_center {
                (elevation, TerrainDevelopmentState::Detailed)
            } else {
                (0, TerrainDevelopmentState::Latent)
            };

            let moisture = match tile_type {
                TileType::Water => 1.0,
                TileType::Sand => 0.3,
                TileType::Forest => 0.8,
                TileType::Meadow => 0.6,
                _ => 0.4,
            };

            commands.spawn((
                GridPosition::new(x, y, initial_elevation),
                IsometricCoordinates::default(),
                DynamicTile {
                    tile_type,
                    moisture,
                    temperature: 20.0 + (raw_val * 4.0),
                    elevation: initial_elevation as f32,
                },
                initial_state,
                Transform::from_xyz(0.0, 0.0, 0.0),
                GlobalTransform::default(),
                Name::new(format!("Tile_{}_{}", x, y)),
            ));
        }
    }
}

/// Drives the radial crystallization wave outward from the Explorer over time.
fn progressive_terrain_genesis_system(
    time: Res<Time>,
    config: Res<TerrainConfig>,
    mut wave: ResMut<TerrainExpansionWave>,
    explorer_query: Query<&GridPosition, With<Explorer>>,
    mut tile_query: Query<
        (
            Entity,
            &mut GridPosition,
            &mut DynamicTile,
            &mut TerrainDevelopmentState,
        ),
        Without<Explorer>,
    >,
    mut commands: Commands,
) {
    if !wave.active {
        return;
    }

    let delta = time.delta_secs();
    wave.elapsed_time += delta;

    let (origin_x, origin_y) = explorer_query
        .iter()
        .next()
        .map(|pos| (pos.x as f32, pos.y as f32))
        .unwrap_or_else(|| (config.width as f32 * 0.5, config.height as f32 * 0.5));

    let wave_radius = 1.0 + (config.wave_expansion_speed * wave.elapsed_time);

    for (entity, mut grid_pos, mut tile, mut dev_state) in tile_query.iter_mut() {
        if dev_state.is_detailed() {
            continue;
        }

        let dx = grid_pos.x as f32 - origin_x;
        let dy = grid_pos.y as f32 - origin_y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > wave_radius {
            continue;
        }

        let current_progress = match *dev_state {
            TerrainDevelopmentState::Latent => 0.0,
            TerrainDevelopmentState::Forming(p) => p,
            TerrainDevelopmentState::Detailed => 1.0,
        };

        let new_progress = (current_progress + delta * 2.0).min(1.0);
        let (target_elevation, target_tile_type, _) =
            evaluate_terrain_sample(grid_pos.x as f32, grid_pos.y as f32, &config);

        if new_progress >= 1.0 {
            *dev_state = TerrainDevelopmentState::Detailed;
            grid_pos.elevation = target_elevation;
            tile.elevation = target_elevation as f32;
            tile.tile_type = target_tile_type;

            // Spawn landscape feature if density check passes
            let hash = tile_hash(grid_pos.x, grid_pos.y);
            if hash < config.feature_density {
                if let Some(feature_type) =
                    select_landscape_feature(config.theme, target_tile_type, hash)
                {
                    commands.entity(entity).with_children(|parent| {
                        parent.spawn((
                            LandscapeFeature {
                                feature_type,
                                scale: 0.75 + (hash * 0.4),
                                variant: ((hash * 100.0) as u32) % 4,
                            },
                            Transform::from_xyz(0.0, 16.0, 0.5),
                            GlobalTransform::default(),
                            Name::new(format!("Feature_{:?}", feature_type)),
                        ));
                    });
                }
            }
        } else {
            *dev_state = TerrainDevelopmentState::Forming(new_progress);
            let rising_elev = ((target_elevation as f32) * new_progress).round() as i32;
            grid_pos.elevation = rising_elev;
            tile.elevation = rising_elev as f32;
        }
    }
}

/// Updates tile moisture and surface reactivity based on diurnal clock.
fn update_tile_conditions(time_of_day: Res<GameTimeOfDay>, mut query: Query<&mut DynamicTile>) {
    let is_night = !time_of_day.is_daytime();

    for mut tile in query.iter_mut() {
        if is_night {
            // Condensation accumulates moisture at night
            tile.moisture = (tile.moisture + 0.001).min(1.0);
            tile.temperature = (tile.temperature - 0.01).max(5.0);
        } else {
            // Daytime evaporates excess surface moisture
            tile.moisture = (tile.moisture - 0.001).max(0.1);
            tile.temperature = (tile.temperature + 0.01).min(35.0);
        }
    }
}
