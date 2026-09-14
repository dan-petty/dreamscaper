# Dreamscaper Engine API Reference

## Reusable Plugins

### `EnginePlugin`
Root plugin registering all core engine subsystems.

```rust
use bevy::prelude::*;
use dreamscaper::engine::EnginePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EnginePlugin)
        .run();
}
```

### Subsystems

#### `RendererPlugin`
- **Resources**:
  - `IsometricCamera`: Controls isometric projection viewport, zoom levels, and pan offsets.
- **Components**:
  - `IsometricCoordinates`: Stores screen-space isometric coordinates $(x_{\text{iso}}, y_{\text{iso}}, \text{depth})$.
- **Mathematical Functions**:
  - `grid_to_isometric(grid_pos: Vec3, tile_w: f32, tile_h: f32) -> Vec2`
  - `isometric_to_grid(screen_pos: Vec2, tile_w: f32, tile_h: f32) -> Vec2`

#### `AiPlugin`
- **Resources**:
  - `GpuSimConfig`: Configuration for GPU compute simulation (speed, avoidance radius, agent count).
  - `AgentComputeBuffer`: Manages GPU compute storage buffers.
- **Systems**:
  - `dispatch_ai_compute`: Dispatches GPU compute shader invocations.
  - `sync_gpu_agent_transforms`: Propagates GPU agent state into Bevy `Transform` and `GridPosition` components.

#### `ResourcePlugin`
- **Resources**:
  - `GameTimeOfDay`: Manages diurnal clock, day length, and normalized solar cycle time ($0.0 \le t \le 1.0$).
  - `EnvironmentSettings`: Global ambient light, season, and atmospheric conditions.
- **Assets**:
  - `TileMapAsset`: Serialized map file representing grid tiles, biomes, and elevations.
