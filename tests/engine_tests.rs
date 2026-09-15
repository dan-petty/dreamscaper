//! Unit tests for engine math, coordinates, resources, and terrain genesis.

use bevy::prelude::*;
use dreamscaper::engine::ecs::components::{
    FeatureType, GridPosition, TerrainDevelopmentState, TileType,
};
use dreamscaper::engine::renderer::{
    grid_to_isometric, isometric_to_grid, DEFAULT_TILE_HEIGHT, DEFAULT_TILE_WIDTH,
};
use dreamscaper::engine::resources::{
    evaluate_terrain_sample, GameTimeOfDay, TerrainConfig, TerrainTheme,
};
use dreamscaper::game::map::{select_landscape_feature, tile_hash};

#[test]
fn test_isometric_projection_roundtrip() {
    let original_x = 10.0;
    let original_y = 15.0;
    let elevation = 0.0;

    let screen_pos = grid_to_isometric(
        original_x,
        original_y,
        elevation,
        DEFAULT_TILE_WIDTH,
        DEFAULT_TILE_HEIGHT,
    );

    let recovered_grid = isometric_to_grid(
        screen_pos.x,
        screen_pos.y,
        DEFAULT_TILE_WIDTH,
        DEFAULT_TILE_HEIGHT,
    );

    assert!(
        (original_x - recovered_grid.x).abs() < 1e-4,
        "X coordinate should roundtrip"
    );
    assert!(
        (original_y - recovered_grid.y).abs() < 1e-4,
        "Y coordinate should roundtrip"
    );
}

#[test]
fn test_isometric_origin_maps_to_zero() {
    let screen_origin = grid_to_isometric(0.0, 0.0, 0.0, DEFAULT_TILE_WIDTH, DEFAULT_TILE_HEIGHT);
    assert_eq!(screen_origin, Vec2::ZERO);
}

#[test]
fn test_diurnal_cycle_day_night_transition() {
    let mut clock = GameTimeOfDay {
        current_time: 0.0,
        day_length_seconds: 100.0,
        normalized_time: 0.0,
    };

    // 0.0 = dawn/night
    assert!(!clock.is_daytime());

    // 0.35 = noon / daytime
    clock.normalized_time = 0.35;
    assert!(clock.is_daytime());

    // 0.85 = midnight / night
    clock.normalized_time = 0.85;
    assert!(!clock.is_daytime());
}

#[test]
fn test_grid_position_creation() {
    let pos = GridPosition::new(12, 34, 2);
    assert_eq!(pos.x, 12);
    assert_eq!(pos.y, 34);
    assert_eq!(pos.elevation, 2);
}

#[test]
fn test_terrain_config_defaults_and_evaluation() {
    let config = TerrainConfig::default();
    assert_eq!(config.width, 32);
    assert_eq!(config.height, 32);
    assert!(config.wave_expansion_speed > 0.0);

    let (elevation, tile_type, raw_val) = evaluate_terrain_sample(16.0, 16.0, &config);
    assert!(elevation >= 0);
    assert!((0.0..=1.0).contains(&raw_val));
    assert!(matches!(
        tile_type,
        TileType::Water | TileType::Sand | TileType::Meadow | TileType::Forest | TileType::Stone
    ));
}

#[test]
fn test_terrain_development_state_lifecycle() {
    let latent = TerrainDevelopmentState::Latent;
    assert!(!latent.is_detailed());
    assert!(!latent.is_forming());
    assert_eq!(latent.progress(), 0.0);

    let forming = TerrainDevelopmentState::Forming(0.5);
    assert!(!forming.is_detailed());
    assert!(forming.is_forming());
    assert_eq!(forming.progress(), 0.5);

    let detailed = TerrainDevelopmentState::Detailed;
    assert!(detailed.is_detailed());
    assert!(!detailed.is_forming());
    assert_eq!(detailed.progress(), 1.0);
}

#[test]
fn test_tile_hash_distribution_and_landscape_features() {
    let hash1 = tile_hash(5, 5);
    let hash2 = tile_hash(5, 6);
    assert!((0.0..=1.0).contains(&hash1));
    assert!((0.0..=1.0).contains(&hash2));
    assert_ne!(hash1, hash2);

    let feature_tree = select_landscape_feature(TerrainTheme::Verdant, TileType::Forest, 0.2);
    assert_eq!(feature_tree, Some(FeatureType::Tree));

    let feature_crystal = select_landscape_feature(TerrainTheme::Celestial, TileType::Stone, 0.4);
    assert_eq!(feature_crystal, Some(FeatureType::CrystalCluster));
}
