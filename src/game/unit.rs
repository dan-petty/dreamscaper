//! RTS unit and explorer spawning, selection, and movement order systems.

use crate::engine::ecs::components::{
    Explorer, GridPosition, IsometricCoordinates, Selectable, TargetDestination, Unit, Velocity,
};
use crate::engine::resources::{evaluate_terrain_sample, TerrainConfig};
use bevy::prelude::*;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_units)
            .add_systems(Update, (unit_order_system, explorer_patrol_system));
    }
}

/// Spawns the centered Explorer character and accompanying scout unit.
fn spawn_initial_units(mut commands: Commands, config: Res<TerrainConfig>) {
    let center_x = (config.width / 2) as i32;
    let center_y = (config.height / 2) as i32;
    let (center_elevation, _, _) =
        evaluate_terrain_sample(center_x as f32, center_y as f32, &config);

    // Primary centered Explorer character
    commands.spawn((
        Explorer {
            sight_radius: 8.0,
            exploration_speed: 4.5,
            active: true,
        },
        Unit {
            name: "Explorer".to_string(),
            speed: 4.5,
            team: 1,
            max_hp: 150.0,
            current_hp: 150.0,
        },
        Selectable { selected: true },
        GridPosition::new(center_x, center_y, center_elevation),
        IsometricCoordinates::default(),
        Velocity::default(),
        TargetDestination { target: None },
        Transform::from_xyz(0.0, 0.0, 1.0),
        GlobalTransform::default(),
        Name::new("Explorer"),
    ));

    // Accompanying Scout companion offset near explorer
    commands.spawn((
        Unit {
            name: "Surveyor Scout".to_string(),
            speed: 6.0,
            team: 1,
            max_hp: 80.0,
            current_hp: 80.0,
        },
        Selectable { selected: false },
        GridPosition::new(center_x + 1, center_y, center_elevation),
        IsometricCoordinates::default(),
        Velocity::default(),
        TargetDestination {
            target: Some(Vec2::new((center_x + 3) as f32, (center_y + 2) as f32)),
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        GlobalTransform::default(),
        Name::new("Surveyor_Scout_1"),
    ));
}

/// Process player orders to assigned target destinations.
fn unit_order_system(query: Query<(&Selectable, &TargetDestination), With<Unit>>) {
    for (selectable, dest) in query.iter() {
        if selectable.selected && dest.target.is_none() {
            // Awaiting player click or programmatic order
        }
    }
}

/// Gently directs idle explorers to discover surrounding perimeter when idle.
fn explorer_patrol_system(
    time: Res<Time>,
    config: Res<TerrainConfig>,
    mut query: Query<(&mut TargetDestination, &GridPosition), With<Explorer>>,
) {
    let elapsed = time.elapsed_secs();
    for (mut dest, _pos) in query.iter_mut() {
        if dest.target.is_none() {
            // Subtle slow circular exploration sweep around center
            let radius = 3.0 + (elapsed * 0.2).min(8.0);
            let angle = elapsed * 0.15;
            let center_x = config.width as f32 * 0.5;
            let center_y = config.height as f32 * 0.5;
            let target_x = center_x + angle.cos() * radius;
            let target_y = center_y + angle.sin() * radius;
            dest.target = Some(Vec2::new(target_x, target_y));
        }
    }
}
