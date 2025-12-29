# Echoes of Beastlight - Development Environment
FROM rust:1.91-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    # Build essentials
    build-essential \
    pkg-config \
    # Git for version control
    git \
    # Python for YAML validation and scripts
    python3 \
    python3-pip \
    python3-yaml \
    # Node.js for build tools and custom action
    nodejs \
    npm \
    # Required for Bevy
    libasound2-dev \
    libudev-dev \
    # Utilities
    curl \
    jq \
    # Clean up
    && rm -rf /var/lib/apt/lists/*

# Install Rust toolchain components
RUN rustup component add rustfmt clippy
RUN rustup target add wasm32-unknown-unknown

# Install cargo tools
RUN cargo install wasm-bindgen-cli
RUN cargo install cargo-watch
RUN cargo install cargo-edit

# Install Node.js tools globally
RUN npm install -g http-server
RUN npm install -g yaml-lint

# Install Python packages
RUN pip3 install --no-cache-dir pyyaml

# Set up working directory
WORKDIR /workspace

# Copy package files first for better caching
# (Commented out as package.json files are missing in this repository)
# COPY build-tools/package*.json ./build-tools/
# COPY .github/actions/openai-game-gen/package*.json ./.github/actions/openai-game-gen/

# Install Node dependencies
# RUN cd build-tools && npm ci
# RUN cd .github/actions/openai-game-gen && npm ci && npm run build

# Copy the rest of the project
COPY . .

# Set environment variables
ENV RUST_BACKTRACE=1
ENV CARGO_TERM_COLOR=always
ENV BEVY_ASSET_PATH=/workspace/assets
ENV GAME_CONFIG_PATH=/workspace/game-config.yaml

# Create directories that might not exist
RUN mkdir -p src assets/data assets/levels assets/tilemaps assets/prompts

# Validate the setup
RUN cargo --version
RUN rustc --version
RUN node --version
RUN python3 --version

# Default command for development
CMD ["cargo", "watch", "-x", "check", "-x", "test", "-x", "run"]
