# 🎮 Echoes of Beastlight - Metaprompt Game Generator

A self-bootstrapping GitHub Actions workflow system that generates an entire JRPG game using AI-powered metaprompts.

## 🚀 Overview

This project demonstrates a closed-loop metaprompt system where:
1. A configuration file (`game-config.yaml`) defines the game parameters
2. GitHub Actions workflows read the config and generate everything automatically
3. The system can even improve itself by suggesting config optimizations

## 🎯 Key Features

- **Config-Driven**: Everything stems from `game-config.yaml`
- **Self-Bootstrapping**: Workflows generate other workflows
- **Cross-Platform**: Targets Windows, Linux, and macOS
- **Procedural Generation**: Maps, monsters, and quests are procedurally generated
- **Final Fantasy × Pokémon**: Unique blend of JRPG mechanics

## 📁 Project Structure

```
.
├── game-config.yaml              # Master configuration file
├── .github/workflows/
│   ├── generate-file.yml         # Base workflow for OpenAI generation
│   ├── bootstrap-beastlight.yml  # Main bootstrapping workflow
│   └── metaprompt-executor.yml   # Direct metaprompt execution
├── src/                          # Generated C++ source code
├── data/                         # Generated game data (monsters, quests)
├── assets/                       # Generated asset specifications
└── docs/                         # Generated documentation
```

## 🔧 Setup

1. **Fork this repository**

2. **Add OpenAI API Key**:
   - Go to Settings → Secrets → Actions
   - Add a new secret named `OPENAI_API_KEY`
   - Paste your OpenAI API key

3. **Trigger Generation**:
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
- Generates C++ code
- Produces game data
- Sets up build system

### 3. `metaprompt-executor.yml`
Simpler workflow that executes the vintage game metaprompt directly.

## 📝 Configuration Schema

```yaml
game:
  title: "Echoes of Beastlight"
  codename: "beastlight"
  
theme:
  setting: "Arcane wilderness survival meets ancient techno-ruins"
  
gameplay:
  core_mechanics:
    - "Monster slaying OR taming choice"
    - "Procedural map generation"
    
monsters:
  types:
    - name: "Shadowbiter"
      abilities: ["bite", "shadow_dash"]
```

## 🛠️ Development

To modify the game:

1. Edit `game-config.yaml`
2. Push changes
3. Watch as workflows regenerate everything
4. The system will create:
   - C++ source files
   - Monster databases
   - Quest templates
   - Build configurations
   - Documentation

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
- Test generated code
- Propose balance changes
- Create optimized config versions

## 🏗️ Building the Game

Once generated, build with:

```bash
mkdir build && cd build
cmake ..
make
./beastlight
```

## 📜 License

This project is open source. The metaprompt system and workflows are free to use and modify.

## 🙏 Credits

- Inspired by classic JRPGs (Final Fantasy, Pokémon)
- Powered by OpenAI GPT-4
- Built with GitHub Actions

---

*"I lost a tooth and found a monster. Kept the monster."* - Wandering Merchant