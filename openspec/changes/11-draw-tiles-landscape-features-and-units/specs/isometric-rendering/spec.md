## Purpose
Defines the visual rendering of the isometric game map, including tile diamonds, unit sprites, landscape features, and day/night colour tinting.

## ADDED Requirements

### Requirement: tiles-render-as-coloured-diamonds
Each tile entity SHALL carry a Mesh2d component referencing a shared diamond mesh and a Material2d component referencing one of six per-type ColorMaterial instances.

#### Scenario: all-tiles-have-visual-components
- **WHEN** the visual plugin is added and the app runs one update cycle
- **THEN** every entity with a GridPosition component has both a Mesh2d and a Material2d component, and the world contains exactly six ColorMaterial resources

### Requirement: units-render-above-their-tile
Each unit entity SHALL have a Transform z value greater than the z value of the tile in the same grid cell.

#### Scenario: unit-z-exceeds-tile-z
- **WHEN** the visual plugin spawns visual components and the app runs one update cycle
- **THEN** for every entity with a Unit component, its Transform z is strictly greater than the Transform z of the tile at the same GridPosition

### Requirement: day-night-tinting-updates-material-colours
The tinting system SHALL multiply each of the six ColorMaterial base colours by the ambient_light scalar from EnvironmentSettings every frame.

#### Scenario: night-darkens-tiles
- **WHEN** EnvironmentSettings.ambient_light is set to Vec3::splat(0.2) and the app runs one update cycle
- **THEN** each of the six ColorMaterial base colours equals the stored base colour multiplied by 0.2

#### Scenario: noon-brightens-tiles
- **WHEN** EnvironmentSettings.ambient_light is set to Vec3::splat(1.0) and the app runs one update cycle
- **THEN** each of the six ColorMaterial base colours equals the stored base colour

### Requirement: headless-mode-runs-without-render-resources
The headless mode and the integration test suite SHALL run to completion without requiring Assets<Mesh> or ColorMaterial resources.

#### Scenario: headless-ticks-complete
- **WHEN** the app is built with MinimalPlugins and the headless tick controller runs 120 ticks
- **THEN** the process exits with status 0 and no panic related to missing render resources
