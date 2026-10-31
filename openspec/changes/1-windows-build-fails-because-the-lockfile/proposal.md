# Proposal

## Why

The Windows CI build fails at the Build step because Cargo.lock resolves two versions of the windows crate. wgpu-hal 29.0.4 uses windows 0.62.2 while gpu-allocator 0.28.0 and gilrs-core 0.6.8 are locked to windows 0.58.0. The compiler sees types from different windows crate versions as incompatible, producing E0308 errors. See #1.

## What Changes

- Update Cargo.lock to resolve windows from 0.58.0 to 0.62.2 for gpu-allocator 0.28.0 and gilrs-core 0.6.8
- Unify all windows crate consumers (wgpu-hal, gpu-allocator, gilrs-core) on version 0.62.2
- The change fits one pull request.

## Capabilities

### New Capabilities

- `dependency-resolution`: Unified windows crate version, No stale windows 0.58.0 in dependency graph

### Modified Capabilities

None.

## Impact

- Code: `Cargo.lock`

## Feasibility

- Cargo.lock confirms gpu-allocator 0.28.0 depends on windows 0.58.0 and gilrs-core 0.6.8 depends on windows 0.58.0, while wgpu-hal 29.0.4, accesskit_windows 0.32.1, and cpal 0.17.3 use windows 0.62.2.
- gpu-allocator 0.28.0 declares windows >=0.53, <=0.62 in its Cargo.toml, so 0.62.2 is within range.
- gilrs-core 0.6.8 declares windows >=0.44, <=0.62 in the crates.io index, so 0.62.2 is within range.
- First check: run `cargo tree --target x86_64-pc-windows-msvc -i windows@0.58.0` to confirm the current dual-version state.
- Second check: after the update, run `cargo tree --target x86_64-pc-windows-msvc -i windows` to confirm a single version appears.

## Existing Tool Considered

Cargo (the Rust package manager) provides `cargo update -p <pkg>@<ver> --precise <target>` for targeted lockfile updates. No new tool or library is required. No internal code is built or deleted.
