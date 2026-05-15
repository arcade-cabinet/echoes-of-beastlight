# Echoes of Beastlight - 1.0 Migration Roadmap

> **Migration**: Rust/Bevy → TypeScript/React Native + Babylon.js
> **Target**: Mobile-first release with Android debug artifacts via GitHub Releases

## Executive Summary

This document outlines the complete migration of Echoes of Beastlight from its current Rust/Bevy implementation to a TypeScript-based stack using React Native and Babylon.js. This migration enables true mobile-first deployment while preserving the game's core vision.

## Vision Alignment

### Core Game Concept (PRESERVED)
- Monster taming JRPG with procedural generation
- Every seed creates unique worlds and creatures
- Classic JRPG feel: Secret of Mana, Chrono Trigger, FF6 inspired
- Turn-based combat with monster synergy mechanics
- Evolution paths discovered through gameplay

### Visual Style (PRESERVED)
- 32x32 pixel art sprites with black outlines
- Limited, cohesive color palettes per biome
- 3-4 frame animation cycles
- Clean, modern take on retro interfaces

### Audio Direction (PRESERVED)
- Chiptune-inspired with modern production
- Procedural audio via Web Audio API
- Punchy, satisfying feedback sounds

## Technology Stack Migration

| Component | Current (Rust) | Target (TypeScript) |
|-----------|----------------|---------------------|
| Game Engine | Bevy 0.17 | Babylon.js + React Native |
| Mobile Bridge | WASM (limited) | Babylon React Native |
| Build System | Cargo | Expo + EAS |
| Tilemap System | bevy_ecs_tilemap | BabylonJS hex tiles |
| Physics | bevy_rapier2d | Babylon.js NavPlugin V2 |
| UI Framework | bevy_egui | React Native + NativeWind |
| State Management | Bevy ECS | Zustand |
| Data Validation | serde | Zod |

## Migration Phases

### Phase 1: Foundation (Week 1-2)
- [ ] Set up Expo + React Native monorepo structure
- [ ] Configure Babylon React Native integration
- [ ] Establish TypeScript project with strict mode
- [ ] Port data schemas (monsters, quests, items) to Zod
- [ ] Set up Zustand stores for game state

### Phase 2: Rendering Core (Week 3-4)
- [ ] Implement isometric camera system (from neo-tokyo guide)
- [ ] Create hex tile floor system with GLB instancing
- [ ] Build parallax background panel system
- [ ] Implement tile trimming and edge cuts
- [ ] Port visual style guide to Babylon.js materials

### Phase 3: Game Systems (Week 5-7)
- [ ] Port combat system (HP, attack, defense, crit, status)
- [ ] Implement monster data structures and generation
- [ ] Build taming mechanics
- [ ] Create evolution system
- [ ] Port procedural world generation

### Phase 4: AI & Navigation (Week 8-9)
- [ ] Implement Babylon.js NavPlugin V2 for pathfinding
- [ ] Create enemy spawn system with random placement
- [ ] Build anime-style combat effects (DBZ clash system)
- [ ] Implement AI pursuit and steering behaviors

### Phase 5: Polish & Audio (Week 10-11)
- [ ] Port procedural audio specs to Web Audio API
- [ ] Implement UI with React Native components
- [ ] Add particle effects and visual polish
- [ ] Build save/load system with AsyncStorage

### Phase 6: Mobile Release (Week 12+)
- [ ] Configure EAS build for Android
- [ ] Set up GitHub Actions for automated builds
- [ ] Create GitHub Releases workflow for APK distribution
- [ ] iOS TestFlight setup (future)

## File Structure (Target)

```
echoes-of-beastlight/
├── apps/
│   ├── mobile/                 # Expo + React Native app
│   │   ├── app/               # Expo Router pages
│   │   ├── components/        # React Native components
│   │   ├── game/              # Babylon.js game code
│   │   │   ├── scenes/        # Game scenes
│   │   │   ├── systems/       # Game systems (combat, taming)
│   │   │   ├── entities/      # Entity definitions
│   │   │   └── generation/    # Procedural generation
│   │   ├── stores/            # Zustand state stores
│   │   └── assets/            # Game assets
│   └── web/                   # Vite web app (optional)
├── packages/
│   ├── game-core/             # Shared game logic
│   │   ├── schemas/           # Zod schemas
│   │   ├── generation/        # World/monster generation
│   │   └── utils/             # Shared utilities
│   └── audio/                 # Procedural audio system
├── .github/
│   ├── workflows/             # CI/CD workflows
│   └── prompts/               # AI generation prompts (preserved)
└── docs/                      # Documentation
```

## What Gets Preserved

### From Current Codebase
1. **AI Generation Prompts** (`/.github/prompts/`)
   - monster-generator.md
   - quest-generator.md
   - level-visuals.md
   - audio-generator.md
   - style-guide-cascade.md

2. **Game Design Documents** (`/docs/`)
   - project-overview.md
   - ARCHITECTURE.md (updated for new stack)

3. **Visual Assets** (after generation)
   - Sprite sheets
   - Tilesets
   - UI elements

### Core Game Logic (Ported to TypeScript)
1. **Combat System**
   - Character stats (HP, attack, defense, crit_chance)
   - Status effects (Normal, Poisoned, Stunned)
   - Damage calculation with crit multiplier
   - Status effect damage modifiers

2. **Monster System**
   - Monster types and base stats
   - Abilities (Physical/Special/Status)
   - Taming mechanics (difficulty, bait, requirements)
   - Loot tables (common/rare drops)

3. **World Generation**
   - Seed-based procedural generation
   - Biome configuration
   - Tilemap layouts

## Dependencies (Target)

### Mobile App
```json
{
  "dependencies": {
    "@babylonjs/core": "^7.0.0",
    "@babylonjs/react-native": "^2.0.0",
    "@babylonjs/loaders": "^7.0.0",
    "@babylonjs/addons": "^7.0.0",
    "expo": "~54.0.0",
    "expo-router": "~6.0.0",
    Target versions (not current):
"react": "19.1.0",
    "react-native": "0.81.5",
    "zustand": "^5.0.0",
    "zod": "^4.0.0",
    "nativewind": "^4.0.0"
  }
}
```

## Success Criteria for 1.0

### Minimum Viable Game
- [ ] Player can start new game with seed
- [ ] Procedural world generates from seed
- [ ] Player can explore hex-based world
- [ ] Monsters spawn and can be encountered
- [ ] Turn-based combat functions
- [ ] Basic taming mechanics work
- [ ] Game state persists

### Mobile Requirements
- [ ] Runs on Android 10+
- [ ] 60 FPS performance target
- [ ] Touch controls work smoothly
- [ ] Portrait and landscape modes
- [ ] APK under 100MB

### Distribution
- [ ] Automated Android builds via GitHub Actions
- [ ] Debug APKs available via GitHub Releases
- [ ] Version tagged releases

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Babylon React Native performance | Profile early, use instancing heavily |
| Complex shader porting | Use standard materials first, enhance later |
| Asset generation pipeline | Keep prompts, use Meshy AI for 3D |
| Save data migration | Fresh start for 1.0, no legacy data |

## References

- [Babylon React Native](https://www.babylonjs.com/reactnative/)
- [neo-tokyo BabylonJS isometric guide](../neo-tokyo-rival-academies/)
- [wheres-ball-though Expo setup](../wheres-ball-though/)

---

*Last updated: 2026-01-16*
*Migration Lead: Claude Opus 4.5*
