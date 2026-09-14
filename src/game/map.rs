//! Map generation, isometric tile grid spawning, and environmental reactivity.

use crate::engine::ecs::components::{DynamicTile, GridPosition, IsometricCoordinates, TileType};
use crate::engine::resources::GameTimeOfDay;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_map)
            .add_systems(Update, update_tile_conditions);
    }
}

/// Generates an initial isometric grid landscape.
fn generate_map(mut commands: Commands) {
    let width = 32;
    let height = 32;

    for x in 0..width {
        for y in 0..height {
            // Determine elevation and tile type procedurally
            let elevation = if (x > 10 && x < 22) && (y > 10 && y < 22) {
                1
            } else {
                0
            };

            let tile_type =
                if (x == 0 || y == 0 || x == width - 1 || y == height - 1) && elevation == 0 {
                    TileType::Water
                } else if elevation == 1 {
                    TileType::Stone
                } else {
                    TileType::Meadow
                };

            commands.spawn((
                GridPosition::new(x, y, elevation),
                IsometricCoordinates::default(),
                DynamicTile {
                    tile_type,
                    moisture: 0.5,
                    temperature: 20.0,
                    elevation: elevation as f32,
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
                GlobalTransform::default(),
                Name::new(format!("Tile_{}_{}", x, y)),
            ));
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
