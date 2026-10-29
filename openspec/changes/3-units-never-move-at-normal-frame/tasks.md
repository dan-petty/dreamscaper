# Tasks

## 1. Add ContinuousPosition and rewrite AI movement

Reads: None.. Changes: None..

- [ ] 1.1 Write tests/test_movement.rs with a test that creates a Scout entity at ContinuousPosition (0,0) with target (3,2), runs 120 updates at 1/60 s using TimeUpdateStrategy::ManualDuration, and asserts GridPosition equals (3,2)., and verify that `cargo test test_scout_reaches_target` fails on the base
- [ ] 1.2 Add a ContinuousPosition { pos: Vec2 } component to src/engine/ecs/components.rs. Export it from src/engine/ecs/mod.rs. Rewrite dispatch_ai_compute in src/engine/ai/ai_plugin.rs to query (&mut ContinuousPosition, &mut GridPosition, &mut Velocity, &TargetDestination, &Unit). Integrate pos += dir * unit.speed * delta. Floor each axis to i32 to derive GridPosition, writing only when the cell changes. Set TargetDestination.target to None on arrival. Remove the sync_agent_destinations function and its registration from the plugin., and verify that `cargo test test_scout_reaches_target` passes
- [ ] 1.3 Update docs/api.md to list the ContinuousPosition component and remove the sync_agent_destinations entry., run `uv run devops docs generate --sync-readme`, and verify that `uv run devops ci docs` passes

## 2. Update renderer to read ContinuousPosition

Reads: None.. Changes: None..

- [ ] 2.1 Add a test to tests/test_movement.rs that creates a unit with ContinuousPosition (1.5, 2.0) and GridPosition (1,2), runs one update, and asserts the Transform translation matches the isometric projection of (1.5, 2.0), not (1,2)., and verify that `cargo test test_transform_uses_continuous_position` fails on the base
- [ ] 2.2 Change the query in update_isometric_transforms in src/engine/renderer/renderer_plugin.rs from (&GridPosition, &mut IsometricCoordinates, &mut Transform) to (&ContinuousPosition, &mut IsometricCoordinates, &mut Transform). Use ContinuousPosition.pos for the isometric coordinate conversion., and verify that `cargo test test_transform_uses_continuous_position` passes
- [ ] 2.3 Update docs/api.md to note the renderer reads ContinuousPosition for Transform computation., run `uv run devops docs generate --sync-readme`, and verify that `uv run devops ci docs` passes

## 3. Add patrol and speed-proportionality tests

Reads: None.. Changes: None..

- [ ] 3.1 Add two tests to tests/test_movement.rs: (1) an Explorer patrol test that runs 300 updates at 1/60 s and asserts the Explorer visits at least two distinct waypoints; (2) a speed-proportionality test that runs two units (speed 3.0 and 6.0) over the same 3-tile distance and asserts the ratio of updates is between 1.5 and 2.5., and verify that `cargo test test_explorer_patrol test_speed_proportionality` fails on the base
- [ ] 3.2 The tests should pass with the implementation from the first group. If the patrol test fails, verify that explorer_patrol_system in src/game/unit.rs correctly assigns a new waypoint when target is None. No additional code changes are expected., and verify that `cargo test test_explorer_patrol test_speed_proportionality` passes
- [ ] 3.3 No documentation changes needed., run `uv run devops docs generate --sync-readme`, and verify that `uv run devops ci docs` passes

## Workflow follow-up

- Pending a person: review the diff in tuicr (tuicr -r <base>..HEAD).
