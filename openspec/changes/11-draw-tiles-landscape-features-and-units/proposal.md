# Proposal

## Why

Issue #11 reports that cargo run shows only the clear colour and HUD text. Tiles, units, and landscape features carry Transform but no visual component. The day/night system computes ambient_light each frame but never applies it. The tile shader is broken and never loaded. The README claims day/night shading is done, but it is not.

## What Changes

- Create a visual plugin that spawns a shared diamond Mesh2d and six ColorMaterial instances for tiles.
- Add primitive Mesh2d and ColorMaterial to units and landscape features with a z-offset above their tile.
- Replace the empty tile_update_system with a tinting system that updates six material colours from ambient_light.
- Delete assets/shaders/tile_dynamic.wgsl.
- Add the visual plugin to the windowed client in main.rs only, leaving headless mode and tests unchanged.
- The change fits one pull request.

## Capabilities

### New Capabilities

- `isometric-rendering`: tiles-render-as-coloured-diamonds, units-render-above-their-tile, day-night-tinting-updates-material-colours, headless-mode-runs-without-render-resources

### Modified Capabilities

None.

## Impact

- Code: `src/engine/renderer/visual_plugin.rs`, `src/engine/renderer/mod.rs`, `src/engine/renderer/renderer_plugin.rs`, `src/main.rs`, `assets/shaders/tile_dynamic.wgsl`

## Feasibility

- Confirmed: src/game/map.rs:96-109 spawns tiles with Transform but no Mesh2d, Sprite, or material. src/game/unit.rs:26-68 spawns units the same way.
- Confirmed: src/engine/renderer/renderer_plugin.rs:149-152 shows tile_update_system with an empty body. src/engine/resources/resource_plugin.rs:168 computes ambient_light but no code reads it for rendering.
- Confirmed: assets/shaders/tile_dynamic.wgsl declares its own ViewUniform at @group(0) and is never loaded by any code in src/.
- Confirmed: src/main.rs:79-92 uses MinimalPlugins for headless mode. tests/integration_tests.rs uses MinimalPlugins in all four tests.
- First checks: run `cargo build` to confirm the project compiles at the current commit. Run `cargo test` to confirm all existing tests pass before any change.

## Existing Tool Considered

Bevy 0.14+ provides Mesh2d and ColorMaterial as first-class 2D rendering components. No external library is needed. The project already depends on Bevy (src/main.rs:80 uses bevy::transform::TransformPlugin).
