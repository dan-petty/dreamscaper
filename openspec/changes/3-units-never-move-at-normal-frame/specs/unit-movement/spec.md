## Purpose
Defines how units move through the game world using continuous position integration, per-unit speed, grid cell derivation, and target arrival handling.

## ADDED Requirements

### Requirement: continuous-position-integration
The movement system SHALL integrate a unit's continuous position by adding the product of its per-unit speed and the frame delta time to the current position each update.

#### Scenario: scout-reaches-target
- **WHEN** A Scout with speed 6.0 starts at cell (0,0) and targets cell (3,2), and the system runs 120 updates at 1/60 s each
- **THEN** The Scout's GridPosition equals (3,2) within the 120 updates

#### Scenario: speed-proportionality
- **WHEN** Two units cover the same 3-tile distance, one with speed 3.0 and one with speed 6.0, at 1/60 s per update
- **THEN** The speed-3.0 unit takes between 1.5 and 2.5 times as many updates as the speed-6.0 unit

### Requirement: grid-position-derivation
The movement system SHALL derive GridPosition from the continuous position by flooring each axis to i32, and SHALL write GridPosition only when the derived cell differs from the current value.

#### Scenario: cell-change-writes-grid
- **WHEN** A unit's continuous position crosses from cell (1,1) to cell (2,1)
- **THEN** GridPosition is updated to (2,1)

#### Scenario: same-cell-no-write
- **WHEN** A unit's continuous position changes from (1.1, 1.1) to (1.2, 1.2)
- **THEN** GridPosition remains (1,1) and is not modified

### Requirement: target-arrival-clears-destination
The movement system SHALL set TargetDestination.target to None when a unit's continuous position is within 0.1 tiles of its target.

#### Scenario: patrol-continues-after-arrival
- **WHEN** An Explorer arrives at its first patrol waypoint and the patrol system runs on the next update
- **THEN** TargetDestination.target is None after arrival, and the patrol system assigns a new waypoint

### Requirement: renderer-reads-continuous-position
The renderer sync system SHALL compute the unit's Transform from the ContinuousPosition component, not from GridPosition.

#### Scenario: sub-tile-transform
- **WHEN** A unit's ContinuousPosition is (1.5, 2.0) and its GridPosition is (1,2)
- **THEN** The Transform reflects the (1.5, 2.0) position, not the (1,2) grid cell
