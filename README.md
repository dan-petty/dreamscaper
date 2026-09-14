# dreamscaper

> **High-performance cross-platform isometric RTS engine written in Rust, powered by Bevy 0.13+ and GPU compute shaders.**

[![CI](https://github.com/dan-petty/dreamscaper/actions/workflows/ci.yml/badge.svg)](https://github.com/dan-petty/dreamscaper/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust: 1.76+](https://img.shields.io/badge/Rust-1.76%2B-orange.svg)](https://www.rust-lang.org/)
[![Bevy: 0.13](https://img.shields.io/badge/Bevy-0.13-blue.svg)](https://bevyengine.org/)
[![WebGPU](https://img.shields.io/badge/WebGPU-Supported-green.svg)](https://w3.org/TR/webgpu/)

`dreamscaper` merges classic isometric RTS depth (inspired by *Age of Empires* and *Command & Conquer*) with emergent, GPU-accelerated simulation. It leverages WebGPU compute shaders (`wgpu`) to simulate autonomous steering, flow fields, and collision avoidance for thousands of units concurrently. In-game isometric tiles are living entities that dynamically modulate moisture, temperature, and visual tint according to diurnal solar cycles and environmental conditions.

---

## 🌟 Highlights & Features

- **Data-Driven Bevy 0.13+ ECS**: Decoupled, modular plugins for rendering, compute AI, environment management, and game logic.
- **GPU Compute AI (`wgpu` / WGSL)**: Offloads agent pathfinding, flocking vector fields, and collision avoidance to GPU workgroups with seamless CPU simulation fallback.
- **Dynamic Reactive Tiles**: Procedural tiles modulate elevation, surface moisture, temperature, and day/night visual shading in real time.
- **Isometric 2:1 Projection**: Complete coordinate projection pipeline with world-to-grid mapping and automated render depth sorting.
- **Diurnal Environment Cycle**: Configurable day/night clock governing sunlight curves, ambient light color temperature, and surface condensation.
- **Multi-Platform Portability**: Native single-codebase builds for Linux, Windows, macOS, WebAssembly (browser), Android, and iOS.
- **Ultra-Fast Developer Iteration**: Optional `dev` feature flag enabling Bevy dynamic linking for sub-second recompile and relink times.

---

## 🏛️ Engine Architecture

The codebase strictly decouples reusable engine primitives (`dreamscaper::engine`) from title-specific game logic (`dreamscaper::game`):

```
                                  +--------------------+
                                  |    src/main.rs     |
                                  +---------+----------+
                                            |
                   +------------------------+-----------------------+
                   |                                                |
        +----------v-----------+                         +----------v----------+
        | dreamscaper::engine  |                         |  dreamscaper::game  |
        +----------+-----------+                         +----------+----------+
                   |                                                |
 +--------+--------+--------+--------+                    +---------+---------+
 |        |                 |        |                    |         |         |
+v---+ +--v---+          +--v---+ +--v---+             +--v---+  +--v---+  +--v--+
|Rend| |  AI  |          | Res  | | ECS  |             | Map  |  | Unit |  | UI  |
+----+ +------+          +------+ +------+             +------+  +------+  +-----+
```

### Module Responsibilities

| Module | Location | Responsibility |
|---|---|---|
| **Renderer** | `src/engine/renderer/` | 2:1 isometric camera projection, depth sorting, and screen-to-grid raycasts |
| **Compute AI** | `src/engine/ai/` | WebGPU compute shader dispatching, `GpuAgent` buffer binding, and CPU fallback steering |
| **Resources** | `src/engine/resources/` | Diurnal cycle clock (`GameTimeOfDay`), environment settings, and `TileMapAsset` loaders |
| **Core ECS** | `src/engine/ecs/` | Spatial coordinates (`GridPosition`, `IsometricCoordinates`), `Unit`, and `DynamicTile` components |
| **Game Map** | `src/game/map.rs` | Procedural isometric terrain generation and tile moisture/temperature update systems |
| **Game Units** | `src/game/unit.rs` | Spawning, unit attributes (Worker, Scout), and player order dispatch |
| **Game UI** | `src/game/ui.rs` | Real-time RTS HUD overlay, active unit counters, and selection indicators |

---

## 📁 Repository Layout

```
.
├── assets/
│   ├── data/
│   │   └── map_default.json         # Default 32x32 isometric map definition
│   └── shaders/
│       ├── ai_compute.wgsl          # WebGPU compute shader for autonomous AI steering
│       └── tile_dynamic.wgsl        # Dynamic tile shader reacting to diurnal curves
├── docs/
│   ├── api.md                       # Public APIs, coordinate formulas, and plugin interfaces
│   ├── architecture.md              # Subsystem design, memory layouts, and data flow
│   └── design.md                    # RTS simulation mechanics and gameplay vision
├── scripts/
│   ├── build.sh                     # Cross-platform compilation script (desktop, wasm, mobile)
│   ├── run.sh                       # Quick runner for desktop and WebAssembly
│   └── setup_dev.sh                 # Environment setup script for targets and dependencies
├── src/
│   ├── engine/                      # Core reusable engine plugins
│   │   ├── ai/                      # Compute shader AI & agent structures
│   │   ├── ecs/                     # Common components & coordinate definitions
│   │   ├── renderer/                # Isometric camera & projection math
│   │   ├── resources/               # Environmental clock & custom map assets
│   │   └── engine_plugin.rs         # Root EnginePlugin aggregating engine subsystems
│   ├── game/                        # RTS title logic (map, units, HUD)
│   ├── lib.rs                       # Library entry point
│   └── main.rs                      # Binary entry point and Bevy App bootstrap
├── tests/
│   ├── engine_tests.rs              # Unit tests for isometric math and diurnal cycle
│   └── integration_tests.rs         # Integration tests for Bevy App plugin registration
├── .cargo/
│   └── config.toml                  # Target overrides and wasm-server-runner configuration
├── .devcontainer/
│   └── devcontainer.json            # VS Code Dev Container with graphics/audio toolchains
├── .github/workflows/
│   └── ci.yml                       # Matrix CI (Linux, Windows, macOS, WebAssembly)
├── Cargo.toml                       # Package manifest, dependencies, and profiles
├── LICENSE                          # MIT License
└── README.md
```

---

## 🎮 Controls & Mechanics

| Input | Action | Description |
|---|---|---|
| **Left Click** | Select Unit | Selects individual unit under cursor |
| **Click & Drag** | Box Select | Selects all friendly units within the selection rectangle |
| **Right Click** | Contextual Order | Issues move or gather orders to selected units |
| **WASD / Arrow Keys** | Pan Camera | Scrolls the isometric camera across the world map |
| **Scroll Wheel** | Zoom | Zooms in/out with orthographic scale preservation |

---

## ⚙️ Cargo Features

`dreamscaper` exposes modular feature flags to optimize compilation and platform targets:

| Feature | Default | Description |
|---|---|---|
| `dev` | No | Enables Bevy's `dynamic_linking` for sub-second local compile times |
| `wasm` | No | Activates `wasm-bindgen`, `web-sys`, `js-sys`, and WebGL2 for browser deployments |
| `webgl2` | No | Enables WebGL2 rendering fallback for older browsers |
| `android` | No | Enables `bevy/android_shared_stdcxx` for Android NDK builds |
| `ios` | No | Target configuration for iOS compilation |

---

## 🛠️ Getting Started & Building

### 1. Prerequisites

- **Rust Toolchain**: Rust 1.76+ stable (`rustup default stable`).
- **Linux Packages** (Ubuntu / Debian):
  ```bash
  sudo apt-get update && sudo apt-get install -y \
    pkg-config libx11-dev libasound2-dev libudev-dev \
    libxkbcommon-x11-0 libvulkan-dev libwayland-dev libxkbcommon-dev
  ```
- **Fedora**:
  ```bash
  sudo dnf install -y pkg-config libX11-devel alsa-lib-devel systemd-devel vulkan-loader-devel
  ```
- **Arch Linux**:
  ```bash
  sudo pacman -S --needed pkgconf alsa-lib systemd vulkan-icd-loader
  ```
- **macOS & Windows**: Graphics drivers and native linkers are supported out-of-the-box (Metal on macOS, DirectX 12 / Vulkan on Windows).

---

### 2. Desktop Development (Fast Iteration)

Run with dynamic linking for rapid local iteration:

```bash
cargo run --features dev
```

For optimized release performance:

```bash
cargo run --release
```

---

### 3. WebAssembly (Browser)

Build and run in the browser using WebGPU / WebGL2:

```bash
# Add Wasm target & runner
rustup target add wasm32-unknown-unknown
cargo install wasm-server-runner

# Run web build
cargo run --target wasm32-unknown-unknown --features wasm
```

---

### 4. Cross-Platform Compilation Scripts

Use the included [`scripts/build.sh`](scripts/build.sh) helper:

```bash
./scripts/build.sh host       # Compile native desktop binary (target/release/dreamscaper)
./scripts/build.sh wasm       # Compile WebAssembly package
./scripts/build.sh android    # Compile Android aarch64 shared library
./scripts/build.sh ios        # Compile iOS aarch64 binary
./scripts/build.sh dev        # Fast compile with dynamic linking
```

---

## 🧪 Testing & Code Quality

Execute the automated test suite:

```bash
# Run unit and integration tests
cargo test

# Check code formatting
cargo fmt --all -- --check

# Run static analysis and clippy lints
cargo clippy --all-targets --all-features -- -D warnings
```

---

## 🐳 Dev Container Support

This repository includes a fully configured [VS Code Dev Container](.devcontainer/devcontainer.json) based on Debian Bookworm with:
- Pre-installed Rust stable and `wasm32-unknown-unknown` target.
- Pre-installed Linux audio (`ALSA`), `udev`, and `Vulkan` libraries.
- Pre-configured `rust-analyzer` and `lldb` debugging extensions.

---

## 🗺️ Roadmap

- [x] Modern Bevy 0.13+ ECS & Plugin Architecture.
- [x] Isometric 2:1 coordinate projection and depth sorting.
- [x] WebGPU WGSL compute shaders for semi-autonomous steering.
- [x] Diurnal daylight simulation and reactive tile dynamics.
- [x] Multi-target matrix CI for Linux, macOS, Windows, and Wasm.
- [ ] Multi-tile structure placement and building construction queues.
- [ ] Fog of war compute shader pass.
- [ ] Deterministic lockstep network multiplayer.

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
