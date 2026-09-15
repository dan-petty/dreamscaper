//! Integration tests for Bevy App plugin registration and system execution.

use bevy::prelude::*;
use dreamscaper::engine::ecs::components::{GridPosition, Unit};
use dreamscaper::engine::resources::GameTimeOfDay;
use dreamscaper::engine::EnginePlugin;

#[test]
fn test_app_minimal_plugin_build() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .add_plugins(EnginePlugin);

    // Verify resources were initialized
    assert!(app.world().contains_resource::<GameTimeOfDay>());
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
