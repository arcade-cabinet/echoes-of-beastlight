# System Patterns

## Game Systems Architecture

### Combat System

The turn-based combat system handles battles between player party and enemy party.

```typescript
// Core combat state
interface CombatState {
  playerParty: CharacterStats[];
  enemyParty: CharacterStats[];
  currentTurn: number;
  isPlayerTurn: boolean;
  battleLog: string[];
}

// Character stats (ported from Rust)
interface CharacterStats {
  hp: number;
  maxHp: number;
  attack: number;
  defense: number;
  critChance: number;  // 0-1
  status: 'Normal' | 'Poisoned' | 'Stunned';
}
```

**Damage Formula**:
```
baseDamage = max(1, attacker.attack - target.defense / 2)
finalDamage = isCritical ? baseDamage * 1.5 : baseDamage
```

**Status Effects**:
- **Poisoned**: 5% maxHP damage per turn
- **Stunned**: Skip turn, clears after one turn

### Monster System

Procedural monster generation with species templates and instances.

```typescript
// Species template (shared data)
interface MonsterSpecies {
  id: string;
  name: string;
  element: ElementType;  // Fire, Water, Earth, Air, Light, Shadow, Nature, Electric
  rarity: Rarity;        // Common, Uncommon, Rare, Epic, Legendary
  baseStats: { hp, attack, defense, critChance };
  abilities: string[];   // Ability IDs
  biomes: string[];      // Where this species spawns
}

// Individual instance (player's monster)
interface MonsterInstance {
  id: string;            // UUID
  speciesId: string;
  nickname?: string;
  level: number;
  experience: number;
  stats: CharacterStats; // Scaled from base
  learnedAbilities: string[];
  isTamed: boolean;
}
```

**Level Scaling**:
```
levelMultiplier = 1 + (level - 1) * 0.1
scaledStat = floor(baseStat * levelMultiplier)
```

**Biome-Element Mapping**:
| Biome | Primary Elements |
|-------|------------------|
| Forest | Nature, Earth, Air |
| Desert | Fire, Earth, Light |
| Tundra | Water, Air, Light |
| Swamp | Water, Nature, Shadow |
| Mountains | Earth, Air, Electric |
| Volcanic | Fire, Earth, Shadow |
| Ocean | Water, Electric, Shadow |
| Cave | Earth, Shadow, Fire |
| Ruins | Shadow, Light, Electric |

### Quest System

Procedural quest generation from templates.

```typescript
interface Quest {
  id: string;
  title: string;
  description: string;
  status: 'NotStarted' | 'InProgress' | 'Completed' | 'Failed';
  objectives: QuestObjective[];
  rewards: QuestReward;
  isMainStory: boolean;
}

interface QuestObjective {
  type: 'DefeatMonster' | 'TameMonster' | 'CollectItem' |
        'TalkToNPC' | 'ExploreArea' | 'DeliverItem';
  targetId?: string;
  targetCount: number;
  currentCount: number;
  isComplete: boolean;
  isOptional: boolean;
}
```

**Quest Templates**:
- Hunt quests: Defeat X monsters
- Taming quests: Capture specific species
- Gather quests: Collect items
- Explore quests: Discover areas
- Delivery quests: Talk to NPCs

### World/Tilemap System

Procedural world generation with biome-specific content.

```typescript
interface Area {
  id: string;
  name: string;
  biome: BiomeType;
  width: number;
  height: number;
  tiles: Tile[];
  monsterSpawnTable: SpawnEntry[];
  connections: AreaConnection[];
  npcs: string[];
  isUnlocked: boolean;
}

interface Tile {
  x: number;
  y: number;
  type: 'Ground' | 'Water' | 'Wall' | 'Bridge' | 'Door' | 'Stairs' | 'Chest' | 'NPC' | 'Spawn' | 'Exit';
  biome: BiomeType;
  isWalkable: boolean;
  isInteractable: boolean;
  spriteKey: string;
}
```

## State Management Patterns

### Zustand Store Structure

```typescript
interface GameState {
  // Current player data
  player: PlayerSave | null;
  isLoading: boolean;

  // Persistent settings
  settings: PlayerSettings;

  // Save slots
  savedGames: SaveSlotMeta[];

  // Actions
  initializeGame: (name: string) => void;
  loadGame: (saveId: string) => void;
  saveGame: () => void;
  updateSettings: (partial: Partial<PlayerSettings>) => void;
  updatePlayer: (partial: Partial<PlayerSave>) => void;
}
```

### Navigation Flow

```
MainMenu
    │
    ├── New Game ──▶ GameScreen (new save)
    │
    ├── Load Game ──▶ LoadGameScreen ──▶ GameScreen (existing save)
    │
    └── Settings ──▶ SettingsScreen
```

## Rendering Patterns

### BabylonJS Scene Setup

```typescript
// Isometric camera configuration
const camera = new ArcRotateCamera(
  'camera',
  -Math.PI / 4,      // Alpha: horizontal rotation
  Math.PI / 3,       // Beta: vertical angle (isometric)
  20,                // Radius: distance
  Vector3.Zero(),
  scene
);

// Camera limits for fixed perspective
camera.lowerBetaLimit = Math.PI / 4;
camera.upperBetaLimit = Math.PI / 3;
camera.lowerRadiusLimit = 15;
camera.upperRadiusLimit = 30;
```

### Scene Separation

| Scene | Purpose | Camera |
|-------|---------|--------|
| WorldScene | Overworld exploration | Isometric, follows player |
| CombatScene | Turn-based battles | Side view, fixed |

## Validation Patterns

All data structures use Zod schemas for runtime validation:

```typescript
// Define schema
export const CharacterStatsSchema = z.object({
  hp: z.number().int().min(0),
  maxHp: z.number().int().min(1),
  attack: z.number().int().min(0),
  defense: z.number().int().min(0),
  critChance: z.number().min(0).max(1),
  status: StatusSchema,
});

// Infer TypeScript type
export type CharacterStats = z.infer<typeof CharacterStatsSchema>;

// Validate at boundaries
const validated = CharacterStatsSchema.parse(untrustedData);
```

## Error Handling Patterns

```typescript
// Result type for game operations
type GameResult<T> =
  | { success: true; data: T }
  | { success: false; error: string };

// Combat actions return results
function executeAttack(attacker: CharacterStats, target: CharacterStats): GameResult<CombatResult> {
  if (attacker.hp <= 0) {
    return { success: false, error: 'Attacker is defeated' };
  }
  // ... perform attack
  return { success: true, data: result };
}
```
