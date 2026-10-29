# Proposal

## Why

Issue #3 reports that units are stationary at 60 fps because each 0.1-tile step rounds back to the same grid cell, the patrol stops at the first waypoint, and per-unit speed is ignored.

## What Changes

- Add a ContinuousPosition (Vec2) component as the source of truth for unit movement.
- Rewrite dispatch_ai_compute to integrate ContinuousPosition with Unit.speed * delta.
- Derive GridPosition from ContinuousPosition, writing only on cell change.
- Clear TargetDestination.target on arrival to unblock the patrol loop.
- Remove the empty sync_agent_destinations system.
- Update the renderer to read ContinuousPosition for Transform computation.
- Add tests for Scout arrival, Explorer patrol, and speed proportionality.
- The change fits one pull request.

## Capabilities

### New Capabilities

- `unit-movement`: continuous-position-integration, grid-position-derivation, target-arrival-clears-destination, renderer-reads-continuous-position

### Modified Capabilities

None.

## Impact

None.

## Feasibility

- The rounding bug is confirmed by src/engine/ai/ai_plugin.rs lines 58-72, which show the round-to-i32 pattern on a 0.1-tile step.
- The 1/60 s pacing is confirmed by src/main.rs lines 80-82, which set ScheduleRunnerPlugin::run_loop to Duration::from_secs_f64(1.0 / 60.0).
- The patrol stall is confirmed by src/engine/ai/ai_plugin.rs lines 72-74 (target not cleared) and src/game/unit.rs lines 81-98 (patrol assigns only when target is None).
- The empty sync_agent_destinations is confirmed at src/engine/ai/ai_plugin.rs lines 80-88.
- First checks: run `cargo build` to confirm the project compiles. Run `cargo test` to confirm the existing suite passes. Run `grep -n 'sync_agent_destinations' src/engine/ai/ai_plugin.rs` to confirm the empty system exists.
- The change is feasible within one pull request. No external dependencies are added.

## Existing Tool Considered

Bevy ECS (already in use) provides the component and system framework. No new external tool or library is required. The change modifies existing project code in src/engine/ai/ai_plugin.rs, src/engine/ecs/components.rs, and src/engine/renderer/renderer_plugin.rs.
