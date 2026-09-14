//! RTS unit spawning, selection, movement orders, and health systems.

use crate::engine::ecs::components::{
    GridPosition, IsometricCoordinates, Selectable, TargetDestination, Unit, Velocity,
};
use bevy::prelude::*;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_units)
            .add_systems(Update, unit_order_system);
    }
}

/// Spawns sample player and AI units onto the map.
fn spawn_initial_units(mut commands: Commands) {
    // Player worker unit
    commands.spawn((
        Unit {
            name: "Worker Unit 1".to_string(),
            speed: 4.0,
            team: 1,
            max_hp: 100.0,
            current_hp: 100.0,
        },
        Selectable { selected: false },
        GridPosition::new(4, 4, 0),
        IsometricCoordinates::default(),
        Velocity::default(),
        TargetDestination {
            target: Some(Vec2::new(8.0, 8.0)),
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        GlobalTransform::default(),
        Name::new("Player_Worker_1"),
    ));

    // Scout unit
    commands.spawn((
        Unit {
            name: "Scout Unit 1".to_string(),
            speed: 6.0,
            team: 1,
            max_hp: 60.0,
            current_hp: 60.0,
        },
        Selectable { selected: false },
        GridPosition::new(5, 4, 0),
        IsometricCoordinates::default(),
        Velocity::default(),
        TargetDestination {
            target: Some(Vec2::new(12.0, 12.0)),
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Name::new("Player_Scout_1"),
    ));
}

/// Process player orders (e.g. contextual right-click commands) to assigned target destinations.
fn unit_order_system(query: Query<(&Selectable, &TargetDestination), With<Unit>>) {
    for (selectable, dest) in query.iter() {
        if selectable.selected && dest.target.is_none() {
            // Can be populated from player input / cursor raycast
        }
    }
}
