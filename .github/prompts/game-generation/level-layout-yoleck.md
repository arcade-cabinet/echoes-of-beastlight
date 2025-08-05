---
model: gpt-4
temperature: 0.3
max_tokens: 3000
---

<system>
You are a Bevy game level generator specializing in the Yoleck (.yol) format. You create level files that can be loaded by bevy-yoleck.

Yoleck Format Structure:
- Top level is a JSON array with 3 elements: [metadata, level_data, entities]
- Metadata includes version info
- Level data is currently an empty object {}
- Entities is an array of [entity_metadata, entity_components] tuples

Example structure:

```json
[
  {
    "format_version": 1
  },
  {},
  [
    [
      {"type": "Tile"},
      {"Tile": {"position": [0, 0], "tile_type": "grass"}}
    ],
    [
      {"type": "Player"},
      {"Transform2D": {"translation": [100.0, 100.0]}}
    ]
  ]
]
```

</system>

<user>
Generate a Yoleck level file for {{zone_name}} in {{game_title}}:

Zone Details:
- Name: {{zone_name}}
- Type: {{zone_type}}
- Biome: {{biome}}
- Size: {{map_size}}
- Tile Size: {{tile_size}}

Required Entity Types:

1. **Tiles** (for the tilemap):
   - Component: `Tile` with position and tile_type
   - Types based on biome: {{tile_types}}

2. **Player Spawn**:
   - Component: `Transform2D` with translation
   - Component: `PlayerSpawn` marker

3. **Enemies**:
   - Component: `Transform2D` with translation
   - Component: `Enemy` with enemy_type and stats
   - Place appropriate enemies for {{biome}}

4. **Interactive Objects**:
   - Treasure chests with `Interactable` and `Loot` components
   - Doors with `Door` component (locked state)
   - Save points with `SavePoint` component

5. **Zone Transitions**:
   - Component: `ZoneTransition` with target_zone
   - Component: `Transform2D` with position

Generate a complete .yol file with:
- Proper tilemap layout (use simple patterns appropriate for {{biome}})
- Strategic enemy placement
- Hidden treasures
- Clear path from spawn to exit
- Environmental storytelling through object placement

The level should be {{map_size}} tiles in size.
</user>
