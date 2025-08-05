#!/bin/bash
set -euo pipefail

echo "🎮 Building Echoes of Beastlight for Web..."

# Ensure WASM target
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen if needed
command -v wasm-bindgen-cli >/dev/null 2>&1 || cargo install wasm-bindgen-cli

# Build
cargo build --release --target wasm32-unknown-unknown

# Generate bindings
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/beastlight.wasm

echo "✅ Build complete! Serve index.html to play."