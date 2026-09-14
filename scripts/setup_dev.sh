#!/usr/bin/env bash
# scripts/setup_dev.sh - Setup development environment for dreamscaper
set -euo pipefail

echo "==> Setting up development prerequisites for dreamscaper..."

# Add targets
echo "==> Adding wasm32 target..."
rustup target add wasm32-unknown-unknown || echo "Warning: failed to add wasm32 target"

# Install tools if cargo is present
if command -v cargo &>/dev/null; then
  echo "==> Installing wasm-server-runner..."
  cargo install wasm-server-runner --quiet || true
fi

echo "==> Development setup complete!"
