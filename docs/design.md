# Dreamscaper Game Design & Mechanics

## Core Philosophy
`dreamscaper` merges classic isometric RTS depth (such as *Age of Empires* and *Command & Conquer*) with emergent, GPU-accelerated simulation. The world is alive: ground conditions shift dynamically with environmental pressures and the diurnal cycle, while hundreds of autonomous units coordinate through compute-shader-driven vector fields.

## 1. Dynamic Reactive Tiles
Tiles in `dreamscaper` are not static textures; they are simulated entities with physical and environmental states:
- **Diurnal Shifting**:
  - During the day, tiles absorb heat and daylight.
  - At dusk and night, moisture accumulates (dew/frost), cooling down temperatures and altering movement friction.
- **Environmental State Variables**:
  - `elevation`: Determines vantage advantage, projectile arc, and climbing speed.
  - `moisture`: Affects fire spread, unit traction, and vegetation growth.
  - `temperature`: Determines ice freezing/melting and heat stress on combat units.

## 2. Semi-Autonomous GPU AI
Rather than running expensive pathfinding on the CPU for every individual unit every tick:
1. **Flow Fields & Vector Grids**: Destination targets generate potential and vector flow fields uploaded to GPU compute storage buffers.
2. **GPU Steering Workgroups**: Compute shaders simulate flocking, local obstacle avoidance, separation, and terrain friction in parallel across thousands of units at sub-millisecond speeds.
3. **High-Level CPU Strategy**: CPU systems handle strategic decision making (e.g. build queues, attack orders, tactical retreats) and dispatch coarse targets to the GPU agents.

## 3. RTS Controls & Ergonomics
- **Camera Controls**:
  - WASD or Middle-Mouse drag for pan.
  - Mouse wheel for smooth isometric zoom.
  - Q/E for rotating isometric perspective in 90-degree increments.
- **Unit Selection**:
  - Left-click to select individual unit.
  - Left-drag box for area selection with multi-unit group formation.
  - Right-click for contextual orders (move, attack, harvest).
