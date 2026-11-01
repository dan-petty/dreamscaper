## Purpose
Defines the linker, asset path, and system package requirements for building the project on Linux x86_64 and aarch64 targets.

## ADDED Requirements

### Requirement: x86_64-linux-linker
The project SHALL NOT require system clang or lld packages for linking on x86_64-unknown-linux-gnu.

#### Scenario: x86_64 build without clang or lld
- **WHEN** a developer runs cargo build on x86_64-unknown-linux-gnu with only the README-listed packages installed
- **THEN** the build succeeds with exit code 0 and no linker error

#### Scenario: config has no x86_64 override
- **WHEN** the file .cargo/config.toml is inspected
- **THEN** it contains no [target.x86_64-unknown-linux-gnu] block

### Requirement: aarch64-linux-linker
The project SHALL use clang with lld as the linker on aarch64-unknown-linux-gnu.

#### Scenario: aarch64 config block present
- **WHEN** the file .cargo/config.toml is inspected
- **THEN** it contains a [target.aarch64-unknown-linux-gnu] block with linker = "clang" and rustflags including -fuse-ld=lld

### Requirement: bevy-asset-root
The project SHALL NOT set BEVY_ASSET_ROOT in the cargo configuration.

#### Scenario: no BEVY_ASSET_ROOT in config
- **WHEN** the file .cargo/config.toml is inspected
- **THEN** it contains no BEVY_ASSET_ROOT entry

### Requirement: ci-linux-packages
The CI Linux jobs SHALL NOT install clang or lld packages.

#### Scenario: CI apt lines have no clang or lld
- **WHEN** the file .github/workflows/ci.yml is inspected
- **THEN** no apt install line contains the strings lld or clang

### Requirement: readme-aarch64-prerequisites
The README SHALL document clang and lld as prerequisites required on aarch64 Linux hosts only.

#### Scenario: README mentions aarch64 requirement
- **WHEN** the Linux prerequisites section of README.md is read
- **THEN** clang and lld appear with a note that they are required on aarch64 hosts only

### Requirement: dev-container-packages
The dev container SHALL NOT install mold.

#### Scenario: dev container has no mold
- **WHEN** the file .devcontainer/devcontainer.json is inspected
- **THEN** the postCreateCommand package list does not contain the string mold
