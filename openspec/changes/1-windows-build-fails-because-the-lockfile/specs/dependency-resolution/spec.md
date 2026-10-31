## Purpose
Defines the expected state of the dependency lockfile for cross-platform builds, ensuring all transitive dependencies resolve to compatible versions.

## ADDED Requirements

### Requirement: Unified windows crate version
wgpu-hal, gpu-allocator, and gilrs-core SHALL all resolve to the same windows crate version in Cargo.lock.

#### Scenario: All three crates list windows 0.62.2
- **WHEN** Cargo.lock is inspected for the windows dependency under gpu-allocator 0.28.0 and gilrs-core 0.6.8
- **THEN** Both list windows 0.62.2, matching the version used by wgpu-hal 29.0.4

### Requirement: No stale windows 0.58.0 in dependency graph
The dependency graph MUST NOT contain any package that references windows 0.58.0.

#### Scenario: cargo tree reports no windows 0.58.0
- **WHEN** `cargo tree --target x86_64-pc-windows-msvc -i windows@0.58.0` is run after the lockfile update
- **THEN** The command reports that no such package is in the graph
