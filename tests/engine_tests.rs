//! Unit tests for engine math, coordinates, and resources.

use bevy::prelude::*;
use dreamscaper::engine::ecs::components::GridPosition;
use dreamscaper::engine::renderer::{
    grid_to_isometric, isometric_to_grid, DEFAULT_TILE_HEIGHT, DEFAULT_TILE_WIDTH,
};
use dreamscaper::engine::resources::GameTimeOfDay;

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
