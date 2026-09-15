#!/usr/bin/env bash
# scripts/run.sh - Run dreamscaper locally, headless, or in browser
set -euo pipefail

TARGET="${1:-native}"

case "${TARGET}" in
  wasm|web)
    echo "==> Serving dreamscaper for WebAssembly..."
    if ! command -v wasm-server-runner &>/dev/null; then
      echo "wasm-server-runner not found. Installing via cargo..."
      cargo install wasm-server-runner
    fi
    cargo run --target wasm32-unknown-unknown --features wasm
    ;;
  headless)
    echo "==> Running dreamscaper in headless simulation mode..."
    cargo run --features dev -- --headless "${@:2}"
    ;;
  ticks)
    TICKS="${2:-120}"
    echo "==> Running dreamscaper headless simulation for ${TICKS} ticks..."
    cargo run --features dev -- --headless --ticks "${TICKS}"
    ;;
  xvfb)
    echo "==> Running dreamscaper under virtual X11 framebuffer (xvfb)..."
    if ! command -v xvfb-run &>/dev/null; then
      echo "Error: xvfb-run not found. Run 'sudo apt-get install -y xvfb' first."
      exit 1
    fi
    xvfb-run -a cargo run --features dev "${@:2}"
    ;;
  novnc|desktop)
    echo "==> Running dreamscaper for in-browser desktop (desktop-lite / noVNC)..."
    echo "==> Make sure you have opened http://localhost:6080 in your browser!"
    export DISPLAY="${DISPLAY:-:1}"
    cargo run --features dev "${@:2}"
    ;;
  dev)
    echo "==> Running dreamscaper with fast dynamic linking..."
    if [[ -z "${DISPLAY:-}" && -z "${WAYLAND_DISPLAY:-}" ]]; then
      echo "==> Note: No display server detected (\$DISPLAY or \$WAYLAND_DISPLAY)."
      echo "==> Falling back to headless simulation mode. Use 'xvfb' or 'novnc' target for GUI."
    fi
    cargo run --features dev "${@:2}"
    ;;
  native|*)
    echo "==> Running dreamscaper desktop..."
    cargo run "${@:2}"
    ;;
esac

