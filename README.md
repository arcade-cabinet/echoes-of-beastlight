# 🎮 Echoes of Beastlight - Metaprompt Game Generator

A self-bootstrapping GitHub Actions workflow system that generates an entire JRPG game using AI-powered metaprompts, built with Rust and Bevy.

## 🚀 Overview

This project demonstrates a closed-loop metaprompt system where:
1. A configuration file (`game-config.yaml`) defines the game parameters
2. GitHub Actions workflows read the config and generate everything automatically
3. The system can even improve itself by suggesting config optimizations

## 🎯 Key Features

- **Config-Driven**: Everything stems from `game-config.yaml`
- **Self-Bootstrapping**: Workflows generate other workflows
- **Cross-Platform**: Native builds for Windows, Linux, macOS + **Web (WASM)**
- **Modern Tech Stack**: Rust + Bevy for performance and safety
- **Procedural Generation**: Maps, monsters, and quests are procedurally generated
- **Final Fantasy × Pokémon**: Unique blend of JRPG mechanics

## 📁 Project Structure

```
.
├── game-config.yaml              # Master configuration file
├── init.sh                       # Project initialization script
├── .cursorrules                  # Cursor AI assistant rules
├── environment.json              # Background agent configuration
├── Dockerfile                    # Development environment
├── docker-compose.yml            # Multi-service development setup
├── .github/workflows/
│   ├── generate-file.yml         # Base workflow for OpenAI generation
│   ├── bootstrap-beastlight.yml  # Main bootstrapping workflow
│   └── metaprompt-executor.yml   # Direct metaprompt execution
├── src/                          # Generated Rust source code
│   ├── main.rs                   # Game entry point
│   ├── components.rs             # ECS components
│   ├── resources.rs              # Game resources
│   └── systems/                  # Game systems
├── assets/                       # Game assets
│   ├── data/                     # Game data (monsters, quests)
│   ├── sprites/                  # Sprite specifications
│   └── audio/                    # Audio specifications
├── tools/                        # Build and validation tools
├── Cargo.toml                    # Rust dependencies
├── build-web.sh                  # Web build script
└── index.html                    # Web deployment page
```

## 🔧 Setup

1. **Fork this repository**

2. **Initialize the project (optional but recommended)**:
   Run the initialization script to create the necessary directory structure.
   ```bash
   ./init.sh
   ```
   Note: You may need to make it executable first: `chmod +x ./init.sh`

3. **Add OpenAI API Key**:
   - Go to Settings → Secrets → Actions
   - Add a new secret named `OPENAI_API_KEY`
   - Paste your OpenAI API key

4. **Install Rust** (for local development):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   cargo install wasm-bindgen-cli
   ```

5. **Trigger Generation**:
   - Option 1: Push a change to `game-config.yaml`
   - Option 2: Go to Actions → Select a workflow → Run workflow

## 🤖 Cursor Integration

This project includes advanced Cursor AI integration for enhanced development experience.

### Cursor Rules (`.cursorrules`)
The project includes comprehensive rules that help Cursor understand:
- Project structure and conventions
- Code generation patterns for Rust + Bevy
- How to use the custom OpenAI action
- Best practices for tilemaps, levels, and monsters
- Common tasks and workflows

### Background Agent Setup
To enable Cursor's background agent for automatic improvements:

1. **Copy environment variables**:
   ```bash
   cp .env.example .env
   # Edit .env with your API keys
   ```

2. **Using Docker** (recommended):
   ```bash
   docker-compose up -d
   ```
   This starts:
   - Development environment with auto-reload
   - Asset validator running continuously
   - Optional web server for WASM testing

3. **Manual setup**:
   The `environment.json` configures background tasks:
   - Auto-formatting on save
   - YAML validation
   - Continuous build checks
   - Asset validation every 5 minutes
   - Config change monitoring

### Background Agent Features
- **Auto-fix**: Formats Rust code and validates YAML on save
- **Validation**: Continuously checks asset integrity
- **Monitoring**: Detects config changes and suggests regeneration
- **Health checks**: Ensures both native and WASM builds work

## 🎮 Game Mechanics

### Core Gameplay Loop
- **3/4 Top-Down Perspective**: Classic JRPG view
- **Monster Taming vs Slaying**: Choose between XP or new party members
- **Procedural Maps**: 12×12 grids with guaranteed traversal
- **Dungeon Crawling**: Maze algorithm with multiple false paths
- **Progressive Difficulty**: Logarithmic scaling based on area progression

### Unique Features
- **Party Size Growth**: Start solo, gain slots by clearing dungeons
- **Dynamic Monster Generation**: Combines nouns, verbs, and adjectives
- **Tile Recoloring**: Maximum variety with minimal assets
- **Boss Scaling**: Based on last 5 maps of progression

## 🔄 Workflow Types

### 1. `generate-file.yml`
Base workflow that calls OpenAI API to generate any file.

### 2. `bootstrap-beastlight.yml`
Complex multi-phase workflow that:
- Generates other workflows
- Creates game assets
- Generates Rust + Bevy code
- Produces game data
- Sets up build system

### 3. `metaprompt-executor.yml`
Simpler workflow that executes the vintage game metaprompt directly.

### 4. `generate-tilemaps.yml`
Specialized workflow for bevy_ecs_tilemap integration.

### 5. `generate-levels.yml`
Creates levels using bevy-yoleck and mapgen.rs algorithms.

## 📝 Configuration Schema

```yaml
game:
  title: "Echoes of Beastlight"
  codename: "beastlight"
  
platform:
  engine: "bevy"
  language: "rust"
  supported: ["windows", "linux", "macos", "web"]
  
theme:
  setting: "Arcane wilderness survival meets ancient techno-ruins"
  
gameplay:
  core_mechanics:
    - "Monster slaying OR taming choice"
    - "Procedural map generation"
```

## 🛠️ Development

To modify the game:

1. Edit `game-config.yaml`
2. Push changes
3. Watch as workflows regenerate everything
4. The system will create:
   - Rust source files with Bevy ECS
   - Monster databases
   - Quest templates
   - Build configurations
   - Web deployment files

### Using Docker for Development
```bash
# Start development environment
docker-compose up

# Run specific services
docker-compose up dev        # Main development container
docker-compose up validator  # Asset validation only
docker-compose --profile web up  # Include web server

# Run commands in container
docker-compose exec dev cargo test
docker-compose exec dev ./build-web.sh
```

## 🎨 Asset Generation

The system generates prompts for:
- **Sprites**: Hero, monsters, NPCs
- **Tiles**: Environment blocks with recoloring
- **Audio**: Music tracks and sound effects
- **UI Elements**: Menus, dialog boxes, health bars

## 🔮 Self-Improvement

The system includes a meta-optimization workflow that can:
- Analyze current configuration
- Suggest improvements
- Test Rust compilation
- Check WASM build size
- Propose balance changes
- Create optimized config versions

## 🏗️ Building the Game

### Native Build
```bash
cargo run --release
```

### Web Build
```bash
./build-web.sh
# Then serve the index.html file
python3 -m http.server 8000
# Visit http://localhost:8000
```

### Deploy to GitHub Pages
The generated workflow can automatically deploy to GitHub Pages, making your game playable online!

## 🌐 Play Online

Once deployed, your game will be available at:
`https://[your-username].github.io/echoes-of-beastlight/`

## 📜 License

This project is open source. The metaprompt system and workflows are free to use and modify.

## 🙏 Credits

- Inspired by classic JRPGs (Final Fantasy, Pokémon)
- Powered by OpenAI GPT-4
- Built with Rust + Bevy
- Enhanced with bevy_ecs_tilemap, bevy-yoleck, mapgen.rs
- Deployed via GitHub Actions

---

*"I lost a tooth and found a monster. Kept the monster."* - Wandering Merchant
