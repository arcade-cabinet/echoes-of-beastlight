# AI Agent Technical Context & Memory

## Current System State (2025-08-05)

### Active Build Issues

1. **String Literal Syntax Error** - Fixed by adding space in raw string: `"#f4f4f4 "`
2. **Missing ImageResponseFormat** - Removed, not needed for URL response
3. **Borrowing Issues** - Clone config values before async operations
4. **DateTime Serde** - Use `#[serde(with = "chrono::serde::ts_seconds")]`
5. **Git Tracker Debug** - Added `#[derive(Debug)]` to GitGenerationTracker

### Architecture Decisions Made

#### Pure Rust Implementation

- **Rationale**: Single language for game + generator, better performance, type safety
- **Key Libraries**:
  - `async-openai` (0.23) - OpenAI API client
  - `tiktoken-rs` (0.5) - Token counting
  - `git2` (0.18) - Git integration for idempotency
  - `bevy` (0.14) - Game engine and studio UI
  - `bevy-inspector-egui` (0.25) - Runtime inspection

#### Division of Responsibility

- **AI Generator (Headless)**: What I run
  - CLI tool for batch generation
  - No GUI dependencies for generator binary
  - Manages all AI API calls and caching
  - Git-based idempotency tracking

- **Studio (GUI)**: Director's tool
  - Full Bevy/Egui interface
  - Code review, asset gallery, live preview
  - Requires SDL2 system libraries
  - Feature-gated to avoid build issues

### Current Generation Pipeline

```rust
// Phase 1: Style Guide (MUST BE FIRST)
generate_style_guide() -> {
    "assets/style/color-palette.json",
    "assets/style/style-rules.md",
    "assets/style/style-reference.png"
}

// Phase 2: Core Game Structure
generate_core() -> {
    "Cargo.toml",
    "src/main.rs"
}

// Phase 3: ECS Components & Systems
generate_components() -> "src/components.rs"
generate_systems() -> [
    "src/systems/movement.rs",
    "src/systems/combat.rs",
    "src/systems/taming.rs",
    "src/systems/inventory.rs"
]

// Phase 4: Content Generation
generate_levels() -> {
    "assets/levels/*.yol",  // Bevy-Yoleck format
    "assets/levels/index.yoli"
}

generate_sprites() -> {
    "assets/sprites/hero.png",
    "assets/sprites/tileset.png"
}

generate_audio() -> {
    "assets/audio/audio_specs.json",  // Procedural specs
    "assets/audio/game-audio.js",     // Web Audio impl
    "assets/audio/README.md"
}

// Phase 5: UI Assets (uses style guide)
generate_ui_assets() -> "assets/sprites/ui-elements.png"
```

### Idempotency System

#### Git-Based Tracking

```rust
struct GenerationManifest {
    id: Uuid,
    timestamp: DateTime<Utc>,
    cascade_tree: CascadeTree,  // Full prompt hierarchy
    generated_files: HashMap<PathBuf, FileMetadata>,
    cache_keys: HashMap<String, String>,
}
```

#### File Tracking

- Every file has metadata: hash, size, prompt_hash, generation_time
- Parent asset dependencies tracked for cascade invalidation
- Git commits preserve full generation history

### Caching Strategy

1. **Prompt Response Cache**: `.cache/ai-gen/` with MD5 keys
2. **Asset Cache**: Future `SmartCache` with zstd compression
3. **Git Manifest**: `.ai-generation/manifest.json` (NOT gitignored)

### API Integration Details

#### OpenAI Configuration

```rust
// Chat Completions
model: "gpt-4-turbo-preview"
temperature: 0.7
max_tokens: 4000

// Image Generation (DALL-E 3)
size: ImageSize::S1024x1024
n: 1
// Note: response_format not needed, defaults to URL
```

#### Token Management

- Using tiktoken-rs for accurate counting
- Logging token usage for cost tracking
- Future: Implement request batching

### Procedural World Generation

#### Monster Taming System

```rust
struct ProceduralMonster {
    name: String,        // Generated from word banks
    monster_type: String,
    element: Element,
    traits: Vec<String>,
    stats: MonsterStats,
    tameable: bool,
    evolution_paths: Vec<EvolutionPath>,
}
```

#### World DAG Structure

```rust
struct WorldDAG {
    nodes: HashMap<NodeId, WorldNode>,
    edges: Vec<Edge>,
    starting_location: NodeId,
}

struct WorldNode {
    location: ProceduralLocation,
    monsters: Vec<ProceduralMonster>,
    connections: Vec<Connection>,
    unlock_requirements: Vec<Requirement>,
}
```

### Pending Implementations

1. **Style Transfer Pipeline**
   - `NeuralStyleTransfer` struct defined but not integrated
   - Need to implement `apply_style_transfer_to_tiles`
   - Pixel art cleanup and quantization

2. **Advanced Caching**
   - `SmartCache` with compression ready to integrate
   - LRU memory cache for hot assets
   - Batch operation support

3. **Prompt Optimization**
   - `PromptOptimizer` learns from success/failure
   - Reinforcement learning for better prompts
   - A/B testing framework

4. **Asset Dependencies**
   - `AssetDependencyGraph` for generation order
   - Topological sort for correct sequencing
   - Cascade invalidation on changes

### Build Configuration

#### Edition 2024

- Using Rust 1.88.0 for edition 2024 support
- Enables latest language features
- Required for some dependencies

#### Feature Flags

```toml
[features]
default = []
studio = ["dep:bevy", "dep:bevy_egui", ...]
```

### Next Technical Tasks

1. **Fix Remaining Build Errors**
   - `with_cache` -> `with_use_cache` in main.rs
   - Complete error handling in generator.rs
   - Resolve all borrowing issues

2. **Complete Git Integration**
   - Uncomment git_tracker code
   - Test manifest generation
   - Implement diff preview

3. **Integrate Advanced Features**
   - Wire up style transfer
   - Enable parallel generation
   - Implement sprite atlasing

4. **Testing Infrastructure**
   - Unit tests for each component
   - Integration tests for full pipeline
   - Benchmark generation performance

### Command Reference

```bash
# Build headless generator
cargo build --bin ai-gen

# Build studio with GUI
cargo build --bin studio --features studio

# Run generation
./run-generator.sh generate --force

# Generate specific component
cargo run --bin ai-gen -- component --type sprite --name boss

# Clean cache
cargo run --bin ai-gen -- clean
```

### Environment Requirements

- Rust 1.88.0+ (for edition 2024)
- SDL2 development libraries (for studio only)
- Git (for idempotency tracking)
- OPENAI_API_KEY environment variable

### Performance Optimizations

1. **Parallel Generation**: Use rayon for CPU tasks
2. **Async I/O**: Tokio for all file/network ops
3. **Smart Caching**: Avoid redundant API calls
4. **Batch Processing**: Group similar requests

### Error Recovery

- All errors use anyhow with context
- Graceful fallbacks for missing assets
- Partial generation recovery from manifest
- Automatic retry with exponential backoff
