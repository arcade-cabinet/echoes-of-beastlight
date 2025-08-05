#!/bin/bash

# Script to run the AI game generator

# Check if OPENAI_API_KEY is set
if [ -z "$OPENAI_API_KEY" ]; then
    echo "Error: OPENAI_API_KEY environment variable is not set"
    echo "Please set it with: export OPENAI_API_KEY=your-api-key-here"
    exit 1
fi

# Build if needed
if [ ! -f "target/release/ai-gen" ]; then
    echo "Building AI generator..."
    cargo build -p ai-game-generator --release --bin ai-gen || exit 1
fi

# Run the generator
echo "Running AI game generator..."
echo "This will generate:"
echo "  - Visual style guide and color palette"
echo "  - Core game files (Cargo.toml, main.rs)"
echo "  - ECS components and systems"
echo "  - Level files in Yoleck format (.yol)"
echo "  - Sprite descriptions and images"
echo "  - Procedural audio specifications"
echo "  - UI assets"
echo ""

# Set log level for better output
export RUST_LOG=info,ai_game_generator=debug

# Run with any passed arguments
./target/release/ai-gen "$@"