# Design

## Decisions

- **Use Bevy Mesh2d and ColorMaterial for all visual elements.**: Bevy 0.14+ provides these as first-class 2D components. No custom shader is needed. The existing tile_dynamic.wgsl is broken and never loaded.
- **Use six shared ColorMaterial instances, one per TileType.**: Six materials cover all 1024 tiles. One tinting system updates them per frame. No per-tile material allocation.
- **Give units a z-offset of +0.1 above their tile.**: Deterministic draw order. Matches the existing pattern where landscape features use a local z of 0.5 above their tile (map.rs:188).
- **Gate all visual systems in a plugin added only to the windowed client.**: Headless mode and tests use MinimalPlugins with no render resources. A separate plugin keeps those paths unchanged.
- **Delete assets/shaders/tile_dynamic.wgsl.**: It declares its own ViewUniform at @group(0), is never loaded, and cannot be used as written. Per-tile moisture shading can be a later item.

## Rejected alternatives

- **Rewrite tile_dynamic.wgsl as a Material2d shader.**: The file declares its own ViewUniform at @group(0) (lines 4-17) instead of using Bevy's 2D mesh bindings. It is never loaded by any code. ColorMaterial meets the current need without a custom shader.
- **Give each of the 1024 tiles its own ColorMaterial instance.**: 1024 material instances is wasteful. No per-tile variation is needed for the base day/night effect. Six shared materials suffice.
- **Use the 3D mesh pipeline (Mesh3d + StandardMaterial).**: The game is a 2D isometric view. The 3D pipeline adds unnecessary transform and lighting complexity. Bevy's 2D path is simpler and sufficient.
- **Use the RenderOrder component for unit draw order.**: A z-offset of +0.1 is simpler and matches the existing child-entity pattern where landscape features use a local z of 0.5 (map.rs:188).
