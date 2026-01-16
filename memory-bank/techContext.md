# Technical Context

## Migration Status

**FROM**: Rust + Bevy 0.17 (ECS game engine)
**TO**: TypeScript + React Native + Babylon.js + Expo

### Why the Migration?
- Rust/Bevy is wrong for mobile-first game
- React Native provides better mobile tooling
- Babylon React Native bridges 3D to native
- TypeScript offers better developer experience
- Expo simplifies build/deploy pipeline

## Current Tech Stack (1.0 Target)

### Runtime
| Component | Technology | Version |
|-----------|------------|---------|
| Framework | React Native | 0.73.6 |
| Platform | Expo SDK | 50 |
| 3D Engine | Babylon.js | 6.49 |
| Bridge | Babylon React Native | 1.9.0 |

### Core Libraries
| Purpose | Library | Version |
|---------|---------|---------|
| State Management | Zustand | 4.5.5 |
| Validation | Zod | 3.24.0 |
| Navigation | React Navigation | 6.x |
| Audio | expo-av | 13.10.6 |
| Fonts | expo-font | 11.10.3 |

### Build Tools
| Tool | Purpose |
|------|---------|
| pnpm | Package manager (monorepo) |
| TypeScript | Type safety (5.3.3) |
| Biome | Linting and formatting |
| EAS | Expo build service |

## Project Structure

```
echoes-of-beastlight/
├── apps/
│   └── mobile/                 # Expo + React Native app
│       ├── App.tsx             # Root navigation
│       ├── screens/            # UI screens
│       │   ├── MainMenuScreen.tsx
│       │   ├── GameScreen.tsx
│       │   ├── LoadGameScreen.tsx
│       │   └── SettingsScreen.tsx
│       ├── components/         # Reusable UI
│       │   └── GameHUD.tsx
│       ├── game/               # BabylonJS integration
│       │   ├── BabylonView.tsx
│       │   ├── scenes/         # 3D scene renderers
│       │   │   ├── WorldScene.ts
│       │   │   └── CombatScene.ts
│       │   └── systems/        # Game logic
│       │       └── CombatSystem.ts
│       └── stores/             # Zustand state
│           └── gameStore.ts
├── packages/
│   └── game-core/              # Shared logic (framework-agnostic)
│       ├── schemas/            # Zod validation schemas
│       │   ├── combat.ts
│       │   ├── monster.ts
│       │   ├── quest.ts
│       │   ├── world.ts
│       │   └── player.ts
│       └── generation/         # Procedural generators
│           ├── monster-generator.ts
│           └── quest-generator.ts
├── memory-bank/                # Agent collaboration context
├── docs/                       # Documentation
├── game/                       # Legacy Rust code (reference only)
└── MIGRATION_ROADMAP.md        # Migration plan
```

## Key Architecture Decisions

### 1. Monorepo with pnpm Workspaces
- `apps/*` for platform-specific code
- `packages/*` for shared logic
- Enables code sharing between platforms

### 2. Framework-Agnostic Game Core
- All game logic in `@echoes-of-beastlight/game-core`
- No React/RN dependencies in core
- Zod for runtime validation
- Enables future web/server use

### 3. BabylonJS for 3D Rendering
- Isometric camera perspective
- Hex/tile-based world rendering
- Turn-based combat animations
- Cross-platform via React Native bridge

### 4. Zustand for State
- Minimal boilerplate
- TypeScript-first
- Easy persistence with AsyncStorage
- Works well with React Navigation

## Development Commands

```bash
# Install dependencies
pnpm install

# Build game-core
pnpm --filter @echoes-of-beastlight/game-core build

# Start mobile dev server
pnpm --filter @echoes-of-beastlight/mobile start

# Run Android
pnpm --filter @echoes-of-beastlight/mobile android

# Build Android APK (EAS)
pnpm --filter @echoes-of-beastlight/mobile exec eas build --platform android --profile preview
```

## CI/CD Pipeline

```
Push to release/1.0 or main
         │
         ▼
┌─────────────────────┐
│  Install deps       │
│  Typecheck          │
│  Build game-core    │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  EAS Build Android  │
│  (APK preview)      │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Upload artifact    │
│  to GitHub Release  │
└─────────────────────┘
```

## Performance Targets

| Metric | Target |
|--------|--------|
| App startup | < 3 seconds |
| Scene load | < 1 second |
| Combat turn | 60 FPS animations |
| Memory | < 512 MB |
| APK size | < 100 MB |

## Security Considerations

- No cloud storage of user data
- Local-only save files
- API keys in environment variables only
- Input validation via Zod schemas

## MASSIVE Development Benefit: Web Testing with Chrome MCP

**React + BabylonJS + React Native = Test in Chrome Browser**

Because BabylonJS is fundamentally a web-first 3D engine, and React Native shares code with React web:

1. **Test 3D rendering directly in Chrome browser**
2. **Use Chrome MCP for live debugging and inspection**
3. **Iterate on visuals WITHOUT mobile build cycles**
4. **Same game-core code runs on ALL platforms**

This dramatically speeds up development:

```
┌─────────────────┐
│   game-core     │──────────────────────┐
│   (TypeScript)  │                      │
└─────────────────┘                      │
         │                               │
    ┌────┴────┐                          │
    ▼         ▼                          ▼
┌───────┐  ┌───────────┐         ┌─────────────┐
│  Web  │  │  Mobile   │         │  BabylonJS  │
│(Chrome│  │(RN+Expo)  │         │  Playground │
│ + MCP)│  │           │         │  (inspect)  │
└───────┘  └───────────┘         └─────────────┘
   ▲
   │
   └── FAST iteration! No build wait!
```

### Web Test App Structure

```
apps/
├── mobile/          # React Native (production)
└── web/             # React (development/testing)
    └── BabylonJS scenes render identically
```

This is a massive advantage over native-only game engines.
