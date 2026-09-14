# Dreamscaper Architecture

`dreamscaper` is engineered as a modular, high-performance isometric Real-Time Strategy (RTS) engine built on top of [Bevy 0.13+](https://bevyengine.org/) and WebGPU (`wgpu`).

```
                                    +--------------------+
                                    |    src/main.rs     |
                                    +---------+----------+
                                              |
                     +------------------------+-----------------------+
                     |                                                |
          +----------v-----------+                         +----------v----------+
          | dreamscaper::engine  |                         |  dreamscaper::game  |
          +----------+-----------+                         +----------+----------+
                     |                                                |
   +--------+--------+--------+--------+                    +---------+---------+
   |        |                 |        |                    |         |         |
+--v--+  +--v---+          +--v---+ +--v---+             +--v---+  +--v---+  +--v--+
|Renderer| | AI |          | Res  | | ECS  |             | Map  |  | Unit |  | UI  |
+-----+  +------+          +------+ +------+             +------+  +------+  +-----+
```

## Layer Responsibilities

### 1. Engine Layer (`dreamscaper::engine`)
Designed to be decoupled and reusable across different isometric game titles.
- **Renderer (`engine::renderer`)**:
  - Isometric camera positioning with 2:1 projection math.
  - Coordinate transformations between grid space $(x, y, z)$, world coordinates, and screen isometric space.
  - Depth-sorting mechanism for sprites, dynamic tiles, and 3D billboards.
- **GPU Compute AI (`engine::ai`)**:
  - Direct `wgpu` compute shader pipelines via Bevy render graph.
  - Offloads pathfinding, flocking, steering behaviors, and collision avoidance to GPU workgroups.
  - Seamless fallback simulation pipeline for environments without compute shader support (e.g. legacy WebGL).
- **Resources (`engine::resources`)**:
  - Time-of-day diurnal cycle clock (`GameTimeOfDay`).
  - Environmental state manager (moisture, temperature, season).
  - Asset loaders and custom asset definitions (`TileMapAsset`).
- **ECS Infrastructure (`engine::ecs`)**:
  - Base components: `GridPosition`, `IsometricCoordinates`, `Unit`, `DynamicTile`, `Selectable`.

### 2. Game Layer (`dreamscaper::game`)
Implements RTS-specific gameplay mechanics and player interaction.
- **Map System (`game::map`)**:
  - Procedural grid generation with elevation and biomes.
  - Real-time tile reactivity reacting to weather and diurnal conditions.
- **Unit System (`game::unit`)**:
  - Worker, scout, and combat units.
  - Order assignment (move, gather, patrol, attack) linked to GPU AI agent buffers.
- **UI System (`game::ui`)**:
  - Selection box drag-and-drop system.
  - Real-time HUD showing diurnal cycle clock, selected unit statistics, and minimap.

## Multi-Platform Target Strategy
- **Desktop (Linux, macOS, Windows)**: Full Vulkan/Metal/DirectX 12 support with multi-threaded Bevy ECS and hardware GPU compute shaders.
- **WebAssembly (wasm32-unknown-unknown)**: Builds for browsers targeting WebGPU / WebGL2 via `wasm-bindgen`.
- **Mobile (Android, iOS)**: Native compilation targeting Vulkan on Android (`aarch64-linux-android`) and Metal on iOS (`aarch64-apple-ios`).
