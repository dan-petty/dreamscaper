#!/usr/bin/env bash
# scripts/build.sh - Multi-target build script for dreamscaper
set -euo pipefail

TARGET="${1:-host}"

echo "==> Building dreamscaper for target: ${TARGET}"

case "${TARGET}" in
  wasm|wasm32|wasm32-unknown-unknown)
    rustup target add wasm32-unknown-unknown
    cargo build --target wasm32-unknown-unknown --release --features wasm
    echo "==> Wasm build complete: target/wasm32-unknown-unknown/release/dreamscaper.wasm"
    ;;
  android|android-arm64|aarch64-linux-android)
    rustup target add aarch64-linux-android
    cargo build --target aarch64-linux-android --release --features android
    echo "==> Android build complete for aarch64-linux-android"
    ;;
  ios|aarch64-apple-ios)
    rustup target add aarch64-apple-ios
    cargo build --target aarch64-apple-ios --release --features ios
    echo "==> iOS build complete for aarch64-apple-ios"
    ;;
  host|native)
    cargo build --release
    echo "==> Host release build complete: target/release/dreamscaper"
    ;;
  dev)
    cargo build --features dev
    echo "==> Development build complete with dynamic linking"
    ;;
  *)
    cargo build --target "${TARGET}" --release
    echo "==> Build complete for ${TARGET}"
    ;;
esac
