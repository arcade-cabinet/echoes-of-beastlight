# Architecture Overview

## System Design

The Echoes of Beastlight AI Game Studio is built as a pure Rust application with two main components:

### 1. Core Game (Main Crate)

- **Engine**: Bevy 0.14
- **Architecture**: Entity Component System (ECS)
- **Target Platforms**: Native (Windows/Mac/Linux) and WebAssembly

### 2. AI Generation Studio (Tools Crate)

- **UI Framework**: egui with bevy_egui integration
- **AI Integration**: async-openai for GPT-4/DALL-E 3
- **Style Transfer**: Custom neural style transfer implementation
- **Caching**: cacache for disk-based caching with compression

## Key Components

### Studio Application (`build-tools/src/studio/`)

```
studio/
├── mod.rs          # Main plugin and orchestration
├── wizard.rs       # Project setup wizard (6 steps)
├── generator.rs    # AI generation engine with style transfer
├── gallery.rs      # Asset gallery and management
├── preview.rs      # Live game preview with hot reload
├── editor.rs       # Code editor with syntax highlighting
├── console.rs      # Generation logs and commands
└── theme.rs        # Visual theme configuration
```

### Generation Pipeline

1. **Style Guide Generation**
   - Creates master visual style
   - Extracts style features for consistency
   - Defines color palette and mood

2. **Dependency Graph**
   - Topological sorting of assets
   - Parent-child style relationships
   - Ensures generation order

3. **Parallel Processing**
   - GPU tasks scheduled separately
   - CPU tasks run in parallel
   - Progress tracking per task

4. **Style Transfer**
   - Applied to all visual assets
   - Maintains consistency
   - Pixel art optimization

### AI Integration

```rust
pub struct ConsistentAssetGenerator {
    openai_client: Client<OpenAIConfig>,
    style_transfer: Arc<NeuralStyleTransfer>,
    pixel_processor: Arc<PixelArtProcessor>,
    dependency_graph: Arc<AssetDependencyGraph>,
    smart_cache: Arc<SmartCache>,
    prompt_optimizer: Arc<PromptOptimizer>,
    sprite_optimizer: Arc<SpriteSheetOptimizer>,
}
```

### Caching Strategy

- **Memory Cache**: LRU cache for frequently accessed assets
- **Disk Cache**: Zstd compressed storage
- **Metadata**: Stored alongside assets in JSON format
- **Versioning**: Content-based hashing for cache keys

### Communication Architecture

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│   Studio    │────▶│  Generator   │────▶│   OpenAI    │
│     UI      │     │   Thread     │     │     API     │
└─────────────┘     └──────────────┘     └─────────────┘
       │                    │
       ▼                    ▼
┌─────────────┐     ┌──────────────┐
│   Preview   │     │    Cache     │
│   Engine    │     │   Storage    │
└─────────────┘     └──────────────┘
```

## Data Flow

1. **User Input** → Wizard Configuration
2. **Configuration** → Generation Request
3. **Generator** → OpenAI API Calls
4. **AI Response** → Style Transfer
5. **Processed Asset** → Cache Storage
6. **Cache** → Gallery Display
7. **Assets** → Live Preview

## Performance Optimizations

- **Parallel Generation**: Independent assets generated concurrently
- **Smart Caching**: Avoid regenerating identical requests
- **Prompt Learning**: Optimize prompts based on success metrics
- **Sprite Atlasing**: Combine sprites for efficient rendering
- **Lazy Loading**: Load assets on-demand in gallery

## Security Considerations

- **API Key Storage**: Environment variable only
- **No Cloud Storage**: All data stored locally
- **Sandboxed Preview**: Game runs in separate context
- **Input Validation**: All user inputs sanitized

## Future Enhancements

1. **Cloud Collaboration**: Multi-user project support
2. **Custom Models**: Fine-tuned models for game assets
3. **Version Control**: Built-in git integration
4. **Plugin System**: Extensible generation pipeline
5. **Real-time Collaboration**: Live editing with others
