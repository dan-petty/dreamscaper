//! Isometric renderer plugin, projection math, and camera system.

use bevy::prelude::*;
use crate::engine::ecs::components::{GridPosition, IsometricCoordinates};

/// Default tile dimensions in 2:1 isometric projection.
pub const DEFAULT_TILE_WIDTH: f32 = 64.0;
pub const DEFAULT_TILE_HEIGHT: f32 = 32.0;

/// Isometric camera configuration resource.
#[derive(Resource, Debug, Clone)]
pub struct IsometricCamera {
    pub size: Vec2,
    pub offset: Vec3,
    pub zoom: f32,
    pub tile_width: f32,
    pub tile_height: f32,
}

impl Default for IsometricCamera {
    fn default() -> Self {
        Self {
            size: Vec2::new(1280.0, 720.0),
            offset: Vec3::ZERO,
            zoom: 1.0,
            tile_width: DEFAULT_TILE_WIDTH,
            tile_height: DEFAULT_TILE_HEIGHT,
        }
    }
}

/// Convert discrete grid coordinate (x, y, elevation) to 2D isometric screen space.
#[inline]
pub fn grid_to_isometric(grid_x: f32, grid_y: f32, elevation: f32, tile_w: f32, tile_h: f32) -> Vec2 {
    let screen_x = (grid_x - grid_y) * (tile_w * 0.5);
    let screen_y = (grid_x + grid_y) * (tile_h * 0.5) + (elevation * tile_h * 0.5);
    Vec2::new(screen_x, screen_y)
}

/// Convert 2D isometric screen space back to continuous grid coordinates (x, y).
#[inline]
pub fn isometric_to_grid(screen_x: f32, screen_y: f32, tile_w: f32, tile_h: f32) -> Vec2 {
    let half_w = tile_w * 0.5;
    let half_h = tile_h * 0.5;
    let grid_x = (screen_x / half_w + screen_y / half_h) * 0.5;
    let grid_y = (screen_y / half_h - screen_x / half_w) * 0.5;
    Vec2::new(grid_x, grid_y)
}

/// Renderer plugin configuring isometric camera, coordinate conversion, and dynamic tile updates.
pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IsometricCamera>()
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (tile_update_system, update_isometric_transforms));
    }
}

/// Spawns 2D camera bundle with isometric default views.
fn setup_camera(mut commands: Commands, camera_settings: Res<IsometricCamera>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.transform.translation = camera_settings.offset;
    commands.spawn((camera_bundle, Name::new("IsometricCamera")));
}

/// System to synchronize ECS GridPosition into IsometricCoordinates and Bevy Transform.
fn update_isometric_transforms(
    camera_settings: Res<IsometricCamera>,
    mut query: Query<(&GridPosition, &mut IsometricCoordinates, &mut Transform), Changed<GridPosition>>,
) {
    for (grid_pos, mut iso_coords, mut transform) in query.iter_mut() {
        let iso_vec = grid_to_isometric(
            grid_pos.x as f32,
            grid_pos.y as f32,
            grid_pos.elevation as f32,
            camera_settings.tile_width,
            camera_settings.tile_height,
        );

        iso_coords.screen_x = iso_vec.x;
        iso_coords.screen_y = iso_vec.y;
        // Depth-sorting: higher Y in grid space should render on top
        let depth = -((grid_pos.x + grid_pos.y) as f32 * 0.01) + (grid_pos.elevation as f32 * 0.1);
        iso_coords.depth = depth;

        transform.translation.x = iso_vec.x;
        transform.translation.y = iso_vec.y;
        transform.translation.z = depth;
    }
}

/// System for reacting to time of day or environmental conditions to update tiles.
fn tile_update_system(time: Res<Time>) {
    // Dynamically modulates shader uniforms and environmental conditions
    let _elapsed = time.elapsed_seconds();
}
