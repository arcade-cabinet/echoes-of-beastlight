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
- Deployed via GitHub Actions

---

*"I lost a tooth and found a monster. Kept the monster."* - Wandering Merchant
