# Proposal

## Why

Issue #4 reports that .cargo/config.toml requires undocumented clang and lld for x86_64 Linux linking, and sets BEVY_ASSET_ROOT to a path that Bevy resolves one level too deep. The x86_64 override is redundant since Rust 1.90 ships rust-lld by default on that target. The aarch64 override is needed but not documented in the README. The dev container installs mold which nothing uses.

## What Changes

- Delete the [target.x86_64-unknown-linux-gnu] block from .cargo/config.toml.
- Delete the [env] block that sets BEVY_ASSET_ROOT from .cargo/config.toml.
- Remove lld and clang from both apt install lines in .github/workflows/ci.yml.
- Remove mold from the postCreateCommand in .devcontainer/devcontainer.json.
- Add clang and lld to the README Linux prerequisites with an aarch64-only note.
- The change fits one pull request.

## Capabilities

### New Capabilities

- `linux-build-configuration`: x86_64-linux-linker, aarch64-linux-linker, bevy-asset-root, ci-linux-packages, readme-aarch64-prerequisites, dev-container-packages

### Modified Capabilities

None.

## Impact

- Code: `.cargo/config.toml`, `.github/workflows/ci.yml`, `.devcontainer/devcontainer.json`
- Documents: `README.md`

## Feasibility

- The x86_64 override is confirmed unnecessary: rust-toolchain.toml sets channel = "stable", and the issue cites rust-lang/rust#140525 for Rust 1.90+ defaulting to rust-lld on x86_64-unknown-linux-gnu.
- The aarch64 override is confirmed needed: .cargo/config.toml lines 7-9 set linker = "clang" with -fuse-ld=lld for that target, and the issue states aarch64 does not default to rust-lld.
- BEVY_ASSET_ROOT is confirmed misconfigured: .cargo/config.toml:21 sets value = "assets" with relative = true, and the issue explains Bevy appends its own assets path to it, producing assets/assets.
- CI is confirmed to install lld clang: .github/workflows/ci.yml:23 and :56 both list lld clang in the apt install line.
- The dev container is confirmed to install mold: .devcontainer/devcontainer.json:9 includes mold in the postCreateCommand package list.
- First implementation check: run `cargo build` on x86_64 Linux with only the README-listed packages and confirm exit code 0.
- Second implementation check: run `grep -c BEVY_ASSET_ROOT .cargo/config.toml` and confirm the output is 0.

## Existing Tool Considered

The Rust toolchain (stable, 1.90+) provides rust-lld as the default linker on x86_64-unknown-linux-gnu. No additional tool or library is needed for x86_64 linking. The change removes the redundant clang and lld system packages and the misconfigured BEVY_ASSET_ROOT variable. No internal code is deleted; only configuration lines are removed.
