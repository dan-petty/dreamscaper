# Tasks

## 1. Create visual plugin with tile diamond mesh and six materials

Reads: `src/game/map.rs`, `src/engine/renderer/renderer_plugin.rs`, `src/engine/renderer/mod.rs`. Changes: `src/engine/renderer/visual_plugin.rs` (new), `src/engine/renderer/mod.rs`.

- [ ] 1.1 Write tests/test_visual_tiles.rs. Build an App with MinimalPlugins, AssetPlugin, TransformPlugin, EnginePlugin, GamePlugin, and VisualPlugin. Call app.update(). Query all entities with GridPosition. Assert each has a Mesh2d component. Assert the world contains exactly six ColorMaterial resources. Assert each tile entity has a Material2d component referencing one of the six materials, and verify that `cargo test --test test_visual_tiles` fails on the base
- [ ] 1.2 Create src/engine/renderer/visual_plugin.rs. Define VisualPlugin. In its setup, build one shared diamond Mesh2d sized tile_width by tile_height. Create six ColorMaterial instances, one per TileType variant, with distinct base colours. In a spawn system, query all entities with GridPosition and DynamicTile. For each, add Mesh2d (the shared diamond) and Material2d (the matching per-type material). Register the plugin in src/engine/renderer/mod.rs, and verify that `cargo test --test test_visual_tiles` passes
- [ ] 1.3 Change no document, run `uv run devops docs generate --sync-readme`, and verify that `uv run devops ci docs` passes

## 2. Add unit and feature visuals with z-offset

Reads: `src/game/unit.rs`, `src/game/map.rs`, `src/engine/renderer/visual_plugin.rs`. Changes: `src/engine/renderer/visual_plugin.rs`.

- [ ] 2.1 Write tests/test_visual_units.rs. Build an App with the same plugins as the tile test. Call app.update(). Query entities with the Unit component. Assert each has a Mesh2d and Material2d. Assert the unit Transform z is greater than the tile Transform z in the same cell. Query landscape feature entities (children of tiles with local z 0.5). Assert they have Mesh2d and Material2d, and verify that `cargo test --test test_visual_units` fails on the base
- [ ] 2.2 In the visual plugin spawn system, add a branch for entities with the Unit component. Give each a small square or circle Mesh2d and a distinct ColorMaterial. Set the unit Transform z to the tile z plus 0.1. For landscape feature entities (already children of tiles with local z 0.5), add a small triangle or box Mesh2d and a feature-specific ColorMaterial, and verify that `cargo test --test test_visual_units` passes
- [ ] 2.3 Change no document, run `uv run devops docs generate --sync-readme`, and verify that `uv run devops ci docs` passes

## 3. Implement day/night tinting and remove empty tile_update_system

Reads: `src/engine/resources/resource_plugin.rs`, `src/engine/renderer/renderer_plugin.rs`, `src/engine/renderer/visual_plugin.rs`. Changes: `src/engine/renderer/visual_plugin.rs`, `src/engine/renderer/renderer_plugin.rs`.

- [ ] 3.1 Write tests/test_day_night_tint.rs. Build an App with the visual plugin. Set EnvironmentSettings.ambient_light to Vec3::splat(0.2). Call app.update(). Read the six ColorMaterial colours. Assert each is the base colour multiplied by 0.2. Set ambient_light to Vec3::splat(1.0). Call app.update(). Assert each material colour equals the base colour. Assert tile_update_system no longer exists in renderer_plugin.rs (grep check), and verify that `cargo test --test test_day_night_tint` fails on the base
- [ ] 3.2 In the visual plugin, add a tinting system that runs each frame. It reads EnvironmentSettings.ambient_light. For each of the six ColorMaterial instances, set the base colour to the stored base colour multiplied by the ambient_light scalar. Remove the empty tile_update_system function from src/engine/renderer/renderer_plugin.rs. Remove its registration from the plugin's add_systems call, and verify that `cargo test --test test_day_night_tint` passes
- [ ] 3.3 Change no document, run `uv run devops docs generate --sync-readme`, and verify that `uv run devops ci docs` passes

## 4. Wire plugin in main.rs and delete tile_dynamic.wgsl

Reads: `src/main.rs`, `tests/integration_tests.rs`. Changes: `src/main.rs`, `assets/shaders/tile_dynamic.wgsl`.

- [ ] 4.1 Run the existing test suite. All four tests in tests/integration_tests.rs must pass. Run `cargo run -- --headless --ticks 120` and confirm exit code 0. Run `test ! -f assets/shaders/tile_dynamic.wgsl` and confirm exit code 0. Run `grep -n 'tile_update_system' src/engine/renderer/renderer_plugin.rs` and confirm no match, and verify that `cargo test && cargo run -- --headless --ticks 120 && test ! -f assets/shaders/tile_dynamic.wgsl` fails on the base
- [ ] 4.2 In src/main.rs, add VisualPlugin to the windowed client plugin list (the branch that adds WindowPlugin and RenderPlugin). Do not add it to the headless branch. Delete the file assets/shaders/tile_dynamic.wgsl, and verify that `cargo test && cargo run -- --headless --ticks 120 && test ! -f assets/shaders/tile_dynamic.wgsl` passes
- [ ] 4.3 Change no document, run `uv run devops docs generate --sync-readme`, and verify that `uv run devops ci docs` passes

## Workflow follow-up

- Pending a person: review the diff in tuicr (tuicr -r <base>..HEAD).
