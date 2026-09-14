# dreamscaper

> **Cross-platform isometric RTS engine written in Rust, powered by Bevy and GPU compute shaders.**

[![CI](https://github.com/dan-petty/dreamscaper/actions/workflows/ci.yml/badge.svg)](https://github.com/dan-petty/dreamscaper/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`dreamscaper` leverages GPU compute shaders (`wgpu` / WebGPU) for semi-autonomous AI steering and dynamically-generated isometric tiles that react to in-game conditions, environmental pressures, and the diurnal day/night cycle. Builds natively for Linux, Windows, macOS, Android, iOS, and WebAssembly.

---

## 🚀 Features

- **Bevy 0.13+ ECS & Rendering**: Modern, idiomatic data-driven architecture.
- **GPU Compute AI**: Offloads pathfinding, flocking, and collision avoidance to GPU compute workgroups with automatic CPU fallback.
- **Dynamic Reactive Tiles**: Tiles dynamically modulate moisture, temperature, and visual tint according to diurnal solar cycles and environmental conditions.
- **Isometric 2:1 Projection**: Complete coordinate projection pipeline with depth sorting.
- **Multi-Platform Support**: Single Rust codebase targeting Desktop, WebAssembly, Android, and iOS.
- **Fast Developer Iteration**: Dynamic linking profile for near-instant compile/link times during local development.

---

## 📁 Repository Layout

```
.
├── assets/                # WGSL shaders, map configs, textures
│   ├── data/              # Map definitions and biome configurations
│   └── shaders/           # Compute shaders & dynamic tile shaders
├── docs/                  # Architectural deep-dives and API reference
│   ├── architecture.md    # Engine vs game layer architecture
│   ├── design.md          # RTS mechanics & simulation design
│   └── api.md             # Public plugin and coordinate APIs
├── src/                   # Rust source code
│   ├── engine/            # Reusable engine core
│   │   ├── ai/            # GPU compute AI & steering systems
│   │   ├── ecs/           # Common components (GridPosition, Unit, Tile)
│   │   ├── renderer/      # Isometric camera & projection math
│   │   ├── resources/     # Diurnal clock, assets, environment settings
│   │   └── engine_plugin.rs
│   ├── game/              # Game-specific RTS logic
│   │   ├── map.rs         # Procedural grid generation & tile reactivity
│   │   ├── unit.rs        # Units, selection, and orders
│   │   └── ui.rs          # HUD overlay and selection boxes
│   ├── lib.rs             # Library root
│   └── main.rs            # Application entry point
├── scripts/               # Build, run, and development scripts
│   ├── build.sh           # Multi-target build helper
│   ├── run.sh             # Quick runner (desktop / web)
│   └── setup_dev.sh       # Target & tool installer
├── tests/                 # Unit & integration tests
│   ├── engine_tests.rs
│   └── integration_tests.rs
├── .cargo/config.toml     # Target overrides and wasm runner
├── .devcontainer/         # Dev container with Linux graphics/audio dependencies
├── .github/workflows/     # GitHub Actions matrix CI
├── Cargo.toml
├── LICENSE
└── README.md
```

---

## 🛠️ Build & Run

### Prerequisites
- [Rust](https://www.rust-lang.org/) (edition 2021, stable)
- Linux dependencies (if building natively on Ubuntu/Debian):
  ```bash
  sudo apt-get update && sudo apt-get install -y \
    pkg-config libx11-dev libasound2-dev libudev-dev \
    libxkbcommon-x11-0 libvulkan-dev libwayland-dev libxkbcommon-dev
  ```

### Desktop (Linux, macOS, Windows)

```bash
# Run with release optimizations
cargo run --release

# Fast iteration development with dynamic linking
cargo run --features dev
```

### WebAssembly (Browser)

```bash
# Install target and runner
rustup target add wasm32-unknown-unknown
cargo install wasm-server-runner

# Run in browser
cargo run --target wasm32-unknown-unknown --features wasm
```

### Multi-Target Script

Use the helper script in `scripts/build.sh`:
```bash
./scripts/build.sh wasm       # Builds WebAssembly
./scripts/build.sh android    # Builds Android (aarch64)
./scripts/build.sh ios        # Builds iOS (aarch64)
./scripts/build.sh host       # Builds native host
```

---

## 🧪 Testing

Run unit and integration tests:

```bash
cargo test
```

---

## 📜 Architecture

Read the full technical guides in [`docs/`](docs/):
- [`docs/architecture.md`](docs/architecture.md) — System layers, plugin decoupling, and data flow.
- [`docs/design.md`](docs/design.md) — RTS mechanics, diurnal cycles, and GPU steering.
- [`docs/api.md`](docs/api.md) — API reference and coordinate conversion formulas.

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
