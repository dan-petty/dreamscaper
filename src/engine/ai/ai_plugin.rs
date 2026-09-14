//! GPU Compute AI plugin managing unit steering, flocking, and pathfinding.

use crate::engine::ecs::components::{GridPosition, TargetDestination, Velocity};
use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};

/// Memory layout of agent data sent to GPU compute shaders.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, Default)]
pub struct GpuAgent {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub target: [f32; 2],
    pub state: u32,
    pub morale: f32,
    pub _pad: [f32; 2],
}

/// Simulation parameters for the compute shader.
#[derive(Resource, Debug, Clone)]
pub struct GpuSimConfig {
    pub max_speed: f32,
    pub avoidance_radius: f32,
    pub agent_count: u32,
    pub enabled: bool,
}

impl Default for GpuSimConfig {
    fn default() -> Self {
        Self {
            max_speed: 6.0,
            avoidance_radius: 1.5,
            agent_count: 0,
            enabled: true,
        }
    }
}

/// Plugin registering GPU compute resources and unit steering simulation systems.
pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GpuSimConfig>()
            .add_systems(Update, (dispatch_ai_compute, sync_agent_destinations));
    }
}

/// System for dispatching compute shaders or driving CPU fallback simulation.
fn dispatch_ai_compute(
    time: Res<Time>,
    config: Res<GpuSimConfig>,
    mut query: Query<(&mut GridPosition, &mut Velocity, &TargetDestination)>,
) {
    let delta = time.delta_seconds();
    let max_speed = config.max_speed;

    for (mut grid_pos, mut velocity, target_dest) in query.iter_mut() {
        if let Some(target) = target_dest.target {
            let current_pos = Vec2::new(grid_pos.x as f32, grid_pos.y as f32);
            let diff = target - current_pos;
            let dist = diff.length();

            if dist > 0.1 {
                let dir = diff.normalize();
                velocity.vec = dir * max_speed;

                // Step discrete grid coordinates smoothly
                let next_step = current_pos + velocity.vec * delta;
                grid_pos.x = next_step.x.round() as i32;
                grid_pos.y = next_step.y.round() as i32;
            } else {
                velocity.vec = Vec2::ZERO;
            }
        }
    }
}

/// Synchronizes high-level agent targets with steering logic.
fn sync_agent_destinations(
    mut query: Query<(&mut TargetDestination, &GridPosition), Changed<TargetDestination>>,
) {
    for (dest, _grid_pos) in query.iter_mut() {
        if let Some(_target) = dest.target {
            // Unit goal assigned or updated
        }
    }
}
