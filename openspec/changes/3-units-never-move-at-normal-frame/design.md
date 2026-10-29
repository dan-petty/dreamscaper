# Design

## Decisions

- **Add a new ContinuousPosition component rather than reusing Velocity.vec or Transform.**: Velocity represents direction and speed, not position. Transform is a rendering concern. A dedicated component keeps simulation state separate from rendering.
- **Floor the continuous position to i32 for GridPosition, not round.**: Floor gives the correct grid cell membership. Round introduces a half-tile offset that does not match the cell the unit occupies.
- **Use Unit.speed for per-unit movement instead of GpuSimConfig.max_speed.**: The issue states Unit.speed (Explorer 4.5, Scout 6.0) is never read. Using per-unit speed respects the design intent.
- **Remove sync_agent_destinations entirely.**: The function body is empty (src/engine/ai/ai_plugin.rs lines 80-88). It serves no purpose and adds noise to the plugin registration.
- **Write GridPosition only when the derived cell changes.**: Avoids unnecessary ECS change-detection churn. GridPosition becomes a derived cache of ContinuousPosition.

## Rejected alternatives

- **Reuse Velocity.vec as the continuous position.**: Velocity represents direction and speed (src/engine/ecs/components.rs:91). Mixing position into it breaks single responsibility.
- **Use Bevy Transform as the source of truth for position.**: Transform is a rendering concern (src/engine/renderer/renderer_plugin.rs:119-123). Using it as simulation state couples game logic to the renderer.
- **Keep GridPosition as source of truth with a fractional offset.**: The offset still requires rounding each step (src/engine/ai/ai_plugin.rs:69-71). It preserves the original bug pattern.
- **Keep sync_agent_destinations as a placeholder for future use.**: The body is empty (src/engine/ai/ai_plugin.rs:80-88). No documented future use exists.
- **Use GpuSimConfig.max_speed for all units.**: This ignores per-unit speed values (src/game/unit.rs:34, :54). All units would move at the same speed, contradicting the design intent.
