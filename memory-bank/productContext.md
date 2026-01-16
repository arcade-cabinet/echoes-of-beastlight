# Product Context

## Why This Project Exists

### The Problem with Traditional JRPGs
- Once played, the mystery is gone
- Guides and wikis remove all surprise
- Limited replay value after completion
- Static content gets dated

### Our Solution
- AI-driven procedural generation creates endless variety
- Every player's journey is truly unique
- Share seeds to experience friends' discoveries
- Consistent quality through style guidelines

## User Experience Goals

### First-Time Player
1. Choose seed (random or specific)
2. Name hero
3. Begin exploring unique world
4. Discover and tame monsters
5. Build team and progress through story

### Returning Player
- Try new seeds for fresh experiences
- Share exceptional seeds with community
- Challenge with harder difficulty options
- Compete in speedrun categories

### Streamer/Content Creator
- Every playthrough is unique content
- Audience can suggest seeds
- Built-in photo mode for capturing moments
- Speedrun-friendly with consistent seed behavior

## Game Loop

```
┌──────────────┐
│   Explore    │◄────────────────┐
│   World      │                 │
└──────┬───────┘                 │
       │                         │
       ▼                         │
┌──────────────┐                 │
│   Encounter  │                 │
│   Monster    │                 │
└──────┬───────┘                 │
       │                         │
       ▼                         │
┌──────────────┐    ┌───────────────┐
│   Battle     │───▶│   Victory     │
│   (Combat)   │    │   Rewards     │
└──────┬───────┘    └───────┬───────┘
       │                    │
       ▼                    │
┌──────────────┐            │
│   Tame or    │            │
│   Defeat     │────────────┘
└──────────────┘
```

## Progression System

### Player Character
- Level 1-100
- Stats: HP, Attack, Defense, Crit Chance
- Experience from battles and quests
- Equipment and items

### Monster Team
- Active party: up to 6 monsters
- Storage for additional monsters
- Individual monster levels
- Learned abilities (up to 4 active)
- Evolution based on level and conditions

### World Progression
- Areas unlock through story/quests
- Biomes have level ranges
- Boss monsters gate progression
- Discovery percentage tracked

## Monetization Strategy (Future)

- Premium price point (one-time purchase)
- No pay-to-win mechanics
- Optional cosmetic DLC
- No ads

## Competitive Advantages

1. **Infinite Replayability**: Procedural generation ensures endless content
2. **Mobile-First**: Native mobile experience, not a port
3. **Nostalgia Factor**: Appeals to JRPG fans aged 25-45
4. **Community**: Seed sharing creates organic virality
5. **Modern Tech**: BabylonJS 3D engine on React Native
