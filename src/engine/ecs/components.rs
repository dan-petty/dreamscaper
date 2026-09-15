//! Core ECS components for entities, tiles, units, and spatial layout.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Discrete tile grid coordinate.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
    pub elevation: i32,
}

impl GridPosition {
    pub const fn new(x: i32, y: i32, elevation: i32) -> Self {
        Self { x, y, elevation }
    }
}

/// Continuous projected 2D isometric coordinates and render depth.
#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
pub struct IsometricCoordinates {
    pub screen_x: f32,
    pub screen_y: f32,
    pub depth: f32,
}

/// Enumeration of distinct terrain and tile types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TileType {
    #[default]
    Meadow,
    Water,
    Sand,
    Stone,
    Forest,
    Road,
}

/// Component attached to simulated tiles reacting to environmental conditions.
#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicTile {
    pub tile_type: TileType,
    pub moisture: f32,
    pub temperature: f32,
    pub elevation: f32,
}

impl Default for DynamicTile {
    fn default() -> Self {
        Self {
            tile_type: TileType::Meadow,
            moisture: 0.5,
            temperature: 20.0,
            elevation: 0.0,
        }
    }
}

/// Unit entity attributes for RTS gameplay.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct Unit {
    pub name: String,
    pub speed: f32,
    pub team: u32,
    pub max_hp: f32,
    pub current_hp: f32,
}

impl Default for Unit {
    fn default() -> Self {
        Self {
            name: "Worker".to_string(),
            speed: 5.0,
            team: 1,
            max_hp: 100.0,
            current_hp: 100.0,
        }
    }
}

/// Marker component for entities selectable by the player cursor/drag-box.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Selectable {
    pub selected: bool,
}

/// Continuous velocity vector for moving entities.
#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
pub struct Velocity {
    pub vec: Vec2,
}

/// Movement target order for autonomous steering or pathfinding.
#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
pub struct TargetDestination {
    pub target: Option<Vec2>,
}

/// Specialized Explorer character that radiates terrain discovery.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct Explorer {
    pub sight_radius: f32,
    pub exploration_speed: f32,
    pub active: bool,
}

impl Default for Explorer {
    fn default() -> Self {
        Self {
            sight_radius: 6.0,
            exploration_speed: 4.5,
            active: true,
        }
    }
}

/// Progressive development and crystallization state of an isometric tile.
#[derive(Component, Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum TerrainDevelopmentState {
    /// Latent cell awaiting the outward discovery wave.
    #[default]
    Latent,
    /// Progressive materialization phase (0.0 to 1.0) with rising elevation.
    Forming(f32),
    /// Fully crystallized and detailed landscape cell hosting natural features.
    Detailed,
}

impl TerrainDevelopmentState {
    pub fn is_detailed(&self) -> bool {
        matches!(self, Self::Detailed)
    }

    pub fn is_forming(&self) -> bool {
        matches!(self, Self::Forming(_))
    }

    pub fn progress(&self) -> f32 {
        match self {
            Self::Latent => 0.0,
            Self::Forming(p) => *p,
            Self::Detailed => 1.0,
        }
    }
}

/// Categories of natural and architectural landscape features on detailed terrain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeatureType {
    Tree,
    AncientStone,
    CrystalCluster,
    WildFlora,
    SpringWater,
}

/// Landscape feature entity spawned on crystallized tiles.
#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LandscapeFeature {
    pub feature_type: FeatureType,
    pub scale: f32,
    pub variant: u32,
}
