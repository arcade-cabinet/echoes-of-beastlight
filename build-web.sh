#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

echo "🎮 Building Echoes of Beastlight for Web..."

# Ensure WASM target
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen if needed
command -v wasm-bindgen-cli >/dev/null 2>&1 || cargo install wasm-bindgen-cli

# Build
cargo build -p echoes-of-beastlight --release --target wasm32-unknown-unknown

# Generate bindings
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/echoes-of-beastlight.wasm

# Ensure index.html exists
if [ ! -f ./out/index.html ]; then
    cp game/assets/index.html ./out/index.html || echo "index.html already in out or missing"
fi

# Copy assets
rm -rf ./out/assets
cp -r game/assets ./out/assets

echo "✅ Build complete! Serve ./out to play."
