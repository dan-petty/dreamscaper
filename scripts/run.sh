#!/usr/bin/env bash
# scripts/run.sh - Run dreamscaper locally or in web browser
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
  dev)
    echo "==> Running dreamscaper with fast dynamic linking..."
    cargo run --features dev
    ;;
  native|*)
    echo "==> Running dreamscaper desktop..."
    cargo run
    ;;
esac
