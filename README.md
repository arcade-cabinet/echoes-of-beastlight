# 🎮 Echoes of Beastlight - AI Game Generation Studio

A revolutionary AI-powered game development studio built entirely in Rust, featuring real-time game generation, visual consistency through style transfer, and a complete IDE for creating JRPG games.

## 🚀 Overview

This project demonstrates cutting-edge AI game generation with:
1. **Pure Rust Implementation** - No JavaScript, just fast, safe Rust code
2. **Integrated Game Studio** - Complete visual IDE with Bevy and egui
3. **Style Transfer System** - Ensures visual consistency across all generated assets
4. **Self-Improving AI** - Learns from successful generations to optimize prompts

## 🎯 Key Features

- **AI Studio Application**: Full-featured game development IDE
- **Neural Style Transfer**: Consistent art style across all assets
- **Dependency Graph**: Smart generation ordering for related assets
- **Real-time Preview**: Hot-reload game preview with inspector
- **Cross-Platform**: Native builds for Windows, Linux, macOS + Web (WASM)
- **Modern Tech Stack**: Rust + Bevy for performance and safety

## 📁 Project Structure

```
.
├── game-config.yaml              # Master configuration file
├── Cargo.toml                    # Rust workspace configuration
├── src/                          # Game source code
│   ├── main.rs                   # Game entry point
│   ├── components.rs             # ECS components
│   └── systems/                  # Game systems
├── tools/                        # AI generation tools
│   ├── Cargo.toml               # Generator dependencies
│   ├── src/
│   │   ├── main.rs              # CLI generator
│   │   ├── studio_main.rs       # Studio application
│   │   ├── studio/              # Studio modules
│   │   │   ├── wizard.rs        # Project setup wizard
│   │   │   ├── generator.rs     # AI generation engine
│   │   │   ├── gallery.rs       # Asset gallery
│   │   │   ├── preview.rs       # Live game preview
│   │   │   └── console.rs       # Generation console
│   │   ├── config.rs            # Configuration parser
│   │   └── templates.rs         # Code templates
├── assets/                       # Game assets
├── .github/
│   └── prompts/                  # AI prompt templates
│       ├── game-generation/      # Asset generation prompts
│       └── metaprompts/          # Cascading prompts
└── target/                       # Build output
```

## 🛠️ Setup

### Prerequisites

- Rust 1.70+ with cargo
- OpenAI API key
- Git

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/echoes-of-beastlight.git
cd echoes-of-beastlight
```

2. Set your OpenAI API key:
```bash
export OPENAI_API_KEY="your-api-key-here"
```

3. Build the project:
```bash
cargo build --release
```

## 🎮 Usage

### Command Line Generator

Generate game assets using the CLI:

```bash
# Generate complete game
cargo run --bin ai-gen -- generate

# Generate specific component
cargo run --bin ai-gen -- component character

# Test generation
cargo run --bin ai-gen -- test
```

### Visual Studio Application

Launch the full game development studio:

```bash
cargo run --bin studio
```

The studio provides:
- **Project Wizard**: 6-step guided game configuration
- **Asset Gallery**: View and manage all generated assets
- **Live Preview**: Real-time game preview with hot reload
- **Code Editor**: Syntax-highlighted Rust code viewer
- **Inspector**: Runtime entity editing with bevy-inspector-egui
- **Console**: Generation logs and commands

## 🎨 AI Generation Pipeline

### 1. Style Guide Generation
The system first generates a master style guide that defines:
- Color palette
- Art style references
- Visual consistency rules

### 2. Dependency-Aware Generation
Assets are generated in topological order based on dependencies:
- Style parents define visual style for children
- Color references share palettes
- Size references maintain consistent dimensions

### 3. Style Transfer
Every generated asset goes through style transfer to ensure consistency:
- Neural style transfer applies the master style
- Pixel art processing for clean sprites
- Automatic outlining and color quantization

### 4. Optimization
- Smart caching with compression
- Prompt learning from successful generations
- Parallel processing for independent assets
- Sprite sheet optimization

## 🔧 Configuration

Edit `game-config.yaml` to customize your game:

```yaml
game_title: "Echoes of Beastlight"
genre: "JRPG"
setting: "Post-apocalyptic fantasy"

graphics:
  style: "pixel_art"
  perspective: "top_down_2d"
  tile_size: 16
  sprite_size: 32

features:
  combat_system: true
  monster_taming: true
  inventory_system: true
```

## 📚 Prompt Templates

The system uses cascading prompt templates in `.github/prompts/`:

- **Metaprompts**: High-level prompts that generate other prompts
- **Generation Prompts**: Specific prompts for each asset type
- **Level Design Cascade**: Multi-stage level generation

## 🚀 Building for Production

### Native Build
```bash
cargo build --release
```

### Web Build (WASM)
```bash
./build-web.sh
```

### Run the Game
```bash
cargo run --release
```

## 🤝 Contributing

Contributions are welcome! Please read our contributing guidelines and submit PRs.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Built with [Bevy](https://bevyengine.org/) game engine
- UI powered by [egui](https://github.com/emilk/egui)
- AI generation via [OpenAI](https://openai.com/)
- Style transfer inspired by neural style transfer research
