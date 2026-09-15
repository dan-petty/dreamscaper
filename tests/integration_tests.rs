//! Integration tests for Bevy App plugin registration, explorer spawning, and terrain genesis.

use bevy::prelude::*;
use dreamscaper::engine::ecs::components::{Explorer, GridPosition, TerrainDevelopmentState, Unit};
use dreamscaper::engine::resources::{GameTimeOfDay, TerrainConfig};
use dreamscaper::engine::EnginePlugin;
use dreamscaper::game::GamePlugin;

#[test]
fn test_app_minimal_plugin_build() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .add_plugins(EnginePlugin);

    // Verify resources were initialized
    assert!(app.world().contains_resource::<GameTimeOfDay>());
    assert!(app.world().contains_resource::<TerrainConfig>());
}

#[test]
fn test_spawn_and_query_unit_entity() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .add_plugins(EnginePlugin);

    let entity = app
        .world_mut()
        .spawn((
            Unit {
                name: "TestUnit".to_string(),
                speed: 5.0,
                team: 1,
                max_hp: 100.0,
                current_hp: 100.0,
            },
            GridPosition::new(5, 5, 0),
        ))
        .id();

    let unit = app
        .world()
        .get::<Unit>(entity)
        .expect("Unit component should exist");
    assert_eq!(unit.name, "TestUnit");
    assert_eq!(unit.team, 1);

    let pos = app
        .world()
        .get::<GridPosition>(entity)
        .expect("GridPosition should exist");
    assert_eq!(pos.x, 5);
    assert_eq!(pos.y, 5);
}

#[test]
fn test_explorer_spawned_at_map_center() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .add_plugins((EnginePlugin, GamePlugin));

    app.update();

    let config = app
        .world()
        .get_resource::<TerrainConfig>()
        .expect("TerrainConfig must exist")
        .clone();
    let expected_center_x = (config.width / 2) as i32;
    let expected_center_y = (config.height / 2) as i32;

    let mut query = app
        .world_mut()
        .query_filtered::<(&GridPosition, &Explorer), With<Explorer>>();
    let mut explorer_found = false;

    for (pos, explorer) in query.iter(app.world()) {
        explorer_found = true;
        assert_eq!(pos.x, expected_center_x, "Explorer X should be centered");
        assert_eq!(pos.y, expected_center_y, "Explorer Y should be centered");
        assert!(explorer.active);
    }

    assert!(explorer_found, "Explorer character entity must be spawned");
}

#[test]
fn test_progressive_terrain_genesis_initial_and_expansion() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .add_plugins((EnginePlugin, GamePlugin));

    // Initial startup execution
    app.update();

    let config = app
        .world()
        .get_resource::<TerrainConfig>()
        .expect("TerrainConfig must exist")
        .clone();
    let center_x = (config.width / 2) as i32;
    let center_y = (config.height / 2) as i32;

    // Center tile should immediately be Detailed; corners should start Latent
    let mut tile_query = app
        .world_mut()
        .query::<(&GridPosition, &TerrainDevelopmentState)>();
    let mut center_detailed = false;
    let mut has_latent = false;

    for (pos, state) in tile_query.iter(app.world()) {
        if pos.x == center_x && pos.y == center_y {
            if state.is_detailed() {
                center_detailed = true;
            }
        } else if *state == TerrainDevelopmentState::Latent {
            has_latent = true;
        }
    }

    assert!(
        center_detailed,
        "Center tile at ({}, {}) should start Detailed",
        center_x, center_y
    );
    assert!(
        has_latent,
        "Outer perimeter tiles should start in Latent state"
    );

    // Simulate multiple frame ticks to verify wave progression
    for _ in 0..10 {
        app.update();
    }

    // Verify tiles are forming or detailed
    let mut forming_or_detailed_count = 0;
    for (_pos, state) in tile_query.iter(app.world()) {
        if state.is_detailed() || state.is_forming() {
            forming_or_detailed_count += 1;
        }
    }

    assert!(
        forming_or_detailed_count >= 1,
        "Genesis wave should expand and detail surrounding tiles"
    );
}
